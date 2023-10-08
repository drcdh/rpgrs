mod animator;
mod components;
mod keyboard;
mod physics;
mod renderer;
mod scripts;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use specs::prelude::*;
use std::time::Duration;
use tiled::{Loader, Map};

use components::*;

const ZOOM: u32 = 2;

pub enum MovementCommand {
    Stop(Direction),
    Move(Direction),
}

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// Create animation frames for the standard character spritesheet
fn character_animation_frames(
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction,
    num_frames: i32,
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..num_frames {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}
/*
fn get_tileset_textures(texture_creator: &TextureCreator, tilesets: str) -> &[Texture] {
}

fn get_map_texture(texture_creator: &TextureCreator, map: &Map) -> Texture {
    let tileset_textures = get_tileset_textures(&texture_creator, &map.tilesets());
    let texture = texture_creator.create_texture(None, TextureAccess::Static, map.width*map.tile_width, map.height*map.tile_height);
    for layer in map.layers() {
//        render_layer(tileset_textures, layer
    }
}
*/
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    let texture_creator = canvas.texture_creator();

    let mut loader = Loader::new();
    let map = loader.load_tmx_map("assets/test.tmx").unwrap();
    let tilesets = map.tilesets();
    let tilecounts = [tilesets[0].tilecount, tilesets[1].tilecount];
    // SDL2 textures created from source images below

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard::new(), "Keyboard", &[])
        .with(scripts::MovementScripts, "MovementScripts", &[])
        .with(physics::Physics, "Physics", &["Keyboard", "MovementScripts"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    renderer::SystemData::setup(&mut world.res);

    // Initialize resource
    let movement_command: Option<MovementCommand> = None;
    world.add_resource(movement_command);

    let textures = [
        texture_creator.load_texture(&tilesets[0].image.as_ref().expect("No source image for tileset 0").source)?,
        texture_creator.load_texture(&tilesets[1].image.as_ref().expect("No source image for tileset 1").source)?,
        texture_creator.load_texture("assets/daniel16.png")?,
        texture_creator.load_texture("assets/rydia.png")?,
    ];
    let player_spritesheet = 2;
    let player_top_left_frame = Rect::new(0, 0, 16, 16);

    let player_animation = MovementAnimation {
        frame_period: 8,
        frames_since_update: 0,
        neutral_frame: 0,
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up, 4),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down, 4),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left, 4),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right, 4),
    };

    // Create the playable character entity
    world.create_entity()
        .with(KeyboardControlled)
        .with(Position {z: 1, location: Point::new(80, 120), orientation: Direction::Right})
        .with(Kinematics {velocity: Point::new(0, 0), max_speed: 1})
        .with(player_animation.right_frames[0].clone())  // Sprite
        .with(player_animation)  // MovementAnimation
        .build();

    let npc_spritesheet = 3;

    let npc_anim = MovementAnimation {
        frame_period: 16,
        frames_since_update: 0,
        neutral_frame: 0,
        current_frame: 0,
        up_frames: character_animation_frames(npc_spritesheet, player_top_left_frame, Direction::Up, 2),
        down_frames: character_animation_frames(npc_spritesheet, player_top_left_frame, Direction::Down, 2),
        left_frames: character_animation_frames(npc_spritesheet, player_top_left_frame, Direction::Left, 2),
        right_frames: character_animation_frames(npc_spritesheet, player_top_left_frame, Direction::Right, 2),
    };

    world.create_entity()
        .with(Script {script_fn: scripts::test as fn(&mut Position, &mut Kinematics)})
        .with(Position {z: 1, location: Point::new(130, 130), orientation: Direction::Right})
        .with(Kinematics {velocity: Point::new(0, 0), max_speed: 1})
        .with(npc_anim.right_frames[0].clone())  // Sprite
        .with(npc_anim)  // MovementAnimation
        .build();

    for layer in map.layers().filter_map(|layer| match layer.layer_type() {
        tiled::LayerType::Tiles(layer) => Some(layer),
        _ => None,
    }) {
        let (width, height) = (layer.width().expect("Map must be finite (for now)"), layer.height().expect("Map must be finite (for now)"));
        for x in 0..width {
            for y in 0..height {
                if let Some(layer_tile) = layer.get_tile(x as i32, y as i32) {
                        let tileset = layer_tile.get_tileset();
                        let tile_idx = layer_tile.id();
                        let location = Point::new((x * map.tile_width) as i32, (y * map.tile_height) as i32);
                        let tileset_coord_x: u32 = tile_idx % tileset.columns;
                        let tileset_coord_y: u32 = (tile_idx - tileset_coord_x)/tileset.columns;
                        let sprite = Sprite {
                            spritesheet: layer_tile.tileset_index(),  // FIXME
                            region: Rect::new(
                                (tileset_coord_x * tileset.tile_width) as i32,
                                (tileset_coord_y * tileset.tile_height) as i32,
                                tileset.tile_width,
                                tileset.tile_height,
                            ),
                        };
                        world.create_entity()
                            .with(Position {z: 0, location, orientation: Direction::Up})
                            .with(sprite)
                            .build();
                }
            }
        }
    }

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        let mut movement_command = None;
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                // Quit
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop(Direction::Left));
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop(Direction::Right));
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop(Direction::Up));
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop(Direction::Down));
                },
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Render
        renderer::render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &textures,
            world.system_data(),
            ZOOM,
        )?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
