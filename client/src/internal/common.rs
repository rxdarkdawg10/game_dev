use std::path::Path;

use sdl3::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, FRect, TextureCreator, TextureQuery},
    video::Window,
};
use sdl3_sys::surface;

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

pub fn render_spritesheet(
    sprite: Vector2,
    size: f32,
    sprite_loc: Vector2,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    let spr_surface = sdl3::surface::Surface::load_bmp("client/assets/spritesheet.bmp").unwrap();
    let texture_creator = canvas.texture_creator();
    let spr_texture = texture_creator
        .create_texture_from_surface(&spr_surface)
        .unwrap();

    let sprite_width = 32;
    let sprite_height = 32;

    let src_rect = FRect::new(
        sprite.x,
        sprite.y,
        sprite_width as f32,
        sprite_height as f32,
    );
    let dst_rect = FRect::new(
        800.0 - (sprite_width as f32 * size) - 10.0,
        0.0,
        sprite_width as f32 * size,
        sprite_height as f32 * size,
    );
    canvas.copy(&spr_texture, src_rect, dst_rect).unwrap();

    Ok(())
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
