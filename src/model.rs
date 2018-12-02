use gl::types::*;
use glm;

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
    vertex_buffer: GLuint,
    index_buffer: GLuint,
    vao: GLuint,
    count: GLsizei,
}

impl Model {
    pub fn new() -> Model {
        let mut vertex_buffer: GLuint = 0;
        let mut index_buffer: GLuint = 0;
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::GenBuffers(1, &mut index_buffer);
            gl::GenVertexArrays(1, &mut vao);
        }
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

        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<ModelVertex>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

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

            gl::DisableVertexAttribArray(pos_location);
            gl::DisableVertexAttribArray(norm_location);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn draw(&self, model_mat: glm::Mat4, view_proj_mat: glm::Mat4, shader: GLuint) {
        unsafe {
            gl::UseProgram(shader);
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader, b"ModelMtx\0".as_ptr() as _), 1, gl::FALSE, model_mat.as_slice().as_ptr() as _);

            let mvp_mat = view_proj_mat * model_mat;
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader, b"ModelViewProjMtx\0".as_ptr() as _), 1, gl::FALSE, mvp_mat.as_slice().as_ptr() as _);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BindVertexArray(self.vao);

            let pos_location: GLuint = 0;
            let norm_location: GLuint = 1;

            gl::EnableVertexAttribArray(pos_location);
            gl::EnableVertexAttribArray(norm_location);

            gl::DrawElements(gl::TRIANGLES, self.count, gl::UNSIGNED_INT, std::ptr::null());

            gl::DisableVertexAttribArray(pos_location);
            gl::DisableVertexAttribArray(norm_location);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::UseProgram(0);
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.index_buffer);
            gl::DeleteBuffers(1, &mut self.vertex_buffer);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
