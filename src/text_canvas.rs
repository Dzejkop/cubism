use ab_glyph::{point, Font, Glyph, Point, PxScale, ScaleFont};
use bevy::prelude::Color;

use crate::Canvas;

pub struct TextCanvas<'a, T, F> {
    canvas: T,
    font: &'a F,
}

impl<'a, T, F> TextCanvas<'a, T, F> {
    pub fn new(canvas: T, font: &'a F) -> Self {
        Self { canvas, font }
    }
}

impl<'a, T, F> TextCanvas<'a, T, F>
where
    T: Canvas,
    F: Font,
{
    #[inline]
    pub fn text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        let font = self.font.as_scaled(PxScale::from(16.0));
        let mut glyphs = Vec::with_capacity(text.len());

        layout_paragraph(font, point(x as f32, y as f32), 1000.0, text, &mut glyphs);

        for g in glyphs {
            if let Some(og) = font.outline_glyph(g) {
                let bounds = og.px_bounds();
                og.draw(|x, y, v| {
                    let x = (x as f32 + bounds.min.x) as i32;
                    let y = (y as f32 + bounds.min.y) as i32;

                    let mut color = color;
                    color.set_a(v);

                    self.canvas.set(x, y, color);
                });
            }
        }
    }
}

// Taken from https://github.com/alexheretic/ab-glyph/blob/main/dev/src/layout.rs
pub fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: ab_glyph::Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}
