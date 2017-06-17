pub struct Mesh {
    primitives: Vec<Primitive>,
}

struct Primitive {
    attributes: u32,
    material: u32,
    indices: u32,
}
