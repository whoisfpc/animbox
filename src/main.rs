extern crate glutin;
extern crate gl;
extern crate nalgebra_glm as glm;

use glutin::GlContext;
use glutin::Event;
use glutin::dpi::*;

mod shader_program;
use shader_program::*;

mod model;
mod camera;
mod spinning_cube;

fn main() {

    let mut width: f32 = 900.0;
    let mut height: f32 = 700.0;
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    let mut left_down = false;
    let mut middle_down = false;
    let mut right_down = false;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("animbox")
        .with_dimensions(LogicalSize::new(width as f64, height as f64));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::Viewport(0, 0, width as i32, height as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
    }

    let shader_program = ShaderProgram::from_file("model", ProgramType::Render);
    let mut camera = camera::Camera::new();
    camera.set_aspect(width / height);
    let mut cube = spinning_cube::SpinningCube::new();

    let mut running = true;

    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                        width = logical_size.width as f32;
                        height = logical_size.height as f32;
                        unsafe { gl::Viewport(0, 0, width as i32, height as i32); }
                        camera.set_aspect(width / height);
                    },
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::R) => {
                                if input.state == glutin::ElementState::Pressed {
                                    camera.reset();
                                    camera.set_aspect(900.0/700.0);
                                    cube.reset();
                                }
                            },
                            _ => {}
                        }
                    },
                    glutin::WindowEvent::CursorMoved { position, .. } => {
                        let max_delta: f32 = 100.0;
                        let dx = glm::clamp_scalar(position.x as f32 - mouse_x, -max_delta, max_delta);
                        let dy = glm::clamp_scalar(-(position.y as f32 - mouse_y), -max_delta, max_delta);

                        mouse_x = position.x as f32;
                        mouse_y = position.y as f32;

                        if left_down {
                            let rate = 1.0;
                            let azimuth = camera.get_azimuth();
                            let incline = camera.get_incline();
                            camera.set_azimuth(azimuth + dx * rate);
                            camera.set_incline(glm::clamp_scalar(incline-dy*rate, -90.0, 90.0));
                        }

                        if right_down {
                            let rate = 0.005;
                            let distance = glm::clamp_scalar(camera.get_distance() * (1.0 - dx * rate), 0.01, 1000.0);
                            camera.set_distance(distance);
                        }
                    },
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        match button {
                            glutin::MouseButton::Left => left_down = state == glutin::ElementState::Pressed,
                            glutin::MouseButton::Right => right_down = state == glutin::ElementState::Pressed,
                            glutin::MouseButton::Middle => middle_down = state == glutin::ElementState::Pressed,
                            _ => {}
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        camera.update();
        cube.update();
        cube.draw(camera.get_view_proj_mat(), shader_program.id());

        gl_window.swap_buffers().unwrap();
    }

}
