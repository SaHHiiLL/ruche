#![deny(missing_docs)]
#![allow(missing_docs)]

use raylib::prelude::*;
use tracing::Level;

mod board;
mod game;

fn main() {
    let (mut rl, thread) = raylib::init().size(500, 600).build();
    rl.set_target_fps(60);

    let (level, span) = if std::option_env!("LOGGER").is_some() {
        (Level::INFO, tracing::info_span!("Main"))
    } else {
        (Level::TRACE, tracing::trace_span!("Main"))
    };
    tracing_subscriber::fmt().with_max_level(level).init();

    let mut game = game::Game::new(500, 0, 100);
    game.load_images();
    game.board
        .load_position("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1".to_string());

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        game.draw_board(&mut d);

        if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_LEFT_BUTTON) {
            if game.selected.is_some() {
                game.make_move();
            } else {
                game.select_piece(&d);
            }
        }

        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_ESCAPE) {
            game.unset_selected();
        }

        if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_RIGHT_BUTTON) {
            game.unset_selected();
        }

        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_ENTER) {
            game.debug();
        }

        game.follow_mouse(&d);
    }
}
