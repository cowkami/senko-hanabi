const GRAVITY: f32 = 9.81;
const TIME_DELTA: f32 = 0.01;

struct Particle {
    position: [f32; 3],
    color: [f32; 4],
    velocity: [f32; 3],
}

impl Particle {
    fn step(&mut self) {
        self.position[0] += self.velocity[0] * TIME_DELTA;
        self.position[1] += self.velocity[1] * TIME_DELTA;
        self.position[2] += self.velocity[2] * TIME_DELTA;
        self.velocity[1] -= GRAVITY * TIME_DELTA;
    }
}

pub struct Fire {
    particles: Vec<Particle>,
}

impl Fire {
    pub fn new() -> Self {
        let particles = vec![Particle {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 0.5],
            velocity: [2.0, 4.0, 0.0],
        }];

        Self { particles }
    }

    pub fn step(&mut self) {
        let l = self.particles.len();
        for i in 0..l {
            self.particles[i].step();
        }
    }

    pub fn update(&mut self) -> (Vec<f32>, Vec<f32>) {
        self.step();
        let (vertices, colors) = self.particles_to_vertices();
        (vertices, colors)
    }

    fn particles_to_vertices(&mut self) -> (Vec<f32>, Vec<f32>) {
        let mut vertices = vec![];
        let mut colors = vec![];

        for particle in &self.particles {
            vertices.extend(particle.position.clone());
            colors.extend(particle.color.clone());
        }

        (vertices, colors)
    }
}
