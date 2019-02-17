use crate::utils;
use gl;
use std::path::Path;

use std::ffi::CString;
use std::ptr;

use super::*;

pub fn parse_uniforms(src: &str) -> Vec<String> {
    let mut uniforms: Vec<String> = Vec::new();
    for line in src.lines() {
        if line.starts_with("uniform ") {
            let attrb: Vec<&str> = line.split(' ').collect();
            if attrb.len() < 3 {
                continue;
            }

            let uniform_name = String::from(attrb[2]).trim_end_matches(';').to_string();
            uniforms.push(uniform_name);
        }
    }

    uniforms
}

pub fn load_shader(path: &Path) -> Option<Shader> {
    #[cfg(feature = "debug")]
    println!("[NFO] Loading shader {}", path.display());

    let src = utils::load_file(path).and_then(|shd_src| CString::new(shd_src.as_bytes()).ok());
    if src.is_none() {
        #[cfg(feature = "debug")]
        eprintln!("[ERR] Couldn't load source for shader {}", path.display());

        return None;
    }
    let src = src.unwrap();

    let shader_type = get_shader_type(path);
    if shader_type.is_none() {
        #[cfg(feature = "debug")]
        eprintln!("[ERR] Couldn't detect shader type for {}", path.display());

        return None;
    }

    let gl_type = get_gl_shader_type(&shader_type);
    unsafe {
        let addr = gl::CreateShader(gl_type.unwrap());
        gl::ShaderSource(addr, 1, &src.as_ptr(), ptr::null());
        gl::CompileShader(addr);

        let mut status: i32 = 0;
        gl::GetShaderiv(addr, gl::COMPILE_STATUS, &mut status);
        if status == i32::from(gl::FALSE) {
            #[cfg(feature = "debug")]
            {
                let mut log_len: i32 = 0;
                gl::GetShaderiv(addr, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log: Vec<u8> = Vec::with_capacity(log_len as usize);
                gl::GetShaderInfoLog(addr, log_len, ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                log.set_len(log_len as usize);
                eprintln!(
                    "[ERR] Couldn't compile shader {}, log:\n{}",
                    path.display(),
                    String::from_utf8_lossy(&log[..])
                );
            }

            return None;
        }

        Some(Shader {
            addr,
            path: String::from(path.to_str().unwrap()),
            uniforms: parse_uniforms(&src.to_string_lossy()),
            shader_type: shader_type.unwrap(),
        })
    }
}

// pub fn load_program
