mod gfx;

use winit::{WindowBuilder, EventsLoop};
use vulkano_win::{Window, VkSurfaceBuild, required_extensions};
use vulkano::command_buffer::CommandBufferBuilder;
use vulkano::device::{Queue, Device, DeviceExtensions};
use vulkano::format;
use vulkano::framebuffer::{RenderPass, RenderPassAbstract, Framebuffer, FramebufferAbstract};
use vulkano::image::attachment::{AttachmentImageAccess, AttachmentImage};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::memory::pool::StdMemoryPoolAlloc;
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode};
use vulkano::sync::GpuFuture;

use std::clone::Clone;
use std::sync::Arc;
use std::time::Duration;

use config::Config;
pub use self::gfx::GraphicsState;
use core::MINIMUM_RESOLUTION;

type FinalFramebuffer = Framebuffer<Arc<RenderPassAbstract + Send + Sync>, (((), Arc<SwapchainImage>), AttachmentImageAccess<format::D16Unorm, StdMemoryPoolAlloc>)>;

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
    queue: Arc<Queue>

}

impl Renderer {

    pub fn new(window_builder: WindowBuilder, config: Arc<Config>) -> (Renderer, Arc<Window>, Arc<EventsLoop>) {
        
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
        let window = Arc::new(window_builder.build_vk_surface(&events_loop, instance.clone()).unwrap());

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
            create_swapchain(&window, &physical, &device, &queue, None, &config);

        let depth_buffer = AttachmentImage::transient(device.clone(), images[0].dimensions(), format::D16Unorm).unwrap();

        // Render Pass
        let render_pass: Arc<RenderPassAbstract + Send + Sync> = Arc::new(
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
        ).unwrap()
    );


    let framebuffers = images.iter().map(|image| {
        Arc::new(Framebuffer::start(render_pass.clone())
            .add(image.clone()).unwrap()
            .add(depth_buffer.clone()).unwrap()
            .build().unwrap())
    }).collect::<Vec<_>>();

        (
            Renderer {
                config,
                window: window.clone(),
                instance,
                physical_device,
                device,
                swapchain,
                images,
                depth_buffer,
                framebuffers,
                render_pass,
                queue
            },
            window,
            Arc::new(events_loop)
        )
    }

    pub fn resize(&mut self) {
                    let (swapchain, images) =
                        create_swapchain(&self.window, 
                                         &PhysicalDevice::from_index(&self.instance, self.physical_device).unwrap(),
                                         &self.device,
                                         &self.queue,
                                         Some(&self.swapchain),
                                         &self.config);
                    self.swapchain = swapchain;
                    self.depth_buffer = AttachmentImage::transient(self.device.clone(), images[0].dimensions(), format::D16Unorm).unwrap();
					self.images = images;
                    self.framebuffers = self.images.iter().map(|image| {
                        Arc::new(Framebuffer::start(self.render_pass.clone())
                            .add(image.clone()).unwrap()
                            .add(self.depth_buffer.clone()).unwrap()
                            .build().unwrap())
                    }).collect::<Vec<_>>();
    }

    pub fn render(&mut self, gfx: &GraphicsState) {
        /*
        // @TODO - For each node in gfxstate
        let command_buffers = self.framebuffers
            .iter()
            .map(|framebuffer| {
                let cmd = PrimaryCommandBufferBuilder::new(&self.device, self.queue.family())
                    .draw_inline(&self.render_pass,
                                 &framebuffer,
                                 render_pass::ClearValues {
                                     color: [0.2, 0.4, 0.8, 1.0],
                                     depth: 1.0,
                                 }) 
                    .draw_end()
                    .build();
                    // renderstate.render(cmd)
                    cmd
            })
            .collect::<Vec<_>>();
        let image_num = self.swapchain
            .acquire_next_image(Duration::new(1, 0))
            .unwrap();

        // @TODO build command buffers with threads and submit the changes in main thread (here)
        self.submissions
            .push(submit(&command_buffers[image_num], &self.queue).unwrap());

        self.swapchain.present(&self.queue, image_num).unwrap();
        */
    }
}


/// Sets up and creates a swapchain
fn create_swapchain(window: &Window,
                    physical_device: &PhysicalDevice,
                    device: &Arc<Device>,
                    queue: &Arc<Queue>,
                    old: Option<&Arc<Swapchain>>,
                    config: &Config)
                    -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    {
        let caps = window
            .surface()
            .get_capabilities(&physical_device)
            .expect("failed to get surface capabilities");

            


        let dimensions = if config.window.resolution[0] <= MINIMUM_RESOLUTION[0] ||
                            config.window.resolution[1] <= MINIMUM_RESOLUTION[1] {

            let min = caps.min_image_extent;

            let extent = caps.current_extent.unwrap_or(MINIMUM_RESOLUTION);

            if extent[0] < min[0] || extent[1] < min[1] {
                min
            }
            else {
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