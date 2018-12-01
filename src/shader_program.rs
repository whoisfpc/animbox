use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use gl::types::*;

#[allow(dead_code)]
pub enum ProgramType {
    Geometry,
    Render,
    Compute,
}

pub struct ShaderProgram {
    program_id: GLuint,
}

impl ShaderProgram {
    pub fn from_file(name: &str, program_type: ProgramType) -> ShaderProgram {
        let mut shaders: Vec<Shader> = Vec::new();

        match program_type {
            ProgramType::Geometry => {
                shaders.push(Shader::from_cstr(&Self::read_cstr(&format!("{}{}", name, ".vert")), ShaderType::Vertex));
                shaders.push(Shader::from_cstr(&Self::read_cstr(&format!("{}{}", name, ".geom")), ShaderType::Geometry));
                shaders.push(Shader::from_cstr(&Self::read_cstr(&format!("{}{}", name, ".frag")), ShaderType::Fragment));
            },
            ProgramType::Render => {
                shaders.push(Shader::from_cstr(&Self::read_cstr(&format!("{}{}", name, ".vert")), ShaderType::Vertex));
                shaders.push(Shader::from_cstr(&Self::read_cstr(&format!("{}{}", name, ".frag")), ShaderType::Fragment));
            },
            ProgramType::Compute => {
                shaders.push(Shader::from_cstr(&Self::read_cstr(&format!("{}{}", name, ".comp")), ShaderType::Compute));
            },
        }

        let program_id = unsafe { gl::CreateProgram() };
        for shader in &shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }
        unsafe {
            gl::LinkProgram(program_id);
        }
        for shader in &shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        ShaderProgram {
            program_id
        }
    }

    pub fn id(&self) -> GLuint {
        self.program_id
    }

    fn read_cstr(filename: &str) -> CString {
        let mut file = File::open(filename).unwrap();
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        unsafe { CString::from_vec_unchecked(contents) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

enum ShaderType {
    Geometry,
    Vertex,
    Fragment,
    Compute,
}

struct Shader {
    shader_id: GLuint,
}

impl Shader {
    fn from_cstr(content: &CStr, shader_type: ShaderType) -> Shader {
        let shader_kind = match shader_type {
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
        };
        let shader_id = unsafe { gl::CreateShader(shader_kind) };

        unsafe {
            gl::ShaderSource(shader_id, 1, &content.as_ptr(), std::ptr::null());
            gl::CompileShader(shader_id);

            // Setup shader compilation checks
            let mut success = i32::from(gl::FALSE);
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // -1 to skip trialing null character
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetShaderInfoLog(
                    shader_id,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }
        }

        Shader {
            shader_id
        }
    }

    fn id(&self) -> GLuint {
        self.shader_id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader_id);
        }
    }
}
