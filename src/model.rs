use gl::types::*;
use crate::buffer::*;

#[allow(dead_code)]
struct ModelVertex {
    position: glm::Vec3,
    normal: glm::Vec3
}

impl ModelVertex {
    fn new(position: glm::Vec3, normal: glm::Vec3) -> ModelVertex {
        ModelVertex {
            position,
            normal
        }
    }
}

pub struct Model {
    vertex_buffer: ArrayBuffer,
    index_buffer: ElementArrayBuffer,
    vao: VertexArray,
    count: GLsizei,
}

impl Model {
    pub fn new() -> Model {
        let vertex_buffer = ArrayBuffer::new();
        let index_buffer = ElementArrayBuffer::new();
        let vao = VertexArray::new();
        Model {
            vertex_buffer,
            index_buffer,
            vao,
            count: 0
        }
    }

    pub fn make_box(&mut self, box_min: glm::Vec3, box_max: glm::Vec3) {
        let vertices = vec![
            ModelVertex::new(glm::vec3(box_min.x, box_min.y, box_max.z), glm::vec3(0.0, 0.0, 1.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_min.y, box_max.z), glm::vec3(0.0, 0.0, 1.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_max.y, box_max.z), glm::vec3(0.0, 0.0, 1.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_max.y, box_max.z), glm::vec3(0.0, 0.0, 1.0)),

            ModelVertex::new(glm::vec3(box_max.x, box_min.y, box_min.z), glm::vec3(0.0, 0.0, -1.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_min.y, box_min.z), glm::vec3(0.0, 0.0, -1.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_max.y, box_min.z), glm::vec3(0.0, 0.0, -1.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_max.y, box_min.z), glm::vec3(0.0, 0.0, -1.0)),

            ModelVertex::new(glm::vec3(box_min.x, box_max.y, box_max.z), glm::vec3(0.0, 1.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_max.y, box_max.z), glm::vec3(0.0, 1.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_max.y, box_min.z), glm::vec3(0.0, 1.0, 0.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_max.y, box_min.z), glm::vec3(0.0, 1.0, 0.0)),

            ModelVertex::new(glm::vec3(box_min.x, box_min.y, box_min.z), glm::vec3(0.0, -1.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_min.y, box_min.z), glm::vec3(0.0, -1.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_min.y, box_max.z), glm::vec3(0.0, -1.0, 0.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_min.y, box_max.z), glm::vec3(0.0, -1.0, 0.0)),

            ModelVertex::new(glm::vec3(box_min.x, box_min.y, box_min.z), glm::vec3(0.0, 0.0, 1.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_min.y, box_max.z), glm::vec3(0.0, 0.0, 1.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_max.y, box_max.z), glm::vec3(0.0, 0.0, 1.0)),
            ModelVertex::new(glm::vec3(box_min.x, box_max.y, box_min.z), glm::vec3(0.0, 0.0, 1.0)),

            ModelVertex::new(glm::vec3(box_max.x, box_min.y, box_max.z), glm::vec3(1.0, 0.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_min.y, box_min.z), glm::vec3(1.0, 0.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_max.y, box_min.z), glm::vec3(1.0, 0.0, 0.0)),
            ModelVertex::new(glm::vec3(box_max.x, box_max.y, box_max.z), glm::vec3(1.0, 0.0, 0.0)),
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 0, 2, 3,           // Front
            4, 5, 6, 4, 6, 7,           // Back
            8, 9, 10, 8, 10, 11,        // Top
            12, 13, 14, 12, 14, 15,     // Bottom
            16, 17, 18, 16, 18, 19,     // Left
            20, 21, 22, 20, 22, 23,     // Right
        ];

        self.set_buffers(&vertices, &indices);
    }

    fn set_buffers(&mut self, vertices: &[ModelVertex], indices: &[u32]) {
        self.count = indices.len() as GLsizei;

        self.vao.bind();

        self.vertex_buffer.bind();
        self.vertex_buffer.static_draw_data(&vertices);

        self.index_buffer.bind();
        self.index_buffer.static_draw_data(&indices);

        unsafe {
            let pos_location: GLuint = 0;
            gl::EnableVertexAttribArray(pos_location);
            gl::VertexAttribPointer(
                pos_location,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<ModelVertex>()) as GLsizei,
                std::ptr::null()
            );

            let norm_location: GLuint = 1;
            gl::EnableVertexAttribArray(norm_location);
            gl::VertexAttribPointer(
                norm_location,
                3,
                gl::FLOAT,
                gl::FALSE,
                (std::mem::size_of::<ModelVertex>()) as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid
            );

        }

        self.vao.unbind();
        self.vertex_buffer.unbind();
        self.index_buffer.unbind();
    }

    pub fn draw(&self, model_mat: glm::Mat4, view_proj_mat: glm::Mat4, shader: GLuint) {
        unsafe {
            gl::UseProgram(shader);
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader, b"ModelMtx\0".as_ptr() as _), 1, gl::FALSE, model_mat.as_slice().as_ptr() as _);

            let mvp_mat = view_proj_mat * model_mat;
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader, b"ModelViewProjMtx\0".as_ptr() as _), 1, gl::FALSE, mvp_mat.as_slice().as_ptr() as _);
        }
        self.index_buffer.bind();
        self.vao.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.count, gl::UNSIGNED_INT, std::ptr::null());
        }
        self.vao.unbind();
        self.index_buffer.unbind();
        unsafe { gl::UseProgram(0); }
    }
}
