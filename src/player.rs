use macroquad::{
  prelude::*,
  rand::RandomRange,
};

use {
  crate::particle_system::{self, Particle},
  crate::game_sounds::GameSounds
};

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    accel: f32,
    radius: f32,
    is_thrusting_up: bool,
    is_thrusting_down: bool,
    is_thrusting_left: bool,
    is_thrusting_right: bool,
    is_grounded: bool,
}

impl Player {

    pub fn new(position: Vec2) -> Self {
        Self {
            position: position,
            velocity: Vec2::ZERO,
            accel: 600.0,
            radius: 32.0,
            is_thrusting_down: false,
            is_thrusting_left: false,
            is_thrusting_right: false,
            is_thrusting_up: false,
            is_grounded: false,
        }
    }

    pub fn handle_input(&mut self, delta_time: f32) {
        self.is_thrusting_down = false;
        self.is_thrusting_left = false;
        self.is_thrusting_right = false;
        self.is_thrusting_up = false;

        if is_key_down(KeyCode::Left) {
            self.velocity.x -= self.accel * delta_time;
            self.is_thrusting_right = true;
        }
        if is_key_down(KeyCode::Right) {
            self.velocity.x += self.accel * delta_time;
            self.is_thrusting_left = true;
        }
        if is_key_down(KeyCode::Up) {
            self.velocity.y -= self.accel * delta_time;
            self.is_thrusting_down = true;
        }
        if is_key_down(KeyCode::Down) {
            self.velocity.y += self.accel * delta_time;
            self.is_thrusting_up = true;
        }
    }

    pub fn update(&mut self, delta_time: f32, particles: &mut Vec<Particle>, sounds: &mut GameSounds) {
        const BOUNCE_FACTOR: f32 = 0.5;
        const GRAVITY: Vec2 = Vec2{x: 0.0, y: 300.0};
        const DRAG:f32 = 0.1;

        let is_any_thrusting = self.is_thrusting_down || self.is_thrusting_left || self.is_thrusting_right || self.is_thrusting_up;

        if is_any_thrusting && !sounds.get_is_jet_playing() {
            sounds.play_jet();
        }
        if !is_any_thrusting {
            sounds.stop_jet();
        }

        // Apply world forces.
        self.velocity += GRAVITY * delta_time;
        self.velocity -= self.velocity * DRAG * delta_time;

        // Move.
        self.position += self.velocity * delta_time;

        // Collide with floor.
        let mut was_impacted = false;
        let speed = self.velocity.length();
        const IMPACT_SPEED_MIN: f32 = 150.0;
        let was_hard_impact = speed > IMPACT_SPEED_MIN;
        if self.position.y + self.radius > screen_height() {
            self.position.y = screen_height() - self.radius;
            self.velocity.y *= -BOUNCE_FACTOR;

            if !self.is_grounded && was_hard_impact {
                sounds.play_impact();
                was_impacted = true;
            }
            self.is_grounded = true;
        }
        else {
            self.is_grounded = false;
        }

        // Collide with walls.
        if self.position.x + self.radius > screen_width() {
            self.position.x = screen_width() - self.radius;
            self.velocity.x *= -BOUNCE_FACTOR;
            if was_hard_impact {
                sounds.play_impact();
            }
            was_impacted = true;
        }

        if self.position.x - self.radius < 0.0 {
            self.position.x = self.radius;
            self.velocity.x *= -BOUNCE_FACTOR;
            if was_hard_impact {
                sounds.play_impact();
            }

            was_impacted = true;
        }

        const SCREAM_CHANCE: f32 = 0.5;
        if was_impacted && RandomRange::gen_range(0.0, 1.0) < SCREAM_CHANCE && was_hard_impact{
            sounds.play_sciencist_fear();
        }

        // Emit thruster particles.
        const P_LIFESPAN: f32 = 1.0;
        const P_DRAG: f32 = 3.5;
        let random_accel_side = self.accel / 5.0;
        let random_accel_thrust = rand::RandomRange::gen_range(self.accel * 0.8, self.accel * 1.2);
        if self.is_thrusting_down {
            particle_system::emit_particle(particles,
                WHITE,
                self.position + Vec2{x: 0.0, y: self.radius},
                Vec2 {
                    x: rand::RandomRange::gen_range(-random_accel_side, random_accel_side),
                    y: random_accel_thrust} + self.velocity,
                P_LIFESPAN, P_DRAG);
        }

        if self.is_thrusting_right {
            particle_system::emit_particle(particles,
                WHITE,
                self.position + Vec2{x: self.radius, y: 0.0},
                Vec2 {
                    x: random_accel_thrust,
                    y: rand::RandomRange::gen_range(-random_accel_side, random_accel_side) } + self.velocity,
                P_LIFESPAN, P_DRAG);
        }

        if self.is_thrusting_left {
            particle_system::emit_particle(particles,
                WHITE,
                self.position - Vec2{x: self.radius, y: 0.0},
                Vec2 {
                    x: -random_accel_thrust,
                    y: rand::RandomRange::gen_range(-random_accel_side, random_accel_side) } + self.velocity,
                P_LIFESPAN, P_DRAG);
        }

        if self.is_thrusting_up {
            particle_system::emit_particle(particles,
                WHITE,
                self.position - Vec2{x: 0.0, y: self.radius},
                Vec2 {
                    x: rand::RandomRange::gen_range(-random_accel_side, random_accel_side),
                    y: -random_accel_thrust} + self.velocity,
                P_LIFESPAN, P_DRAG);
        }

    }

    pub fn draw(&self) {
        draw_circle_lines(
            self.position.x,
            self.position.y,
            self.radius,
            2.0, RED);
    }
}
