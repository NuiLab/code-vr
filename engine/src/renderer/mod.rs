pub mod gfx;
pub mod vulkan;

use winit::{WindowBuilder, EventsLoop};
use vulkano_win::{Window, VkSurfaceBuild, required_extensions};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferBuilder};
use vulkano::device::{Queue, Device, DeviceExtensions};
use vulkano::format;
use vulkano::buffer::cpu_access::CpuAccessibleBuffer;
use vulkano::buffer::BufferUsage;
use vulkano::framebuffer::{RenderPassAbstract, Framebuffer};
use vulkano::image::attachment::{AttachmentImageAccess, AttachmentImage};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::memory::pool::StdMemoryPoolAlloc;
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, acquire_next_image};
use vulkano::sync::{now, GpuFuture};


use std::clone::Clone;
use std::sync::Arc;
use std::time::Duration;
use std::mem;

use config::Config;
pub use self::gfx::GraphicsState;
use self::vulkan::VulkanGraphicsState;
use core::MINIMUM_RESOLUTION;

type FinalFramebuffer = Framebuffer<Arc<RenderPassAbstract + Send + Sync>,
                                    (((), Arc<SwapchainImage>),
                                     AttachmentImageAccess<format::D16Unorm, StdMemoryPoolAlloc>)>;

/// Handles the rendering of the graphics state.
pub struct Renderer {
    /// Engine configuration
    config: Arc<Config>,

    /// Current Window
    window: Arc<Window>,

    // Vulkan API
    instance: Arc<Instance>,
    physical_device: usize,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    depth_buffer: Arc<AttachmentImage<format::D16Unorm>>,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FinalFramebuffer>>,
    queue: Arc<Queue>,
    previous_frame: Box<GpuFuture>,

    // Vulkan Graphics State
    api_gfx: VulkanGraphicsState,
}

impl Renderer {

    /// Initializes Vulkan Renderer
    pub fn new(window_builder: WindowBuilder,
               config: Arc<Config>)
               -> (Renderer, Arc<Window>, Arc<EventsLoop>) {

        // Create Vulkan Instance, Physical Device
        let instance = {
            let extensions = required_extensions();
            Instance::new(None, &extensions, &[]).expect("Failed to create Vulkan instance.")
        };
        let ins = instance.clone();

        let physical = PhysicalDevice::enumerate(&ins)
            .next()
            .expect("No vulkan device is available.");

        let physical_device = physical.index();

        // Create Window
        let events_loop = EventsLoop::new();
        let window = Arc::new(window_builder
                                  .build_vk_surface(&events_loop, instance.clone())
                                  .unwrap());

        // Queue ID for Device generation
        let queue = physical
            .queue_families()
            .find(|&q| q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false))
            .expect("Couldn't find a graphical queue family.");

        // Logical Device
        let (device, mut queues) = {
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            };

