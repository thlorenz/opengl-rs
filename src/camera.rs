extern crate nalgebra_glm as glm;

use self::glm::TMat4;

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,
    pub world_up: glm::Vec3,

    pub yaw: f32,
    pub pitch: f32,

    pub zoom: f32,

    pub mov_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            position: glm::vec3(0.0, 0.0, 3.0),
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            mov_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };
        camera.update_camera_vectors();
        camera
    }
}

impl Camera {
    pub fn get_view(&self) -> TMat4<f32> {
        let target = &self.position + &self.front;
        glm::look_at(&self.position, &target, &self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, dt: f32) {
        let velocity = self.mov_speed * dt;
        match direction {
            CameraMovement::Forward => {
                self.position += &self.front * velocity;
            }
            CameraMovement::Backward => {
                self.position -= &self.front * velocity;
            }
            CameraMovement::Left => {
                self.position -= &self.right * velocity;
            }
            CameraMovement::Right => {
                self.position += &self.right * velocity;
            }
        }
    }

    pub fn process_mouse_move(&mut self, dx: f32, dy: f32, constrain_pitch: bool) {
        let dx = dx * self.mouse_sensitivity;
        let dy = dy * self.mouse_sensitivity;

        self.yaw += dx;
        self.pitch += dy;

        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0
            }
        }
        self.update_camera_vectors();
    }

    pub fn process_mouse_scroll(&mut self, dy: f32) {
        self.zoom -= dy;
        if self.zoom < 1.0 {
            self.zoom = 1.0
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0
        }
    }

    fn update_camera_vectors(&mut self) {
        let front = glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
        self.right = self.front.cross(&self.world_up).normalize();
        self.up = self.right.cross(&self.front).normalize();
    }
}
