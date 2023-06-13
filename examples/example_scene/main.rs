mod animator;
mod components;
mod keyboard;
mod physics;
mod renderer;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture, TextureAccess, TextureCreator};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use specs::prelude::*;
use std::time::Duration;
use tiled::{Loader, Map};

use components::*;

//const ZOOM: u32 = 2;

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
/*
fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    world_texture: &Texture,
    player_texture: &Texture,
    player: &Player,
    camera: &Kinematics,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (player_frame_width, player_frame_height) = player.sprite.size();
    let player_current_frame = Rect::new(
        player.sprite.x() + player_frame_width as i32 * player.current_frame,
        player.sprite.y() + player_frame_height as  i32 * direction_spritesheet_row(player.direction),
        player_frame_width,
        player_frame_height,
    );

    // Treat the center of the screen as the (0, 0) coordinate
    let player_screen_position = player.kinematics.position + Point::new(width as i32 / 2, height as i32 / 2);
    let player_screen_rect = Rect::from_center(player_screen_position, ZOOM*player_frame_width, ZOOM*player_frame_height);

	// TODO: un-hardcode this
    let world_current_frame = Rect::new(0, 0, 192, 128);
    let world_screen_position = Point::new(-96, -64) + Point::new(width as i32 / 2, height as i32 / 2);
    let world_screen_rect = Rect::from_center(world_screen_position, ZOOM*192, ZOOM*128);

    canvas.copy(world_texture, world_current_frame, world_screen_rect)?;
    
    // Draw the player character
    canvas.copy(player_texture, player_current_frame, player_screen_rect)?;

    canvas.present();

    Ok(())
}
*/
/// Create animation frames for the standard character spritesheet
fn character_animation_frames(
	spritesheet: usize,
	top_left_frame: Rect,
	direction: Direction,
) -> Vec<Sprite> {
    // All assumptions about the spritesheets are now encapsulated in this function instead of in
    // the design of our entire system. We can always replace this function, but replacing the
    // entire system is harder.

    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
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
fn update_player(player: &mut Player) {
    update_location(&mut player.kinematics);
}

fn get_tileset_textures(texture_creator: &TextureCreator, tilesets: str) -> &[Texture] {
}

fn get_map_texture(texture_creator: &TextureCreator, map: &Map) -> Texture {
	let tileset_textures = get_tileset_textures(&texture_creator, &map.tilesets());
	let texture = texture_creator.create_texture(None, TextureAccess::Static, map.width*map.tile_width, map.height*map.tile_height);
	for layer in map.layers() {
//		render_layer(tileset_textures, layer
	}
}
*/
fn main() -> Result<(), String> {
//    let mut loader = Loader::new();
//    let map = loader.load_tmx_map("assets/test.tmx").unwrap();
//	let map_texture = get_map_texture(map);

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

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    renderer::SystemData::setup(&mut world.res);

    // Initialize resource
    let movement_command: Option<MovementCommand> = None;
    world.add_resource(movement_command);

    let textures = [
		texture_creator.load_texture("assets/daniel16.png")?,
	];
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 16, 16);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
    };

	let layers = [
		texture_creator.load_texture("assets/test.png")?,
	];
/*
	let mut camera = Kinematics {
		position: Point::new(0, 0),
		mark: Point::new(0, 0),
		velocity: Point::new(0, 0),
		max_speed: ZOOM as i32 * 2,
	};
    let mut player = Player {
		kinematics: Kinematics {
			position: Point::new(0, 0),
			mark: Point::new(0, 0),
			velocity: Point::new(0, 0),
			max_speed: ZOOM as i32 * 2,
		},
        sprite: Rect::new(0, 0, 16, 16),
        direction: Direction::Right,
        current_frame: 0,
    };
*/
	// Create the playable character entity
    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(-10*16, -10*16)))
        .with(Kinematics {mark: Position(Point::new(-10*16, -10*16)), velocity: Position(Point::new(0, 0)), max_speed: 4, orientation: Direction::Right})
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

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
//        update_player(&mut player);
//        update_location(&mut camera);
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(i, 64, 255 - i), &layers, &textures, world.system_data())?;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
