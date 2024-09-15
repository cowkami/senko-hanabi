use black_body::BlackBody;
use rand::prelude::*;
use std::f32::consts;
use std::iter::zip;

const GRAVITY: f32 = 9.81;
const KELVIN: f32 = 273.0;

const TIME_DELTA: f32 = 0.010;
const AIR_RESISTANCE: f32 = 0.1;
const MAX_SPEED: f32 = 50.0;

#[derive(Clone)]
struct Particle {
    position: [f32; 3],
    color: [f32; 4],
    velocity: [f32; 3],
    temperature: f32,
}

impl Particle {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Self {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            velocity: Self::init_velocity(
                rng.gen_range(0.0, MAX_SPEED),
                rng.gen_range(0.0, 2.0 * consts::PI),
                rng.gen_range(0.0, 2.0 * consts::PI),
            ),
            temperature: rng.gen_range(800.0 + KELVIN, 1000.0 + KELVIN),
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
        self.update_position();
        self.update_velocity();
        self.update_color();
    }

    fn update_position(&mut self) {
        self.position[0] += self.velocity[0] * TIME_DELTA;
        self.position[1] += self.velocity[1] * TIME_DELTA;
        self.position[2] += self.velocity[2] * TIME_DELTA;
    }

    fn update_velocity(&mut self) {
        let air = |v: f32| -> f32 { -v * (AIR_RESISTANCE * TIME_DELTA) };
        self.velocity[0] += air(self.velocity[0]);
        self.velocity[1] += air(self.velocity[1]) - (GRAVITY * TIME_DELTA);
        self.velocity[2] += air(self.velocity[2]);
    }

    fn update_color(&mut self) {
        let speed = [
            self.velocity[0].powi(2),
            self.velocity[1].powi(2),
            self.velocity[2].powi(2),
        ]
        .iter()
        .sum::<f32>()
        .sqrt();

        let intensity = 1.0 * speed / MAX_SPEED;

        let black_body = BlackBody::new(self.temperature as f64);
        let color = black_body.color_for_eye();

        self.color[0] = intensity * color[0] as f32;
        self.color[1] = intensity * color[1] as f32;
        self.color[2] = intensity * color[2] as f32;
    }
}

pub struct Fire {
    particles: Vec<Particle>,
}

impl Fire {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let particles = (0..100)
            .map(|_| Particle::new(&mut rng))
            .collect::<Vec<Particle>>();

        Self { particles }
    }

    pub fn update(&mut self) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
        let previous = self.particles.clone();

        let l = self.particles.len();
        for i in 0..l {
            self.particles[i].update();
        }
        let (prev_vertices, prev_colors) = Self::particles_to_vertices(&previous);
        let (current_vertices, current_colors) = Self::particles_to_vertices(&self.particles);

        let vertices = [prev_vertices, current_vertices].concat();
        let colors = [prev_colors, current_colors].concat();

        let vertex_count = vertices.len() / 3;
        let links: Vec<u16> = zip(0..(vertex_count / 2), (vertex_count / 2)..vertex_count)
            .into_iter()
            .flat_map(|s| vec![s.0 as u16, s.1 as u16])
            .collect::<Vec<u16>>();

        (vertices, colors, links)
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
