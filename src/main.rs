extern crate gl;
extern crate glutin;

mod shaders;
mod utils;

use glutin::GlContext;
use std::path::Path;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    let shader = shaders::shader_loader::load_shader(Path::new("data/shaders/plane/plane.vs"));
    println!("Shader {:?}", shader);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(vkey) = input.virtual_keycode {
                        if vkey == glutin::VirtualKeyCode::Escape
                            && input.state == glutin::ElementState::Pressed
                        {
                            running = false
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        }

        gl_window.swap_buffers().unwrap();
    }
}
