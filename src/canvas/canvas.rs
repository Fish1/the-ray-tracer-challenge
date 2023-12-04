use std::fs;

use crate::color::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Color::new_color(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.pixels[self.width * y + x] = *color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.pixels[self.width * y + x]
    }

    pub fn save_to_ppm(&self) {
        let mut data = format!("P3\n{} {}\n255\n", self.width, self.height);

        for pixel in self.pixels.iter() {
            let r = (255.0 * pixel.red()) as u8;
            let g = (255.0 * pixel.green()) as u8;
            let b = (255.0 * pixel.blue()) as u8;

            data += &format!("{} {} {} ", r, g, b);
        }

        data += "\n";

        fs::write("./image.ppm", data).expect("unable to write to file");
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::math::{transform::transform, tuple::Tuple};

    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(100, 100);
        let black = Color::new_color(0.0, 0.0, 0.0);
        for pixel in canvas.pixels.iter() {
            assert_eq!(pixel.equals(&black), true);
        }
    }

    #[test]
    fn set_pixel() {
        let mut canvas = Canvas::new(100, 100);
        let red = Color::new_color(1.0, 0.0, 0.0);
        canvas.set_pixel(3, 3, &red);
        let pixel = canvas.get_pixel(3, 3);
        assert_eq!(pixel.equals(&red), true);
    }

    #[test]
    fn draw_projectile() {
        let mut canvas = Canvas::new(1000, 1000);
        let red = Color::new_color(1.0, 0.0, 0.0);

        let mut projectile = Tuple::new_point(0.0, 999.0, 0.0);
        let mut velocity = Tuple::new_vector(3.0, -12.0, 0.0);
        let gravity = Tuple::new_vector(0.0, 0.1, 0.0);

        for _ in 0..250 {
            let x = projectile.x as usize;
            let y = projectile.y as usize;

            if x >= canvas.width || y >= canvas.height {
                break;
            }

            canvas.set_pixel(x, y, &red);

            projectile += velocity;
            velocity += gravity;
        }
        canvas.set_pixel(3, 3, &red);
        canvas.save_to_ppm();
    }

    #[test]
    fn draw_clock() {
        let mut canvas = Canvas::new(100, 100);
        let mut color = Color::new_color(1.0, 1.0, 0.5);

        let mut dot = Tuple::new_point(25.0, 0.0, 0.0);

        let offset = transform::new_translation(50.0, 50.0, 0.0);
        let rotate = transform::new_rotation_z(PI / 32.0);

        for _ in 0..100 {
            dot = &rotate * dot;
            let with_offset = &offset * dot;
            let x = with_offset.x as usize;
            let y = with_offset.y as usize;
            canvas.set_pixel(x, y, &color);
            color = &rotate * color;
        }

        canvas.save_to_ppm();
    }
}
