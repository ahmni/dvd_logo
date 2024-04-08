extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::time::Duration;

struct DVDLogo {
    position: Position,
    velocity: Velocity,
    width: u32,
    height: u32,
    color: Color,
    sprite: Rect,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

fn render(
    texture: &sdl2::render::Texture,
    canvas: &mut WindowCanvas,
    dvd_logo: &DVDLogo,
    points: &Vec<Point>,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(155, 0, 50));
    for point in points.iter() {
        canvas.draw_point(*point)?;
    }
    canvas.copy(
        texture,
        dvd_logo.sprite,
        Rect::new(
            dvd_logo.position.x,
            dvd_logo.position.y,
            dvd_logo.width,
            dvd_logo.height,
        ),
    )?;
    Ok(())
}

fn get_random_color() -> Color {
    Color::RGB(
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
        rand::thread_rng().gen_range(0..255),
    )
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.load_texture("assets/DVD_logo.png")?;

    let mut dvd_logo = DVDLogo {
        position: Position { x: 0, y: 0 },
        velocity: Velocity { x: -2, y: -2 },
        width: 140,
        height: 100,
        sprite: Rect::new(14, 12, 612, 301), // hard coded to remove whitespace around logo
        color: Color::RGB(0, 255, 255),
    };

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut points: Vec<Point> = Vec::new();
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(0, 20, 50));
        canvas.clear();
        let mut hit_counter = 0;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        if dvd_logo.position.x + dvd_logo.width as i32 >= 800 || dvd_logo.position.x <= 0 {
            dvd_logo.velocity.x = -dvd_logo.velocity.x;
            dvd_logo.color = get_random_color();
            hit_counter += 1;
        }
        if dvd_logo.position.y + dvd_logo.height as i32 >= 600 || dvd_logo.position.y <= 0 {
            dvd_logo.velocity.y = -dvd_logo.velocity.y;
            dvd_logo.color = get_random_color();
            hit_counter += 1;
        }

        if hit_counter == 2 {
            println!("Hit corner");
        }

        // Uncomment to visualize pattern
        //points.push(Point::new(dvd_logo.position.x, dvd_logo.position.y));

        dvd_logo.position.x += dvd_logo.velocity.x;
        dvd_logo.position.y += dvd_logo.velocity.y;

        texture.set_color_mod(dvd_logo.color.r, dvd_logo.color.g, dvd_logo.color.b);
        render(&texture, &mut canvas, &dvd_logo, &points)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
