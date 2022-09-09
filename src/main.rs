mod game_sounds;
mod particle_system;
mod player;

use {game_sounds::*, particle_system::*, player::*};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Scratchpad".to_owned(),
        window_width: 630,
        window_height: 500,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Loading the assets takes some time, so in the meantime show loading text.
    clear_background(BLACK);
    let text_width = measure_text("Loading...", None, 64, 1.0);
    draw_text_ex(
        "Loading...",
        screen_width() / 2.0 - text_width.width / 2.0,
        screen_height() / 2.0,
        TextParams {
            font_size: 64,
            color: WHITE,
            ..Default::default()
        },
    );

    next_frame().await;

    set_pc_assets_folder("assets");
    let mut game_sounds = GameSounds::new().await;
    game_sounds.play_looping_ambience();

    let mut player = Player::new(Vec2 {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    });
    let mut particles: Vec<Particle> = Vec::new();

    // Finished loading and initializing everything, play the main game loop.
    loop {
        let delta_time = get_frame_time();

        // ===================================================
        // HANDLE INPUT
        // ===================================================

        player.handle_input(delta_time);

        // ===================================================
        // UPDATE
        // ===================================================

        player.update(delta_time, &mut particles, &mut game_sounds);
        for p in particles.iter_mut() {
            p.update(delta_time);
        }
        particles.retain(|p| p.is_active());

        // ===================================================
        // DRAW
        // ===================================================

        clear_background(BLACK);
        player.draw();
        for p in particles.iter_mut() {
            p.draw();
        }

        let particle_size = particles.len();
        draw_text(
            format!("Particles {particle_size}").as_str(),
            20.0,
            100.0,
            32.0,
            WHITE,
        );

        let position_string = format!("[{:.0}, {:.0}]", player.position.x, player.position.y);
        let velocity_string = format!("[{:.0}, {:.0}]", player.velocity.x, player.velocity.y);
        draw_text(
            format!("Position: {position_string}").as_str(),
            20.0,
            200.0,
            32.0,
            WHITE,
        );
        draw_text(
            format!("Velocity: {velocity_string}").as_str(),
            20.0,
            200.0 + 32.0,
            32.0,
            WHITE,
        );

        next_frame().await
    }
}
