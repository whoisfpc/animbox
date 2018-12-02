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

fn main() {

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("animbox")
        .with_dimensions(LogicalSize::new(900.0, 700.0));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let shader_program = ShaderProgram::from_file("triangle", ProgramType::Render);
    let mut triangle = model::Model::new();
    triangle.make_triangle();

    let mut running = true;

    while running {
        events_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                        unsafe { gl::Viewport(0, 0, logical_size.width as i32, logical_size.height as i32); }
                    },
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.draw_triangle(shader_program.id());

        gl_window.swap_buffers().unwrap();
    }

}
