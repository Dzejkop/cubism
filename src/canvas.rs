use bevy::prelude::Color;

use self::shape::Shape;
use crate::Blit;

pub mod blit;
pub mod shape;

pub trait Canvas {
    fn set(&mut self, x: i32, y: i32, color: Color);

    fn size(&self) -> (i32, i32);

    fn clear(&mut self, color: Color) {
        let (width, height) = self.size();

        for x in 0..width {
            for y in 0..height {
                self.set(x, y, color);
            }
        }
    }

    fn shape(&mut self) -> Shape<'_, Self>
    where
        Self: Sized,
    {
        Shape::new(self)
    }

    fn blit(&mut self) -> Blit<'_, Self>
    where
        Self: Sized,
    {
        Blit::new(self)
    }
}

impl<'a, T> Canvas for &'a mut T
where
    T: Canvas,
{
    fn set(&mut self, x: i32, y: i32, color: Color) {
        T::set(self, x, y, color);
    }

    fn size(&self) -> (i32, i32) {
        T::size(self)
    }
}
