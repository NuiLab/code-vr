use winit::{WindowBuilder, get_available_monitors, get_primary_monitor, Event, ElementState};
use vulkano_win::{Window, VkSurfaceBuild, required_extensions};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Queue, Device, DeviceExtensions};
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode};
use vulkano::image::SwapchainImage;
use vulkano::command_buffer::Submission;

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
    config: Config
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
        let (swapchain, images) =
            create_swapchain(&window, &physical, &device, &queue, None, &config);

        Engine {
            window: window,
            instance,
            physical_device,
            device,
            swapchain,
            images,
            queue,
            config,
        }
    }

    /// Handles input/output events from the window and any input middleware.
    pub fn io(&mut self) -> bool {

        for ev in self.window.window().poll_events() {
            // @TODO
            // For each axis: config::input::Axis in config.input
            // For each axis_value: config::input::AxisValue in event
            // map axis_value.key to winit::Event as mapped_event

            // Core Events
            match ev {
                Event::Resized(w, h) => {
                    self.config.window.resolution = [w, h];
                    let (swapchain, images) =
                        create_swapchain(&self.window,
                                         &PhysicalDevice::from_index(&self.instance,
                                                                     self.physical_device)
                                                  .unwrap(),
                                         &self.device,
                                         &self.queue,
                                         Some(&self.swapchain),
                                         &self.config);
                    self.swapchain = swapchain;
                    self.images = images;
                }
                Event::Closed => return false,
                _ => (),
            };
        }
        
        true
    }

    /// Recursively updates application tree.
    pub fn update(&self) {

        //@TODO recursively map each update function from each node.
        //node.children.map(update)

    }

    /// Updates the display surface with a new image.
    pub fn render(&self) {

        let image_num = self.swapchain
            .acquire_next_image(Duration::new(1, 0))
            .unwrap();

        // @TODO build command buffers with threads and submit the changes in main thread (here)
        // submissions.push(command_buffer::submit(&command_buffer, &queue).unwrap());

        self.swapchain.present(&self.queue, image_num).unwrap();
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


        let dimensions = if config.window.resolution[0] <= 240 ||
                            config.window.resolution[1] <= 240 {
            caps.current_extent.unwrap_or([800, 600])
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

