mod mandelbrot;
use mandelbrot::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;

fn hsl_to_rgb(h: f64) -> Color {
    let s = 1.0; // Saturation
    let l = 0.5; // Lightness (for full brightness)

    let c = (1.0 - (2f64 * l - 1f64).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let (r, g, b) = (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    );

    Color::RGB(r, g, b)
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let (width, height) = (700, 750);
    let window = video_subsystem
        .window("SDL2 Demo", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut generated = false;
    let mut mandelbrot = Mandelbrot::new(width, height, 40);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::MouseButtonDown { x, y, .. } => {
                    mandelbrot.resize(ScreenCoord::new(x, y));
                    generated = false;
                }
                _ => {}
            }
        }

        if generated {
            continue;
        }

        for (coord, is_in_set) in mandelbrot.generate() {
            match is_in_set {
                IsInSet::Is => {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                IsInSet::Not(iteration) => {
                    canvas.set_draw_color(hsl_to_rgb((iteration % 100) as f64 * 3.6))
                }
            }
            canvas.draw_point(Point::new(coord.x, coord.y)).unwrap()
        }
        canvas.present();
        // Insert a delay to reduce CPU usage
        // std::thread::sleep(std::time::Duration::from_millis(10));
        generated = true;
    }
}
