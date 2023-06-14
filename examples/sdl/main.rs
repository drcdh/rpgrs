use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

/*
fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    canvas.copy(texture, None, None)?;

    canvas.present();

    Ok(())
}
*/

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Example: The Game", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    // let game = Game

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
			// game.process_input(event)
        }

		// game.update()

        // game.render

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
