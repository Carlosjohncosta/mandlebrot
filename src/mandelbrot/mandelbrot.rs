use num_complex::{Complex64, ComplexFloat};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ScreenCoord {
    pub x: i32,
    pub y: i32,
}

impl ScreenCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub enum IsInSet {
    Is,
    Not(u32),
}

pub struct Mandelbrot {
    width: i32,
    height: i32,
    pub max_iter: u32,
    scale: f64,
    center: Complex64,
}

impl Mandelbrot {
    pub fn new(width: i32, height: i32, max_iter: u32) -> Self {
        Self {
            width,
            height,
            max_iter,
            scale: 350.0,
            center: Complex64::new(0.0, 0.0),
        }
    }

    fn coord_to_complex(&self, coord: ScreenCoord) -> Complex64 {
        let x_centered = coord.x - (self.width / 2);
        let y_centered = -coord.y + (self.height / 2);
        let real = x_centered as f64 / self.scale;
        let imaginary = y_centered as f64 / self.scale;
        Complex64::new(real, imaginary) + self.center
    }

    fn check_in_set(&self, initial: Complex64) -> IsInSet {
        let mut z = initial;
        for i in 0..self.max_iter {
            z = z.powu(2) + initial;
            if z.abs() > 2.0 {
                return IsInSet::Not(i);
            }
        }
        IsInSet::Is
    }

    pub fn resize(&mut self, screen_coord: ScreenCoord) {
        let new_center = self.coord_to_complex(screen_coord);
        self.center = new_center;
        self.scale *= 2.0;
        self.max_iter += 10;
    }

    pub fn generate<'a>(&'a self) -> impl Iterator<Item = (ScreenCoord, IsInSet)> + 'a {
        (0..self.width).flat_map(move |x| {
            (0..self.height).map(move |y| {
                let screen_coord = ScreenCoord::new(x, y);
                let complex = self.coord_to_complex(screen_coord);
                let is_in_set = self.check_in_set(complex);
                (screen_coord, is_in_set)
            })
        })
    }
}
