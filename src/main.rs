extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::{Canvas, RenderTarget};
use std::time::Duration;

struct canvas_buffer {
    brightness: Vec<Vec<u8>>,
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
    
    pub fn circle(&mut self, center: Point, radius: f32) {
        for y in 0..600i32 {
            for x in 0..800i32 {
                let r = (((x - center.x).pow(2) + (y - center.y).pow(2)) as f32).sqrt();
                if r.round() == radius.round() {
                    self.brightness[y as usize][x as usize] = 255;
                } else {
                    self.brightness[y as usize][x as usize] = 0;
                }
            }
        }
    }
    
    pub fn circle1(&mut self, center: Point, radius: f32) {
        for y in 0..600i32 {
            for x in 0..800i32 {
                let r = (((x - center.x).pow(2) + (y - center.y).pow(2)) as f32).sqrt();
                self.brightness[y as usize][x as usize] = (r / 500f32 * 255f32) as u8;
            }
        }
    }
    
    pub fn circle2(&mut self, center: Point, radius: f32) {
        for y in 0..600i32 {
            for x in 0..800i32 {
                let r = (((x - center.x).pow(2) + (y - center.y).pow(2)) as f32).sqrt();
                let b = 100f32 + ((r *(1f32-r/1400f32)/5f32 + self.phase as f32).sin() + 1f32) / 2f32 * 50f32;
                self.brightness[y as usize][x as usize] =( b * (1f32-(r/1400f32))) as u8 ;
            }
        }
        self.phase += 5;
    }
    
    pub fn blend(&mut self, other: &Self) {
        for y in 0..600 {
            for x in 0..800 {
                self.brightness[y][x] = ((self.brightness[y][x] as i32 + other.brightness[y][x] as i32) / 2) as u8;
            }
        }
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

    let mut buffer: Box<canvas_buffer> = Box::new(canvas_buffer{brightness: vec![vec![0;800];600],phase:0});
    let mut buffer2: Box<canvas_buffer> = Box::new(canvas_buffer{brightness: vec![vec![0;800];600],phase:0});
    
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
        // buffer.sine_wave();
        // buffer.circle(Point::new(400, 300), 80f32);
        buffer.circle2(Point::new(200, 350), 80f32);
        buffer2.circle2(Point::new(600, 350), 40f32);
        buffer.blend(&buffer2);
        buffer.draw_on_canvas(&mut canvas);
        // The rest of the game loop goes here...
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}