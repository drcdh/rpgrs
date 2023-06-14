use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    layers: &[Texture],
    textures: &[Texture],
    data: SystemData,
    zoom: u32,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for layer in layers {
        canvas.copy(&layer, Rect::new(0, 80, 320, 320), Rect::new(0, 0, zoom*320, zoom*240))?;
    }
    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos.location + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, zoom*current_frame.width(), zoom*current_frame.height());
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
