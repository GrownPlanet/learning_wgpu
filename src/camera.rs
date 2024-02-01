use glam::{f32::Mat4, Vec3, Vec4};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0
]);

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [f32; 16],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: *Mat4::ZERO.as_ref(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = *camera.build_view_projection_matrix().as_mut();
    }
}

fn _print_4x4(mat: Mat4) {
    fn print_v4(v: Vec4) {
        println!("[{}, {}, {}, {}]", v.x, v.y, v.z, v.w);
    }
    print_v4(mat.x_axis);
    print_v4(mat.y_axis);
    print_v4(mat.z_axis);
    print_v4(mat.w_axis);
}

pub struct CameraControler {
    speed: f32,
    forward_pressed: bool,
    backward_pressed: bool,
    right_pressed: bool,
    left_pressed: bool,
}

impl CameraControler {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            forward_pressed: false,
            backward_pressed: false,
            right_pressed: false,
            left_pressed: false,
        }
    }

    pub fn process_keypress(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyW),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.forward_pressed = true;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyS),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.backward_pressed = true;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyD),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.right_pressed = true;
                true
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::KeyA),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.left_pressed = true;
                true
            }
            _ => false,
        }
    }

    pub fn update(&self, camera: &mut Camera) {
        println!("{}", self.forward_pressed);
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        if self.forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.length();

        if self.right_pressed {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.left_pressed {
            camera.eye = camera.target + (forward + right * self.speed).normalize() * forward_mag;
        }
    }
}
