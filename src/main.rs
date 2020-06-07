use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

const PLAYER_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    direction: Direction,
    next_frame: i32,
    speed: i32,
}

/// Update the player current frame.
/// This function is to be called only once on each iteration
/// of the main loop.
fn update_player_frame(player: &mut Player){
    player.sprite = Rect::new(
        //(player.next_frame / 5) * 94,
        (player.next_frame / 3) * 72,
        //direction_to_spritesheet_row(player.direction) * 100,
        direction_to_spritesheet_row(player.direction) * 102,
        //94,
        72,
        //100,
        102,
    );
    player.next_frame += 1;
    //player.next_frame %= 3 * 5;
    player.next_frame %= 12 * 3;
}

/// Update the player speed.
/// This function is to be called only once on each iteration
/// of the main loop.
fn update_player(player: &mut Player){
    use self::Direction::*;
    player.position = match player.direction {
        Up => player.position.offset(0, -player.speed),
        Down => player.position.offset(0, player.speed),
        Left => player.position.offset(-player.speed, 0),
        Right => player.position.offset(player.speed, 0),
    };
}

/// Returns the row of the spritesheet corresponding to the given direction
fn direction_to_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        //Up => 3,
        //Down => 0,
        //Left => 1,
        //Right => 2,
        Up => 0,
        Down => 0,
        Left => 1,
        Right => 0,
    }
}

/// Render the window.
fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (frame_width, frame_height) = player.sprite.size();
    let screen_rect = Rect::from_center(
        player.position,
        frame_width,
        frame_height,
    );
    canvas.copy(
        texture,
        player.sprite,
        screen_rect,
    )?;
    canvas.present();

    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("my litle game", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    //let texture = texture_creator.load_texture("assets/raptor.png")?;
    let texture = texture_creator.load_texture("assets/sky-sheet.png")?;

    let (width, height) = canvas.output_size()?;

    let mut player = Player {
        position: Point::new(width as i32 / 2,  height as i32 / 2),
        //sprite: Rect::new(0, 0, 94, 100),
        sprite: Rect::new(0, 0, 72, 102),
        direction: Direction::Down,
        next_frame: 0,
        speed: 0,
    };

    update_player_frame(&mut player);
    render(
        &mut canvas,
        Color::RGB(0, 255, 255),
        &texture,
        &player,
    )?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Up;
                    player.speed = PLAYER_SPEED;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Down;
                    player.speed = PLAYER_SPEED;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Left;
                    player.speed = PLAYER_SPEED;
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.direction = Direction::Right;
                    player.speed = PLAYER_SPEED;
                },
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.speed = 0;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        update_player_frame(&mut player);
        update_player(&mut player);
        render(
            &mut canvas,
            Color::RGB(i, 64, 255 - i),
            &texture,
            &player,
        )?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
