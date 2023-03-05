use crate::{Canvas, Color};

pub struct Shape<'a, C> {
    canvas: &'a mut C,
    fill: Color,
    outline: Color,
}

impl<'a, C> Shape<'a, C> {
    pub fn new(canvas: &'a mut C) -> Self {
        Self {
            canvas,
            fill: Color::WHITE,
            outline: Color::rgba(0.0, 0.0, 0.0, 0.0),
        }
    }
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = color;
        self
    }

    pub fn set_fill(&mut self, color: Color) {
        self.fill = color;
    }

    pub fn outline(mut self, color: Color) -> Self {
        self.outline = color;
        self
    }

    pub fn set_outline(&mut self, color: Color) {
        self.outline = color;
    }
}

impl<'a, C> Shape<'a, C>
where
    C: Canvas,
{
    pub fn rect(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.canvas.set(x, y, self.fill);
            }
        }

        if self.outline == self.fill {
            return;
        }

        if is_transparent(&self.outline) {
            return;
        }

        for x in x1..=x2 {
            self.canvas.set(x, y1, self.outline);
            self.canvas.set(x, y2, self.outline);
        }

        for y in y1..=y2 {
            self.canvas.set(x1, y, self.outline);
            self.canvas.set(x2, y, self.outline);
        }
    }

    pub fn line(&mut self, mut x1: i32, mut y1: i32, x2: i32, y2: i32) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.canvas.set(x1, y1, self.outline);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }

            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    // TODO: This iterates over all image pixels, which is slow
    //       can be optimized to only iterate over the potential circle pixels (bound by a square)
    pub fn circle(&mut self, x: i32, y: i32, r: i32) {
        let r2 = r * r;

        let (w, h) = self.canvas.size();

        for px in 0..w {
            for py in 0..h {
                let dx = px - x;
                let dy = py - y;

                let d2 = dx * dx + dy * dy;

                if !is_transparent(&self.fill) && d2 < r2 {
                    self.canvas.set(px, py, self.fill);
                }

                if !is_transparent(&self.outline) {
                    let diff = (d2 - r2).abs();
                    // We're comparing squares so abs difference of 25 is 5 pixels
                    if diff <= 25 {
                        self.canvas.set(px, py, self.outline);
                    }
                }
            }
        }
    }
}

fn is_transparent(color: &Color) -> bool {
    color.a() == 0.0
}
