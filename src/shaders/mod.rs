pub mod shader_loader;

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use gl;

#[derive(Debug)]
pub enum ShaderType {
    VERTEX,
    FRAGMENT,
    GEOMETRY,
    COMPUTE,
}

#[derive(Debug)]
pub struct Shader {
    pub addr: u32,
    pub path: String,
    pub uniforms: Vec<String>,
    pub shader_type: ShaderType,
}

#[derive(Debug)]
pub struct Program {
    pub addr: u32,
    pub shader: Vec<Rc<Shader>>,
    pub uniforms_location: HashMap<String, u32>,
}

pub fn get_shader_type(path: &Path) -> Option<ShaderType> {
    let ext = path.extension().and_then(|extension| extension.to_str());
    match ext {
        Some("vs") => Some(ShaderType::VERTEX),
        Some("fs") => Some(ShaderType::FRAGMENT),
        Some("gs") => Some(ShaderType::GEOMETRY),
        Some("cs") => Some(ShaderType::COMPUTE),
        _ => None,
    }
}

pub fn get_gl_shader_type(shader_type: &Option<ShaderType>) -> Option<u32> {
    match shader_type {
        Some(ShaderType::VERTEX) => Some(gl::VERTEX_SHADER),
        Some(ShaderType::FRAGMENT) => Some(gl::FRAGMENT_SHADER),
        Some(ShaderType::GEOMETRY) => Some(gl::GEOMETRY_SHADER),
        Some(ShaderType::COMPUTE) => Some(gl::COMPUTE_SHADER),
        _ => None,
    }
}
