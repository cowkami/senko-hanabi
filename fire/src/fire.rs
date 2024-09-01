use rand::prelude::*;
use std::f32::consts;

const GRAVITY: f32 = 9.81;
const TIME_DELTA: f32 = 0.03;
const RESISTANCE: f32 = 0.8;

#[derive(Clone)]
struct Particle {
    position: [f32; 3],
    color: [f32; 4],
    velocity: [f32; 3],
}

impl Particle {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            position: [0.0, 0.7, 0.0],
            color: [1.0, 0.7, 0.2, 1.0],
            velocity: Self::init_velocity(
                rng.gen_range(0.0, 2.0 * consts::PI),
                rng.gen_range(0.0, 2.0 * consts::PI),
                rng.gen_range(10.0, 20.0),
            ),
        }
    }

    fn init_velocity(velocity: f32, latitude: f32, longitude: f32) -> [f32; 3] {
        [
            latitude.cos() * longitude.sin() * velocity,
            latitude.cos() * longitude.cos() * velocity,
            latitude.sin() * velocity,
        ]
    }

    fn update(&mut self) {
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
        let particles = (0..1000)
            .map(|_| Particle::new(&mut rng))
            .collect::<Vec<Particle>>();

        Self { particles }
    }

    pub fn update(&mut self) -> (Vec<f32>, Vec<f32>) {
        let previous = self.particles.clone();

        let l = self.particles.len();
        for i in 0..l {
            self.particles[i].update();
        }
        let (prev_vertices, prev_colors) = Self::particles_to_vertices(&previous);
        let (current_vertices, current_colors) = Self::particles_to_vertices(&self.particles);

        let vertices = [prev_vertices, current_vertices].concat();
        let colors = [prev_colors, current_colors].concat();

        (vertices, colors)
    }

    fn particles_to_vertices(particles: &Vec<Particle>) -> (Vec<f32>, Vec<f32>) {
        let mut vertices = vec![];
        let mut colors = vec![];

        for particle in particles {
            vertices.extend(particle.position.clone());
            colors.extend(particle.color.clone());
        }

        (vertices, colors)
    }
}
