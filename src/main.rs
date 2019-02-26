extern crate cgmath;
extern crate gl;
extern crate glutin;

mod mesh;
mod shaders;
mod utils;

use glutin::GlContext;
use std::path::Path;
use std::rc::Rc;

use shaders::{Program, Shader};

use cgmath::prelude::*;
use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

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

    let vertex_shader = Shader::load_shader(Path::new("data/shaders/basic/projection.vs"));
    println!("{:?}", vertex_shader);

    let pixel_shader = Shader::load_shader(Path::new("data/shaders/basic/phong/phong.fs"));
    println!("{:?}", pixel_shader);

    let program = Program::load_program(&vec![
        Rc::new(pixel_shader.unwrap()),
        Rc::new(vertex_shader.unwrap()),
    ])
    .unwrap();
    println!("{:?}", program);

    let mut cube = mesh::Mesh::cube();
    cube.ready_up();
    println!("{:?}", cube);
    cube.draw();

    let projection = perspective(Deg(75.0), 1024.0 / 768.0, 0.1, 10.0);

    let cam_pos = Point3::new(3.0, 2.0, 3.0);
    let cam_target = Point3::new(0.0, 0.0, 0.0);
    let cam_forward = cam_target - cam_pos;
    let cam_right = cam_forward.cross(Vector3::new(0.0, 1.0, 0.0)).normalize();
    let cam_up = cam_right.cross(cam_forward).normalize();
    let view = Matrix4::look_at(cam_pos, cam_target, cam_up);

    let mut model = Matrix4::<f32>::identity();

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

        program.bind();
        program.set_mat4("projection", &projection);
        program.set_mat4("view", &view);
        program.set_mat4("model", &model);
        program.set_vec4("eye_pos", &cam_pos.to_homogeneous());
        program.set_vec4("light_pos", &Point3::new(3.0, 1.0, 1.0).to_homogeneous());

        cube.draw();

        gl_window.swap_buffers().unwrap();
    }
}
