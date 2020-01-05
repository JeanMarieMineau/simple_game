extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut point1 = Point::new(100, 300);
    let mut point2 = Point::new(100, 300);
    let mut point3 = Point::new(100, 300);
    let center_point = Point::new(400, 300);
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
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
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        point1 = next_pos(point1, 10);
        point2 = next_pos(point2, 1);
        point3 = next_pos(point3, 100);
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        canvas.draw_line(point1, center_point).expect("");
        canvas.set_draw_color(Color::RGB(0, 210, 255));
        canvas.draw_line(point2, center_point).expect("");
        canvas.set_draw_color(Color::RGB(255, 0, 210));
        canvas.draw_line(point3, center_point).expect("");



        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn next_pos(point: Point, d: i32)->Point{
    if point.x() == 100 && point.y() == 100{
        point.offset(d, 0) 
    }else if point.x() == 700 && point.y() == 100{
        point.offset(0, d)
    }else if point.x() == 700 && point.y() == 500{
        point.offset(-d, 0) 
    }else if point.x() == 100 && point.y() == 500{
        point.offset(0, -d)   
    }else if point.x() == 100{
        point.offset(0, -d)   
    }else if point.y() == 100{
        point.offset(d, 0)    
    }else if point.x() == 700{
        point.offset(0, d)  
    } else if point.y() == 500{
        point.offset(-d, 0) 
    }else{
        panic!("point1 should not be here: ({}, {})", point.x(), point.y());
    }
}
