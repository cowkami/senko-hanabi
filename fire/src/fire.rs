pub struct Fire {
    particles: Vec<Particle>,
}

struct Particle {
    position: Vec<f32>,
    color: Vec<f32>,
}

impl Fire {
    pub fn new() -> Self {
        let mut particles = vec![Particle {
            position: vec![0.0, 0.0, 0.0],
            color: vec![1.0, 0.0, 0.0, 1.0],
        }];

        Self { particles }
    }

    pub fn update(&self) -> (Vec<f32>, Vec<f32>) {
        let (vertices, colors) = self.particles_to_vertices();
        (vertices, colors)
    }

    fn particles_to_vertices(&self) -> (Vec<f32>, Vec<f32>) {
        let mut vertices = get_vertices();
        let mut colors = get_colors();

        for particle in &self.particles {
            vertices.extend(particle.position.clone());
            colors.extend(particle.color.clone());
        }

        (vertices, colors)
    }
}

fn get_vertices() -> Vec<f32> {
    vec![-0.5, -0.5, 0.5]
}

fn get_colors() -> Vec<f32> {
    vec![1.0, 0.0, 0.0, 1.0]
}
