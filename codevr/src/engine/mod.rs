use winit::{WindowBuilder, get_available_monitors, get_primary_monitor, Event, ElementState};
use vulkano_win::{Window, VkSurfaceBuild, required_extensions};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Queue, Device, DeviceExtensions};
use vulkano::swapchain::{Swapchain, SurfaceTransform};
use vulkano::image::SwapchainImage;

use config::Config;
use config::WindowConfig;

use std::sync::Arc;
use std::time::Duration;

pub struct Engine {
    window: Window,
    instance: Arc<Instance>,
    physical_device: usize,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
}

impl Engine {
    pub fn new(config: Config) -> Engine {

        let window_builder = create_window(&config.window);

        let instance = {
            let extensions = required_extensions();
            Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance.")
        };

        let ins = &instance.clone();

        let physical = PhysicalDevice::enumerate(&ins)
            .next()
            .expect("No vulkan device is available.");

        let physical_device = physical.index();

        let window = window_builder.build_vk_surface(&instance).unwrap();

        let queue = physical
            .queue_families()
            .find(|q| q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false))
            .expect("Couldn't find a graphical queue family.");

        // Logical Device, Queues
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
        let (swapchain, images) = create_swapchain(&window,
                                                   &physical,
                                                   &device,
                                                   &queue,
                                                   &config.window.resolution,
                                                   None);

        Engine {
            window: window,
            instance,
            physical_device,
            device,
            swapchain,
            images,
            queue,
        }
    }

    pub fn io(&mut self) -> bool {
        for ev in self.window.window().poll_events() {
            match ev {
                Event::Resized(w, h) => {
                    //self.images.map(| i | i.drop());

                    let (swapchain, images) =
                        create_swapchain(&self.window,
                                         &PhysicalDevice::from_index(&self.instance,
                                                                     self.physical_device)
                                                  .unwrap(),
                                         &self.device,
                                         &self.queue,
                                         &[w, h],
                                         Some(&self.swapchain));
                    self.swapchain = swapchain;
                    self.images = images;
                }
                Event::Closed => return false,
                _ => (),
            };
        }
        true
    }

    pub fn update(&self) {}

    pub fn render(&self) {
        let image_num = self.swapchain
            .acquire_next_image(Duration::new(1, 0))
            .unwrap();

        self.swapchain.present(&self.queue, image_num).unwrap();
    }
}

fn create_swapchain(window: &Window,
                    physical_device: &PhysicalDevice,
                    device: &Arc<Device>,
                    queue: &Arc<Queue>,
                    dimension: &[u32; 2],
                    old: Option<&Arc<Swapchain>>)
                    -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    {
        let caps = window
            .surface()
            .get_capabilities(&physical_device)
            .expect("failed to get surface capabilities");


        let dimensions = if dimension[0] <= 240 || dimension[1] <= 240 {
            caps.current_extent.unwrap_or([1280, 720])
        } else {
            *dimension
        };

        let present = caps.present_modes.iter().next().unwrap();

        let alpha = caps.supported_composite_alpha.iter().next().unwrap();

        let format = caps.supported_formats[0].0;

        Swapchain::new(&device,
                       &window.surface(),
                       caps.min_image_count,
                       format,
                       dimensions,
                       1,
                       &caps.supported_usage_flags,
                       queue,
                       SurfaceTransform::Identity,
                       alpha,
                       present,
                       true,
                       old)
                .expect("failed to create swapchain")
    }
}

/// Creates a window builder with a given window configuration.
fn create_window(config: &WindowConfig) -> WindowBuilder {

    let mut w = get_primary_monitor();
    let dimensions = w.get_dimensions();

    // Set resolution
    let resolution = if config.resolution[0] <= 240 || config.resolution[1] <= 240 {
        [dimensions.0, dimensions.1]
    } else {
        config.resolution.clone()
    };

    // Set fullscreen
    if config.fullscreen {
        for (i, monitor) in get_available_monitors().enumerate() {
            if i == config.display as usize {
                w = monitor;
            }
        }
    }

    // Create manager
    let window_manager = WindowBuilder::new()
        .with_title("CodeVR")
        .with_dimensions(resolution[0], resolution[1]);

    if config.fullscreen {
        return window_manager.with_fullscreen(w);
    }


    window_manager
}

