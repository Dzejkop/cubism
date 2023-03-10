use crate::{Canvas, Color};

pub struct Blit<'a, C> {
    canvas: &'a mut C,
    src_x: i32,
    src_y: i32,
    src_w: i32,
    src_h: i32,
    dst_x: i32,
    dst_y: i32,
    img_data: &'a [u8],
    img_stride: usize,
    mul_color: Color,
}

#[cfg(feature = "bevy_ext")]
impl<'a, C> Blit<'a, C> {
    pub fn bevy_image(mut self, image: &'a bevy::prelude::Image) -> Self {
        use bevy::render::render_resource::TextureFormat;

        self.img_data = &image.data;
        let stride_mul = match image.texture_descriptor.format {
            TextureFormat::Rgba8Unorm => 1,
            TextureFormat::Rgba8UnormSrgb => 1,
            unsupported => panic!("Unsupported texture format: {unsupported:?}!",),
        };

        self.img_stride = stride_mul * image.size().x as usize;

        self.src_w = image.size().x as i32;
        self.src_h = image.size().y as i32;

        self
    }
}

impl<'a, C> Blit<'a, C> {
    pub fn new(canvas: &'a mut C) -> Self {
        Self {
            canvas,
            src_x: 0,
            src_y: 0,
            src_w: 0,
            src_h: 0,
            dst_x: 0,
            dst_y: 0,
            img_data: &[],
            img_stride: 0,
            mul_color: Color::WHITE,
        }
    }

    pub fn image_full(mut self, img_data: &'a [u8], stride: usize) -> Self {
        self.img_data = img_data;
        self.img_stride = stride;

        self.src_w = stride as i32 / 4;
        self.src_h = img_data.len() as i32 / (4 * stride as i32);

        self
    }

    pub fn image_clip(
        mut self,
        img_data: &'a [u8],
        stride: usize,
        src_x: i32,
        src_y: i32,
        src_w: i32,
        src_h: i32,
    ) -> Self {
        self.img_data = img_data;
        self.img_stride = stride;

        self.src_x = src_x;
        self.src_y = src_y;
        self.src_w = src_w;
        self.src_h = src_h;
        self
    }

    pub fn color(mut self, mul_color: Color) -> Self {
        self.mul_color = mul_color;
        self
    }

    pub fn pos(mut self, dst_x: i32, dst_y: i32) -> Self {
        self.dst_x = dst_x;
        self.dst_y = dst_y;
        self
    }
}

impl<'a, C> Blit<'a, C>
where
    C: Canvas,
{
    pub fn finish(self) {
        let sampler = BlitSampler::new(self.img_data, self.img_stride as i32);

        let mul_color = self.mul_color.as_rgba_f32();

        for x in 0..self.src_w {
            for y in 0..self.src_h {
                let src_x = self.src_x + x;
                let src_y = self.src_y + y;

                let dst_x = self.dst_x + x;
                let dst_y = self.dst_y + y;

                let color = sampler.sample(src_x, src_y);
                let color = color * mul_color;

                self.canvas.set(dst_x, dst_y, color);
            }
        }
    }
}

struct BlitSampler<'a> {
    img_data: &'a [u8],
    stride: i32,
}

impl<'a> BlitSampler<'a> {
    fn new(img_data: &'a [u8], stride: i32) -> Self {
        Self { img_data, stride }
    }

    fn sample(&self, x: i32, y: i32) -> Color {
        let index = (y * self.stride + x) as usize * 4;

        if index >= self.img_data.len() {
            return Color::rgba_u8(0, 0, 0, 0);
        }

        let r = self.img_data[index];
        let g = self.img_data[index + 1];
        let b = self.img_data[index + 2];
        let a = self.img_data[index + 3];

        Color::rgba_u8(r, g, b, a)
    }
}
