use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use std::time::Duration;

use crate::internal::entities::Entity;
use crate::internal::entities::player::Player;

mod internal;

pub fn main() {
    // Initialize SDL3 context
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Build the window
    let window = video_subsystem
        .window("SDL3 Rust Demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Initialize Canvas for drawing
    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize Scene Elements
    let mut player = Player::new();

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        if key == Keycode::Escape {
                            break 'running;
                        }
                        player.move_player(key);
                    }
                }
                _ => {}
            }
        }

        // Clear Screen
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();

        // Update Elements
        player.update();

        // Draw Elements
        player.draw(&mut canvas);

        canvas.present();

        std::thread::sleep(Duration::from_millis(16)); // Cap at ~60 FPS
    }
}
