use model::*;
use glm;
use gl::types::*;

pub struct SpinningCube {
    cube: Model,
    position: glm::Vec3,
    axis: glm::Vec3,
    spin_delta: f32,
    angle: f32,
    world_mat: glm::Mat4,
}

impl SpinningCube {
    pub fn new() -> SpinningCube {
        let mut cube = Model::new();
        cube.make_box(glm::vec3(-1.0, -1.0, -1.0), glm::vec3(1.0, 1.0, 1.0));
        SpinningCube {
            cube,
            position: glm::vec3(0.0, 0.0, 0.0),
            axis: glm::vec3(0.0, 1.0, 0.0),
            spin_delta: 1.0,
            angle: 0.0,
            world_mat: glm::Mat4::identity()
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.angle += self.spin_delta * dt;
        self.world_mat = glm::rotation(self.angle, &self.axis);
        self.world_mat = glm::translate(&self.world_mat, &self.position);
    }

    pub fn draw(&self, view_proj_mat: glm::Mat4, shader: GLuint) {
        self.cube.draw(self.world_mat, view_proj_mat, shader);
    }

    pub fn reset(&mut self) {
        self.angle = 0.0;
        self.world_mat = glm::Mat4::identity();
    }
}
