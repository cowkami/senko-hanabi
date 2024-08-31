use std::f32::consts;
use web_sys::{WebGl2RenderingContext as GL, *};

pub struct Cube {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u16>,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            vertices: Self::get_vertices(),
            colors: Self::get_colors(),
            indices: Self::get_indices(),
        }
    }

    fn get_vertices() -> Vec<f32> {
        vec![
            // 前面
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
            // 背面
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
            // 上面
            [-0.5, 0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            // 下面
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // 右面
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // 左面
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn get_colors() -> Vec<f32> {
        [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [1.0, 1.0, 0.0, 1.0],
        ]
        .repeat(6)
        .into_iter()
        .flatten()
        .collect()
    }

    fn get_indices() -> Vec<u16> {
        let vertex_indices = [0, 1, 2, 0, 2, 3];
        [vertex_indices; 6]
            .iter()
            .enumerate()
            .flat_map(|(i, v)| v.iter().map(move |u| u + 4 * i as u16))
            .collect::<Vec<_>>()
    }

    pub fn move_next_frame(
        &self,
        gl: &GL,
        location: &WebGlUniformLocation,
        canvas: &HtmlCanvasElement,
        frame_count: i32,
    ) {
        let radians = (frame_count % 360) as f32 * consts::PI / 180.0;
        let axis = glm::Vec3::new(1.0, 1.0, 1.0);
        let model_matrix = glm::rotate(&glm::Mat4::identity(), radians, &axis);

        let eye = glm::Vec3::new(0.0, 0.0, 3.0);
        let center = glm::Vec3::new(0.0, 0.0, 0.0);
        let up = glm::Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = glm::look_at(&eye, &center, &up);

        let aspect = canvas.width() as f32 / canvas.height() as f32;
        let fovy = 45.0 * consts::PI / 180.0;
        let near = 0.1;
        let far = 10.0;
        let projection_matrix = glm::perspective(aspect, fovy, near, far);

        let mvp_matrix = projection_matrix * view_matrix * model_matrix;
        let mvp_arrays: [[f32; 4]; 4] = mvp_matrix.into();
        let mvp_matrices = mvp_arrays.iter().flat_map(|a| *a).collect::<Vec<_>>();

        gl.uniform_matrix4fv_with_f32_array_and_src_offset_and_src_length(
            Some(location),
            false,
            &mvp_matrices,
            0,
            0,
        );
    }
}
