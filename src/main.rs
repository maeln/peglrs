extern crate cgmath;
extern crate gl;
extern crate glutin;

mod camera;
mod frame;
mod mesh;
mod scene;
mod shaders;
mod utils;

use glutin::GlContext;
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};

use camera::{Camera, Direction};
use shaders::{Program, Shader};

use cgmath::prelude::*;
use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthMask(gl::TRUE);
        gl::DepthFunc(gl::LEQUAL);
        gl::DepthRange(0.0, 1.0);
        gl::Enable(gl::DEPTH_CLAMP);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let vertex_shader = Shader::load_shader(Path::new("data/shaders/basic/projection.vs"));
    let pixel_shader = Shader::load_shader(Path::new("data/shaders/basic/phong/phong.fs"));

    let program = Program::load_program(&vec![
        Rc::new(pixel_shader.unwrap()),
        Rc::new(vertex_shader.unwrap()),
    ])
    .unwrap();

    let mut cube = mesh::Mesh::cube();
    cube.ready_up();

    let mut model = Matrix4::<f32>::identity();
    let projection = perspective(Deg(75.0), 1024.0 / 768.0, 0.1, 10.0);
    let mut cam = Camera::new(
        Point3::new(0.0, 0.0, -2.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let mut time = Instant::now();
    let mut dt: f64 = 0.0;

    let mut mouse_pos: (f32, f32) = (0.0, 0.0);
    let mut mouse_delta: (f32, f32) = (0.0, 0.0);
    let mut mouse_pressed = false;

    let mut dirs: Vec<Direction> = Vec::with_capacity(6);

    let mut running = true;
    while running {
        mouse_delta = (0.0, 0.0);
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(vkey) = input.virtual_keycode {
                        if input.state == glutin::ElementState::Pressed {
                            match vkey {
                                glutin::VirtualKeyCode::Escape => running = false,
                                glutin::VirtualKeyCode::W => {
                                    if !dirs.contains(&Direction::FORWARD) {
                                        dirs.push(Direction::FORWARD);
                                    }
                                }
                                glutin::VirtualKeyCode::S => {
                                    if !dirs.contains(&Direction::BACKWARD) {
                                        dirs.push(Direction::BACKWARD);
                                    }
                                }
                                glutin::VirtualKeyCode::A => {
                                    if !dirs.contains(&Direction::LEFT) {
                                        dirs.push(Direction::LEFT);
                                    }
                                }
                                glutin::VirtualKeyCode::D => {
                                    if !dirs.contains(&Direction::RIGHT) {
                                        dirs.push(Direction::RIGHT);
                                    }
                                }
                                glutin::VirtualKeyCode::Space => {
                                    if !dirs.contains(&Direction::UP) {
                                        dirs.push(Direction::UP);
                                    }
                                }
                                glutin::VirtualKeyCode::C => {
                                    if !dirs.contains(&Direction::DOWN) {
                                        dirs.push(Direction::DOWN);
                                    }
                                }
                                _ => {}
                            }
                        } else if input.state == glutin::ElementState::Released {
                            match vkey {
                                glutin::VirtualKeyCode::W => {
                                    if let Some(n) =
                                        dirs.iter().position(|x| x == &Direction::FORWARD)
                                    {
                                        dirs.remove(n);
                                    }
                                }
                                glutin::VirtualKeyCode::S => {
                                    if let Some(n) =
                                        dirs.iter().position(|x| x == &Direction::BACKWARD)
                                    {
                                        dirs.remove(n);
                                    }
                                }
                                glutin::VirtualKeyCode::A => {
                                    if let Some(n) = dirs.iter().position(|x| x == &Direction::LEFT)
                                    {
                                        dirs.remove(n);
                                    }
                                }
                                glutin::VirtualKeyCode::D => {
                                    if let Some(n) =
                                        dirs.iter().position(|x| x == &Direction::RIGHT)
                                    {
                                        dirs.remove(n);
                                    }
                                }
                                glutin::VirtualKeyCode::Space => {
                                    if let Some(n) = dirs.iter().position(|x| x == &Direction::UP) {
                                        dirs.remove(n);
                                    }
                                }
                                glutin::VirtualKeyCode::C => {
                                    if let Some(n) = dirs.iter().position(|x| x == &Direction::DOWN)
                                    {
                                        dirs.remove(n);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    mouse_delta = (
                        mouse_pos.0 - position.x as f32,
                        mouse_pos.1 - position.y as f32,
                    );
                    mouse_pos = (position.x as f32, position.y as f32);
                }
                glutin::WindowEvent::MouseInput { state, button, .. } => {
                    if state == glutin::ElementState::Pressed && button == glutin::MouseButton::Left
                    {
                        mouse_pressed = true;
                    }

                    if state == glutin::ElementState::Released
                        && button == glutin::MouseButton::Left
                    {
                        mouse_pressed = false;
                    }
                }
                _ => (),
            },
            _ => (),
        });

        if mouse_pressed {
            cam.move_target(mouse_delta.0, mouse_delta.1, dt as f32);
        }

        for dir in dirs.iter() {
            cam.move_cam(&dir, dt as f32);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        }

        program.bind();
        program.set_mat4("projection", &projection);
        program.set_mat4("view", &cam.view());
        program.set_mat4("model", &model);
        program.set_vec4("eye_pos", &cam.position.to_homogeneous());
        program.set_vec4("light_pos", &Point3::new(3.0, 1.0, 1.0).to_homogeneous());

        cube.draw();

        gl_window.swap_buffers().unwrap();

        let delta = time.elapsed();
        dt = (delta.as_micros() as f64) / 1000000.0;
        let fps = 1.0 / dt;
        print!("\r{:.8}", fps);
        time = Instant::now();
    }
}
