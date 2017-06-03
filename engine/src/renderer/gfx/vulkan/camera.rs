fn lol() {
    let uniform_buffer = {
        CpuAccessibleBuffer::<fs::ty::Block>::from_data(&device,
                                                        &BufferUsage::all(),
                                                        Some(queue.family()),
                                                        ModelViewProjection {
                                                          model,
                                                          view, 
                                                          projection
                                                        })
                .expect("failed to create Uniform Buffer")
    };
}
