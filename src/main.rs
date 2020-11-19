extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

type Sprite = Rect;

const PLAYER_MOVEMENT_SPEED: i32 = 10;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Sprite,
    pub speed: i32,
    pub direction: Direction,
    pub current_frame: i32,
}
pub fn direction_spritesheet_row(direction: Direction) -> i32 {
    match direction {
        Direction::Down => 0,
        Direction::Left => 1,
        Direction::Right => 2,
        Direction::Up => 3,
    }
}

pub fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;
    let (frame_width, frame_height) = player.sprite.size();

    let current_frame = Rect::new(
        // Frame heights for the cut out from sprite sheet
        // Current frame for animation
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height,
    );

    // Makes center of screen 0,0
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);

    let screen_rect = Rect::from_center(
        screen_position,
        player.sprite.width(),
        player.sprite.height(),
    );

    canvas.copy(&texture, current_frame, screen_rect)?;
    canvas.present();
    Ok(())
}

pub fn update_player(player: &mut Player) {
    match player.direction {
        Direction::Right => player.position = player.position.offset(player.speed, 0),
        Direction::Left => player.position = player.position.offset(-player.speed, 0),
        Direction::Up => player.position = player.position.offset(0, -player.speed),
        Direction::Down => player.position = player.position.offset(0, player.speed),
    };
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("Could not init video");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not convert window to canvas");

    let texture_creator = canvas.texture_creator();
    let texture_bytes = include_bytes!("../assets/bardo.png");
    let texture = texture_creator.load_texture_bytes(texture_bytes)?;

    let position = Point::new(-PLAYER_MOVEMENT_SPEED, 10);
    let sprite = Rect::new(0, 0, 26, 36);
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut player = Player {
        position,
        sprite,
        speed: 0,
        current_frame: 0,
        direction: Direction::Right,
    };
    'running: loop {
        // Get Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => player.speed = 0,
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        update_player(&mut player);
        if player.speed > 0 {
            player.current_frame = (player.current_frame + 1) % 3;
        };

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
