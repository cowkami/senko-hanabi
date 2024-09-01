use rand::prelude::*;

const GRAVITY: f32 = 9.81;
const TIME_DELTA: f32 = 0.01;
const RESISTANCE: f32 = 4.0;

#[derive(Clone)]
struct Particle {
    position: [f32; 3],
    color: [f32; 4],
    velocity: [f32; 3],
}

impl Particle {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                1.0,
            ],
            velocity: Self::init_velocity(rng),
        }
    }

    fn init_velocity(rng: &mut ThreadRng) -> [f32; 3] {
        [
            rng.gen_range(-8.0, 8.0),
            rng.gen_range(-8.0, 8.0),
            rng.gen_range(-8.0, 8.0),
        ]
    }

    fn step(&mut self) {
        self.update_velocity();
        self.update_position();
    }

    fn update_position(&mut self) {
        self.position[0] += self.velocity[0] * TIME_DELTA;
        self.position[1] += self.velocity[1] * TIME_DELTA;
        self.position[2] += self.velocity[2] * TIME_DELTA;
    }

    fn update_velocity(&mut self) {
        let f = |v: f32| -> f32 { -v * (RESISTANCE * TIME_DELTA) };
        self.velocity[0] += f(self.velocity[0]);
        self.velocity[1] += f(self.velocity[1]) - (GRAVITY * TIME_DELTA);
        self.velocity[2] += f(self.velocity[2]);
    }
}

pub struct Fire {
    particles: Vec<Particle>,
}

impl Fire {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let particles = (0..100000)
            .map(|_| Particle::new(&mut rng))
            .collect::<Vec<Particle>>();

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
