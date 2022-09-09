use macroquad::prelude::{Color, draw_circle_lines, Vec2};

pub struct Particle {
    color: Color,
    radius: f32,
    position: Vec2,
    velocity: Vec2,
    time_to_live: f32,
    max_time_to_live: f32,
    drag: f32,
}

impl Particle {
    pub fn new(color: Color, radius: f32, position: Vec2, velocity: Vec2, time_to_live: f32, drag: f32) -> Self {
        Self { color, radius, position, velocity, time_to_live, max_time_to_live: time_to_live, drag}
    }

    pub fn is_active(&self) -> bool {
        return self.time_to_live > 0.0;
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.is_active() { return; }
        self.velocity -= self.velocity * (self.drag * delta_time);
        self.position += self.velocity * delta_time;
        self.time_to_live -= delta_time;
    }

    pub fn draw(&self) {
        if !self.is_active() { return; }

        let normalized_time = self.time_to_live / self.max_time_to_live;
        let color = Color::new(self.color.r, self.color.g, self.color.b, normalized_time);
        draw_circle_lines(
            self.position.x,
            self.position.y,
            self.radius,
            1.0,
            color);
    }
}

pub fn emit_particle(particles: &mut Vec<Particle>, color: Color, position: Vec2, velocity: Vec2, time_to_live: f32, drag: f32) {
    let to_emit = Particle::new(color,1.5, position, velocity, time_to_live, drag);
    particles.push(to_emit);
}
