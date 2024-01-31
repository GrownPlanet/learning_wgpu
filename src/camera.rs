use glam::{f32::Mat4, Vec3, Vec4};

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, -0.5, -0.5,
    0.0, 0.0, 0.0, -1.0
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
        // let view = cgmath::Matrix4::look_to_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj
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
