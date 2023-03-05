use std::ops::Mul;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use cubism::{Canvas, PixelBuffer};

// The buffer is 256x144 with this scale
const SCALE: f32 = 7.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1350.,
                height: 750.,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(render)
        .run();
}

#[derive(Resource)]
struct HeartSprites {
    full: Handle<Image>,
    half: Handle<Image>,
}

#[derive(Resource)]
struct MinecraftFont(ab_glyph::FontArc);

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    windows: Res<Windows>,
) {
    commands.spawn(Camera2dBundle::default());

    // We could also use assets, but this is simpler
    let font_data = include_bytes!("../fonts/Minecraft.ttf");
    let font = ab_glyph::FontArc::try_from_slice(font_data).unwrap();
    commands.insert_resource(MinecraftFont(font));

    let window = windows.primary();

    let width = window.width() as u32;
    let height = window.height() as u32;

    let ui_width = (width as f32 / SCALE) as u32;
    let ui_height = (height as f32 / SCALE) as u32;

    let ui_size = Extent3d {
        width: ui_width,
        height: ui_height,
        depth_or_array_layers: 1,
    };
    let data = vec![0; (ui_width * ui_height) as usize * 4];
    let mut ui_image = Image::new(
        ui_size,
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
    );
    ui_image.sampler_descriptor = ImageSampler::nearest();
    let ui_image = images.add(ui_image);

    // This is the image we'll use as canvas.
    // Note that if you have multiple images, you'll want to mark this entity with some marker component
    // or save the entity id in some resource.
    commands.spawn(SpriteBundle {
        texture: ui_image,
        transform: Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::ONE * SCALE),
        ..default()
    });

    commands.insert_resource(HeartSprites {
        full: assets.load("heart.png"),
        half: assets.load("half_heart.png"),
    });
}

fn render(
    time: Res<Time>,
    mut images: ResMut<Assets<Image>>,
    image: Query<&Handle<Image>>,
    heart_sprites: Res<HeartSprites>,
) {
    let image_handle = image.single();

    // We have to remove the canvas image from images
    // because we need to access it mutably while also accessing other image immutably
    let mut canvas_image = images.remove(image_handle).unwrap();

    let heart_images = images
        .get(&heart_sprites.full)
        .zip(images.get(&heart_sprites.half));

    if let Some((heart_img, half_heart_img)) = heart_images {
        let width = canvas_image.size().x;

        let mut canvas = PixelBuffer::new(width as i32, &mut canvas_image.data);
        let (w, h) = canvas.size();

        let img = if time.elapsed_seconds().mul(0.5).sin() > 0.0 {
            heart_img
        } else {
            half_heart_img
        };

        let img_w = img.size().x as i32;
        let img_h = img.size().y as i32;

        let bounce = (3.0 * time.elapsed_seconds().mul(3.0).sin()) as i32;

        let x = w / 2 - img_w / 2;
        let y = h / 2 - img_h / 2 + bounce;

        canvas.clear(Color::BLACK);

        canvas.blit().bevy_image(&img).pos(x, y).finish();
    }

    // Reinsert the canvas image
    let _ = images.set(image_handle, canvas_image);
}
