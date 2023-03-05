use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use cubism::{Canvas, PixelBuffer, TextCanvas};
use rand::{Rng, SeedableRng};

// The buffer is 256x144 with this scale
const SCALE: f32 = 7.5;

const STREAK_SPEED: f32 = 1.0;

const BUTTON_WIDTH: i32 = 160;
const BUTTON_HEIGHT: i32 = 80;

const BUTTON_OUTLINE_THICKNESS: i32 = 5;
const CONTENT_OFFSET: i32 = 24;

const STREAK_LENGTH: f32 = 20.0;

const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

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
struct State {
    streaks: Vec<Streak>,
}

impl Default for State {
    fn default() -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(1024);

        const N: usize = 64;

        let streaks = (0..N)
            .map(|_| Streak {
                x: rng.gen_range(0..BUTTON_WIDTH - STREAK_LENGTH as i32),
                y: rng.gen_range(0..BUTTON_HEIGHT - STREAK_LENGTH as i32),
                progress: rng.gen_range(0.0..1.0),
            })
            .collect();

        Self { streaks }
    }
}

#[derive(Clone)]
struct Streak {
    x: i32,
    y: i32,
    progress: f32,
}

#[derive(Resource)]
struct MinecraftFont(ab_glyph::FontArc);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Res<Windows>) {
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
        TextureFormat::Rgba8UnormSrgb,
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

    commands.insert_resource(State::default());
}

fn render(
    time: Res<Time>,
    mut images: ResMut<Assets<Image>>,
    image: Query<&Handle<Image>>,
    mut state: ResMut<State>,
    font: Res<MinecraftFont>,
) {
    let image = image.single();

    let image = images.get_mut(image).unwrap();
    let width = image.size().x;

    let mut canvas = PixelBuffer::new(width as i32, &mut image.data);

    let t = time.elapsed_seconds();

    let (w, h) = canvas.size();

    let offset_x = (w - BUTTON_WIDTH) / 2;
    let offset_y = (h - BUTTON_HEIGHT) / 2;

    canvas.clear(Color::BLACK);

    draw_thick_rect_outline(
        &mut canvas,
        offset_x,
        offset_y,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        BUTTON_OUTLINE_THICKNESS,
    );

    for streak in &mut state.streaks {
        let mut streak = streak.clone();
        streak.progress = ((STREAK_SPEED * t) + streak.progress) % 1.0;

        draw_streak(&mut canvas, offset_x, offset_y, &streak);
    }

    draw_rect_highlight(
        t,
        &mut canvas,
        offset_x - BUTTON_OUTLINE_THICKNESS,
        offset_y - BUTTON_OUTLINE_THICKNESS,
        BUTTON_WIDTH + (BUTTON_OUTLINE_THICKNESS * 2),
        BUTTON_HEIGHT + (BUTTON_OUTLINE_THICKNESS * 2),
    );

    draw_button_content(&mut canvas, offset_x, offset_y, BUTTON_WIDTH, BUTTON_HEIGHT);

    {
        // Manually centered
        // TODO: Allow calculating layout before rendering
        let text_offset_x = offset_x + CONTENT_OFFSET + 26;
        let text_offset_y = offset_y + CONTENT_OFFSET + 10;

        let mut text_canvas = TextCanvas::new(&mut canvas, &font.0);
        text_canvas.text(text_offset_x, text_offset_y, "Cubism", Color::WHITE);
    }
}

fn draw_thick_rect_outline(
    canvas: &mut impl Canvas,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    thickness: i32,
) {
    let mut shape = canvas
        .shape()
        .fill_color(Color::WHITE)
        .outline_color(TRANSPARENT);

    shape.rect(x - thickness, y - thickness, x + w + thickness, y);
    shape.rect(x - thickness, y + h, x + w + thickness, y + h + thickness);
    shape.rect(x - thickness, y, x, y + h);
    shape.rect(x + w, y, x + w + thickness, y + h);
}

fn draw_streak(canvas: &mut impl Canvas, offset_x: i32, offset_y: i32, streak: &Streak) {
    let mut shape = canvas
        .shape()
        .fill_color(TRANSPARENT)
        .outline_color(Color::WHITE);

    let streak_start = f(streak.progress, 0.3);
    let streak_end = f(streak.progress, 0.7);

    let x = streak.x as f32;
    let y = streak.y as f32;

    let sx = x + (STREAK_LENGTH * streak_start);
    let sy = y + (STREAK_LENGTH * streak_start);

    let ex = x + (STREAK_LENGTH * streak_end);
    let ey = y + (STREAK_LENGTH * streak_end);

    shape.line(
        offset_x + sx as i32,
        offset_y + sy as i32,
        offset_x + ex as i32,
        offset_y + ey as i32,
    );
}

fn f(x: f32, t: f32) -> f32 {
    let v = simple_bezier(vec2(0.0, 0.0), vec2(t, 1.0), vec2(1.0, 0.0), x);

    v.x
}

fn simple_bezier(a: Vec2, b: Vec2, c: Vec2, t: f32) -> Vec2 {
    let d = a + t * (b - a);
    let e = b + t * (c - b);

    d + t * (e - d)
}

fn draw_rect_highlight(time: f32, canvas: &mut impl Canvas, x: i32, y: i32, w: i32, h: i32) {
    let is_on = (time * 4.0).sin() > 0.0;

    if !is_on {
        return;
    }

    const OFFSET: i32 = 3;
    const L: i32 = 6;

    let mut shape = canvas
        .shape()
        .fill_color(TRANSPARENT)
        .outline_color(Color::WHITE);

    {
        let x = x - OFFSET;
        let y = y - OFFSET;
        shape.line(x, y, x + L, y);
        shape.line(x, y, x, y + L);
    }

    {
        let x = x + w + OFFSET;
        let y = y - OFFSET;

        shape.line(x, y, x - L, y);
        shape.line(x, y, x, y + L);
    }

    {
        let x = x - OFFSET;
        let y = y + h + OFFSET;

        shape.line(x, y, x + L, y);
        shape.line(x, y, x, y - L);
    }

    {
        let x = x + w + OFFSET;
        let y = y + h + OFFSET;

        shape.line(x, y, x - L, y);
        shape.line(x, y, x, y - L);
    }
}

fn draw_button_content(canvas: &mut impl Canvas, x: i32, y: i32, w: i32, h: i32) {
    let mut shape = canvas
        .shape()
        .fill_color(Color::BLACK)
        .outline_color(TRANSPARENT);

    let sx = x + CONTENT_OFFSET;
    let sy = y + CONTENT_OFFSET;

    let ex = x + w - CONTENT_OFFSET;
    let ey = y + h - CONTENT_OFFSET;

    shape.rect(sx, sy, ex, ey);
}
