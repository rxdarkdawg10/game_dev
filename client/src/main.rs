use sdl3::event::Event;
use sdl3::pixels::Color;
use std::time::Duration;

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

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Update & Render
        canvas.set_draw_color(Color::RGB(0, 64, 255));
        canvas.clear();
        canvas.present();

        std::thread::sleep(Duration::from_millis(16)); // Cap at ~60 FPS
    }
}
