use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use std::time::Duration;

use sdl2::image::{self, LoadTexture, InitFlag};

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();

    let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;
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
    let texture = texture_creator.load_texture("assets/raptor.png")?;

    let (width, height) = canvas.output_size()?;
    let position = Point::new(width as i32 / 2,  height as i32 / 2);
    let sprite = Rect::new(0, 0, 94, 100);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    render(&mut canvas, Color::RGB(0, 255, 255), &texture, position, sprite)?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut j : i32 = 0;
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
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        let sprite = Rect::new((j / 5) * 94, 0, 94, 100);
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, position, sprite)?;
        j = (j + 1) % (3 * 5);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
