extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::{Canvas, RenderTarget};
use std::time::Duration;

struct canvas_buffer {
    brightness: [[u8;800];600],
    phase: i32
}

impl canvas_buffer {
    pub fn draw_on_canvas<T>(&self, canvas :&mut Canvas<T>) 
    where T: RenderTarget {
        for i in 0..600 {
            for j in 0..800 {
                canvas.set_draw_color(Color::RGB(self.brightness[i][j], self.brightness[i][j], self.brightness[i][j]));
                canvas.draw_point(Point::new(j as i32,i as i32));
            }
        }
    }
    
    pub fn sine_wave(& mut self) {
        for i in 0..600 {
            for j in 0..800 {
                let sin = ((j + self.phase as usize) as f32 / 800.0 * 20.0).sin() + 1.0;
                let b = 55 + (sin / 2.0 * 200.0) as u8; 
                self.brightness[i][j] = b;
            }
        }
        self.phase += 5;
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut buffer = canvas_buffer{brightness: [[0;800];600],phase:0};
    
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
        buffer.sine_wave();
        buffer.draw_on_canvas(&mut canvas);
        // The rest of the game loop goes here...
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}