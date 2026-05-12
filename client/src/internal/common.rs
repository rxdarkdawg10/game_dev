use std::path::Path;

use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureQuery},
    video::Window,
};

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x: x, y: y }
    }
}

pub fn render_text(
    text: &str,
    canvas: &mut Canvas<Window>,
    size: f32,
    color: Color,
    loc: Vector2,
) -> Result<(), String> {
    let ttf_context = sdl3::ttf::init().unwrap();

    let ttf_font = ttf_context
        .load_font(Path::new("client/assets/fonts/UbuntuMono-R.ttf"), size)
        .unwrap();

    // let ttf_surface = if text.len() * size as usize > 800 as usize {
    //     let count = (text.len() * size as usize - 800 as usize) / 18;
    //     ttf_font.render(&text[count..]).blended(color).unwrap()
    // } else {
    //     ttf_font.render(&text).blended(color).unwrap()
    // };

    let ttf_surface = ttf_font.render(&text).blended(color).unwrap();

    let texture_creator = canvas.texture_creator();
    let ttf_texture = texture_creator
        .create_texture_from_surface(&ttf_surface)
        .unwrap();
    let TextureQuery { width, height, .. } = ttf_texture.query();

    let target_rec = Rect::new(loc.x as i32, loc.y as i32, width, height);
    canvas.copy(&ttf_texture, None, target_rec).unwrap();

    Ok(())
}
