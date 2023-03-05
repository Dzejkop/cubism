use bevy::prelude::Color;

use crate::canvas::Canvas;

pub struct PixelBuffer<'a> {
    pub width: i32,
    pub pixels: &'a mut [u8],
}

impl<'a> PixelBuffer<'a> {
    pub fn new(width: i32, pixels: &'a mut [u8]) -> Self {
        Self { width, pixels }
    }
}

impl<'a> Canvas for PixelBuffer<'a> {
    fn set(&mut self, x: i32, y: i32, color: Color) {
        let [r, g, b, a] = color.as_rgba_f32();

        let index = (y * self.width + x) as usize * 4;

        if index >= self.pixels.len() {
            return;
        }

        let dest_r = self.pixels[index] as f32 / 255.0;
        let dest_g = self.pixels[index + 1] as f32 / 255.0;
        let dest_b = self.pixels[index + 2] as f32 / 255.0;
        let dest_a = self.pixels[index + 3] as f32 / 255.0;

        let r = (r * a) + dest_r * (1.0 - a);
        let g = (g * a) + dest_g * (1.0 - a);
        let b = (b * a) + dest_b * (1.0 - a);
        let a = a + dest_a;

        self.pixels[index] = (r * 255.0) as u8;
        self.pixels[index + 1] = (g * 255.0) as u8;
        self.pixels[index + 2] = (b * 255.0) as u8;
        self.pixels[index + 3] = (a * 255.0) as u8;
    }

    fn size(&self) -> (i32, i32) {
        let width = self.width;
        let height = self.pixels.len() as i32 / (4 * width);

        (width, height)
    }

    fn clear(&mut self, color: Color) {
        let [r, g, b, a] = color.as_rgba_f32();

        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel[0] = (r * 255.0) as u8;
            pixel[1] = (g * 255.0) as u8;
            pixel[2] = (b * 255.0) as u8;
            pixel[3] = (a * 255.0) as u8;
        }
    }
}
