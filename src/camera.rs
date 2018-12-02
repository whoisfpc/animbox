use glm;

pub struct Camera {
    fov: f32,
    aspect: f32,
    near_clip: f32,
    far_clip: f32,
    distance: f32,
    azimuth: f32,
    incline: f32,
    view_proj_mat: glm::Mat4,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            fov: 45.0,
            aspect: 1.33,
            near_clip: 0.1,
            far_clip: 100.0,
            distance: 10.0,
            azimuth: 0.0,
            incline: 20.0,
            view_proj_mat: glm::Mat4::identity(),
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }

    pub fn set_azimuth(&mut self, azimuth: f32) {
        self.azimuth = azimuth;
    }

    pub fn set_incline(&mut self, incline: f32) {
        self.incline = incline;
    }

    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    pub fn get_azimuth(&self) -> f32 {
        self.azimuth
    }

    pub fn get_incline(&self) -> f32 {
        self.incline
    }

    pub fn get_view_proj_mat(&self) -> glm::Mat4 {
        self.view_proj_mat
    }

    pub fn update(&mut self) {
        let mut world = glm::Mat4::identity();
        world[(2, 3)] = self.distance;
        world = glm::rotation(glm::pi::<f32>() * -self.azimuth / 180.0, &glm::vec3(0.0, 1.0, 0.0))
            * glm::rotation(glm::pi::<f32>() * -self.incline / 180.0, &glm::vec3(1.0, 0.0, 0.0))
            * world;

        let view = glm::inverse(&world);
        let project = glm::perspective(self.aspect, glm::pi::<f32>() * self.fov / 180.0 , self.near_clip, self.far_clip);
        self.view_proj_mat = project * view;
    }
}
