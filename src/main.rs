extern crate cgmath;
extern crate gl;
extern crate gl_loader;
extern crate glutin;

mod camera;
mod frame;
mod mesh;
mod scene;
mod shaders;
mod utils;

use glutin::{GlContext, GlWindow};
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};

use camera::{Camera, Direction};
use shaders::{Program, Shader};

use cgmath::prelude::*;
use cgmath::{perspective, Deg, Matrix4, Point3, Vector3};

fn resize_window(window: &GlWindow, projection: &mut Matrix4<f32>) {
    let dpi = window.get_hidpi_factor();
    let wlsize = window.get_inner_size().unwrap();
    let wpsize = wlsize.to_physical(dpi);

    window.resize(wpsize);
    unsafe {
        gl::Viewport(0, 0, wpsize.width as i32, wpsize.height as i32);
    }

    *projection = perspective(
        Deg(75.0),
        wpsize.width as f32 / wpsize.height as f32,
        0.1,
        10.0,
    );
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_decorations(true)
        .with_transparency(false);
    //    .with_fullscreen(Some(events_loop.get_primary_monitor()));
    let context = glutin::ContextBuilder::new().with_vsync(false);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    let dpi = gl_window.get_hidpi_factor();
    unsafe {
        gl_window.make_current().unwrap();
        println!("gl_lib: {}", gl_loader::init_gl());
        // println!("glad: {}", gl_loader::start_glad());
        gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
    }

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthMask(gl::TRUE);
        gl::DepthFunc(gl::LEQUAL);
        gl::DepthRange(0.0, 1.0);
        gl::Enable(gl::DEPTH_CLAMP);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        let wlsize = gl_window.get_inner_size().unwrap();
        let wpsize = wlsize.to_physical(dpi);
        gl::Viewport(0, 0, wpsize.width as i32, wpsize.height as i32);
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
    let wpsize = gl_window.get_inner_size().unwrap().to_physical(dpi);
    let mut projection: Matrix4<f32> = perspective(
        Deg(75.0),
        wpsize.width as f32 / wpsize.height as f32,
        0.1,
        10.0,
    );
    let mut cam = Camera::new(
        Point3::new(0.0, 0.0, -2.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let mut time = Instant::now();
    let mut dt: f64 = 0.0;

    let mut mouse_init = false;
    let mut mouse_prev: (f64, f64) = (0.0, 0.0);
    let mut mouse_next: (f64, f64) = (0.0, 0.0);
    let mut mouse_pressed = false;

    // forward, backward, left, right, up, down
    let mut dirs = [false, false, false, false, false, false];

    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(_) => resize_window(&gl_window, &mut projection),
                glutin::WindowEvent::HiDpiFactorChanged(_) => {
                    resize_window(&gl_window, &mut projection)
                }
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(vkey) = input.virtual_keycode {
                        match vkey {
                            glutin::VirtualKeyCode::Escape => running = false,
                            glutin::VirtualKeyCode::W => {
                                dirs[0] = input.state == glutin::ElementState::Pressed;
                            }
                            glutin::VirtualKeyCode::S => {
                                dirs[1] = input.state == glutin::ElementState::Pressed;
                            }
                            glutin::VirtualKeyCode::A => {
                                dirs[2] = input.state == glutin::ElementState::Pressed;
                            }
                            glutin::VirtualKeyCode::D => {
                                dirs[3] = input.state == glutin::ElementState::Pressed;
                            }
                            glutin::VirtualKeyCode::Space => {
                                dirs[4] = input.state == glutin::ElementState::Pressed;
                            }
                            glutin::VirtualKeyCode::LShift => {
                                dirs[5] = input.state == glutin::ElementState::Pressed;
                            }
                            _ => {}
                        }
                    }
                }
                glutin::WindowEvent::CursorMoved { position, .. } => {
                    if mouse_pressed {
                        if !mouse_init {
                            mouse_prev = (position.x, position.y);
                            mouse_init = true;
                        } else {
                            mouse_next = (position.x, position.y);
                        }
                    }
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
            let mouse_delta = (mouse_prev.0 - mouse_next.0, mouse_prev.1 - mouse_next.1);
            cam.move_target(mouse_delta.0 as f32, mouse_delta.1 as f32, dt as f32);
            mouse_prev = mouse_next;
        }

        for (i, &val) in dirs.iter().enumerate() {
            if val {
                let dir = match i {
                    0 => Direction::FORWARD,
                    1 => Direction::BACKWARD,
                    2 => Direction::LEFT,
                    3 => Direction::RIGHT,
                    4 => Direction::UP,
                    5 => Direction::DOWN,
                    _ => Direction::FORWARD,
                };
                cam.move_cam(&dir, dt as f32);
            }
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
        dt = (delta.subsec_micros() as f64) / 1_000_000.0;
        let fps = 1.0 / dt;
        print!("\r{:.8} ms", dt * 1000.0);
        time = Instant::now();
    }
}
