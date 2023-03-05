use std::ops::Mul;

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn as_rgba_f32(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn set_a(&mut self, a: f32) {
        self.a = a;
    }

    pub fn a(&self) -> f32 {
        self.a
    }
}

impl Mul<[f32; 4]> for Color {
    type Output = Color;

    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        Color {
            r: self.r * rhs[0],
            g: self.g * rhs[1],
            b: self.b * rhs[2],
            a: self.a * rhs[3],
        }
    }
}
