struct Renderer {
    instance: Arc<Instance>,
    physical_device: usize,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    depth_buffer: Arc<AttachmentImage<format::D16Unorm>>,
    render_pass: Arc<render_pass::CustomRenderPass>,
    framebuffers: Vec<Arc<Framebuffer<render_pass::CustomRenderPass>>>,
    submissions: Vec<Arc<Submission>>,
    queue: Arc<Queue>,
    render_threads: Vec<JoinHandle>
}