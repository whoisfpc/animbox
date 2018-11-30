extern crate glutin;
extern crate gl;

use glutin::GlContext;
use glutin::Event;
use glutin::dpi::*;

use gl::types::*;


const VS_SRC: &'static [u8] = b"
#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;

out Vertex_Data {
    vec3 color;
} OUT;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    OUT.color = color;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 330 core

in Vertex_Data {
    vec3 color;
} IN;

void main() {
    gl_FragColor = vec4(IN.color, 1.0);
}
\0";

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

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 1.0
    ];

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vs = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    let fs = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

    let program_id = unsafe { gl::CreateProgram() };
    let mut vao: GLuint = 0;

    unsafe {
        // Setup shader compilation checks
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // -1 to skip trialing null character

        gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
        gl::CompileShader(vs);
        // Check for shader compilation errors
        gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                vs,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }
        gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
        gl::CompileShader(fs);
        // Check for shader compilation errors
        gl::GetShaderiv(fs, gl::COMPILE_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetShaderInfoLog(
                fs,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }

        gl::AttachShader(program_id, vs);
        gl::AttachShader(program_id, fs);
        gl::LinkProgram(program_id);

        // Check for linking errors
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        if success != i32::from(gl::TRUE) {
            gl::GetProgramInfoLog(
                program_id,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        gl::UseProgram(program_id);

        let mut vbo: GLuint = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as GLsizei,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as GLsizei,
            (2 * std::mem::size_of::<f32>()) as *const GLvoid
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }


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
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }

        gl_window.swap_buffers().unwrap();
    }

}
