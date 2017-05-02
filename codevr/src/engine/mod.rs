use winit::{WindowBuilder, get_available_monitors, get_primary_monitor};
use vulkano_win::{Window, VkSurfaceBuild, required_extensions};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Queue, Device, DeviceExtensions};
use vulkano::swapchain::{Swapchain, SurfaceTransform};

use config::Config;
use config::WindowConfig;

use std::sync::Arc;

pub struct Engine {
    window: Window,
    instance: Arc<Instance>,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    queue: Arc<Queue>

}

impl Engine {
    pub fn new(config: Config) -> Engine {

        let window_builder = create_window(config.window);

        // @TODO - Pass window to renderer specific initializer...

        let instance = {
            let extensions = required_extensions();
            Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance.")
        };

        let physical_device = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("No vulkan device is available.");

        let window = window_builder.build_vk_surface(&instance).unwrap();

        let queue = physical_device
            .queue_families()
            .find(|q| q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false))
            .expect("Couldn't find a graphical queue family.");

        // Logical Device, Queues
        let (device, mut queues) = {
            let device_ext = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            };

            Device::new(&physical_device,
                        physical_device.supported_features(),
                        &device_ext,
                        [(queue, 0.5)].iter().cloned())
                    .expect("failed to create device")
        };

        // Device Queue
        let queue = queues.next().unwrap();

        // Swapchain, Swapchain Images
        let (swapchain, images) = {

            let caps = window
                .surface()
                .get_capabilities(&physical_device)
                .expect("failed to get surface capabilities");

            let dimensions = caps.current_extent.unwrap_or([1280, 720]);

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
                           &queue,
                           SurfaceTransform::Identity,
                           alpha,
                           present,
                           true,
                           None)
                    .expect("failed to create swapchain")
        };

        Engine {
            window: window,
            instance: instance.clone(),
            device,
            swapchain,
            queue
        }
    }

    pub fn io(&self) {}

    pub fn update(&self) -> bool {
        false
    }

    pub fn render(&self) {}
}

/// Creates a window builder with a given window configuration.
pub fn create_window(config: WindowConfig) -> WindowBuilder {

    let window_manager = WindowBuilder::new()
        .with_title("CodeVR")
        .with_dimensions(config.resolution[0], config.resolution[1]);

    let mut w = get_primary_monitor();

    if config.fullscreen {
        for (i, monitor) in get_available_monitors().enumerate() {
            if i == config.display as usize {
                w = monitor;
            }
        }
    }

    window_manager.with_fullscreen(w)
}

