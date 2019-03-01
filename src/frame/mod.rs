#[derive(Debug)]
struct Framebuffer {
    pub addr: u32,
    pub color_attachment: Option<u32>,
    pub depth_stencil_attachment: Option<u32>,
}
