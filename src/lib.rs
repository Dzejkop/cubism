mod canvas;
mod pixel_buffer;
mod text_canvas;

#[cfg(not(feature = "bevy_color"))]
mod color;

#[cfg(feature = "bevy_color")]
pub use bevy::prelude::Color;

pub use self::canvas::blit::Blit;
pub use self::canvas::Canvas;
#[cfg(not(feature = "bevy_color"))]
pub use self::color::Color;
pub use self::pixel_buffer::PixelBuffer;
pub use self::text_canvas::TextCanvas;