            Device::new(&physical,
                        physical.supported_features(),
                        &device_ext,
                        [(queue, 0.5)].iter().cloned())
                .expect("failed to create device")
        };

        // Device Queue
        let queue = queues.next().unwrap();

        // Swapchain, Swapchain Images
        let (swapchain, images) =
            create_swapchain(&window, physical, &device, &queue, None, &config);

        let depth_buffer =
            AttachmentImage::transient(device.clone(), images[0].dimensions(), format::D16Unorm)
                .unwrap();

        // Render Pass
        let render_pass: Arc<RenderPassAbstract + Send + Sync> =
            Arc::new(
                single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: format::Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        ).unwrap(),
            );

        let framebuffers = images
            .iter()
            .map(|image| {
                Arc::new(Framebuffer::start(render_pass.clone())
                             .add(image.clone())
                             .unwrap()
                             .add(depth_buffer.clone())
                             .unwrap()
                             .build()
                             .unwrap())
            })
            .collect::<Vec<_>>();

        let api_gfx = VulkanGraphicsState::new();

        (Renderer {
             config,
             window: window.clone(),
             instance,
             physical_device,
             swapchain,
             images,
             depth_buffer,
             framebuffers,
             render_pass,
             queue,
             api_gfx,
             previous_frame: Box::new(now(device.clone())) as Box<GpuFuture>,
             device,
         },
         window,
         Arc::new(events_loop))
    }

    /// Resizes Vulkan data structures
    pub fn resize(&mut self) {
        let (swapchain, images) =
            create_swapchain(&self.window,
                             PhysicalDevice::from_index(&self.instance, self.physical_device)
                                 .unwrap(),
                             &self.device,
                             &self.queue,
                             Some(&self.swapchain),
                             &self.config);
        self.swapchain = swapchain;
        self.depth_buffer = AttachmentImage::transient(self.device.clone(),
                                                       images[0].dimensions(),
                                                       format::D16Unorm)
            .unwrap();
        self.images = images;
        self.framebuffers = self.images
            .iter()
            .map(|image| {
                Arc::new(Framebuffer::start(self.render_pass.clone())
                             .add(image.clone())
                             .unwrap()
                             .add(self.depth_buffer.clone())
                             .unwrap()
                             .build()
                             .unwrap())
            })
            .collect::<Vec<_>>();
    }

    /// Renders the Graphics State data layer, converting to API data along the way.
    pub fn render(&mut self, gfx: &GraphicsState) {

        // Acquire frame future
        self.previous_frame.cleanup_finished();

        let (image_num, acquire_future) =
            acquire_next_image(self.swapchain.clone(), Duration::new(1, 0)).unwrap();

        let command_buffer = AutoCommandBufferBuilder::new(self.device.clone(),
                                                           self.queue.family())
            .unwrap()
            .begin_render_pass(self.framebuffers[image_num].clone(),
                               false,
                               vec![[0.0, 0.0, 1.0, 1.0].into(), 1f32.into()])
            .unwrap()
            .build()
            .unwrap();

        // Graphics Data Layer Traversal
        for (camera_index, camera) in gfx.cameras.iter().enumerate() {

            // Deallocate assets with no references
            if Arc::strong_count(camera) == 1 {

                //gfx.cameras.remove(camera_index);
                //self.api_gfx.cameras.remove(camera_index);
                
            } else if let Ok(cam) = camera.lock() {

                // Create an Vulkan Camera if one doesn't already exist
                if self.api_gfx.cameras.len() < gfx.cameras.len() {
                    self.api_gfx.cameras.push(vulkan::Camera::new(&self.device, &self.queue, cam.view.into(), cam.projection.into()));
                }

                {
                    // if get index and its null, create it
                    // aquiring write lock for the uniform buffer
                    let mut buffer_content = self.api_gfx.cameras[camera_index].ubo.write().unwrap();
                    buffer_content.view = cam.view.into();
                    buffer_content.projection = cam.projection.into();
                }


                for (node_index, node) in gfx.nodes.iter().enumerate() {
                /* 
                   // if its descriptor set 
                   // update model matrix ubo for that node
                   let set = 0;

                   for primitive in node.mesh {
                       let vbo = api_gfx.buffers[pr];
                            .draw_indexed(
                                primitive.pipeline.clone(), DynamicState::none(),
                                primitive.vertex_buffer.clone(), 
                                primitive.index_buffer.clone(), node.set.clone(), ()).unwrap()
                            .end_render_pass().unwrap()
                            .build().unwrap();
                   }
                   */
                }

            }
        }

        // Setup next Future
        let prev = mem::replace(&mut self.previous_frame, Box::new(now(self.device.clone())));

        let future = prev.join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush()
            .unwrap();

        self.previous_frame = Box::new(future) as Box<_>;

    }
}


/// Sets up and creates a swapchain
fn create_swapchain(window: &Window,
                    physical_device: PhysicalDevice,
                    device: &Arc<Device>,
                    queue: &Arc<Queue>,
                    old: Option<&Arc<Swapchain>>,
                    config: &Config)
                    -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    {
        let caps = window
            .surface()
            .capabilities(physical_device)
            .expect("failed to get surface capabilities");


        let dimensions = if config.window.resolution[0] <= MINIMUM_RESOLUTION[0] ||
                            config.window.resolution[1] <= MINIMUM_RESOLUTION[1] {

            let min = caps.min_image_extent;

            let extent = caps.current_extent.unwrap_or(MINIMUM_RESOLUTION);

            if extent[0] < min[0] || extent[1] < min[1] {
                min
            } else {
                extent
            }
        } else {
            config.window.resolution
        };


        let present = if config.graphics.vsync &&
                         caps.present_modes.supports(PresentMode::Mailbox) {
            PresentMode::Mailbox
        } else {
            caps.present_modes.iter().next().unwrap()
        };

        let alpha = caps.supported_composite_alpha.iter().next().unwrap();

        let format = caps.supported_formats[0].0;

        Swapchain::new(device.clone(),
                       window.surface().clone(),
                       caps.min_image_count,
                       format,
                       dimensions,
                       1,
                       caps.supported_usage_flags,
                       queue,
                       SurfaceTransform::Identity,
                       alpha,
                       present,
                       true,
                       old)
            .expect("failed to create swapchain")
    }
}
