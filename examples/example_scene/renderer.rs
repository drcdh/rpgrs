use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};
use tiled::{Map, Tileset};

use crate::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
    zoom: u32,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let mut data = (&data.0, &data.1).join().collect::<Vec<_>>();
    data.sort_by(|&a, &b| a.0.z.cmp(&b.0.z));
    for (pos, sprite) in data.iter() {
        let current_frame = sprite.region;

        let screen_rect = Rect::new(zoom as i32 * pos.location.x, zoom as i32 * pos.location.y, zoom*sprite.region.width(), zoom*sprite.region.height());
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
