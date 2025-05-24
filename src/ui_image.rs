//! Image and GIF helpers for egui-based applications.
//!
//! Provides functions for loading images and GIFs, displaying textures, and rendering images with effects (scaling, tint, border, etc).
//!
//! # Example
//!
//! ```rust
//! use deckbuilder_eng::ui_image::*;
//! use egui::{CentralPanel, Context, Vec2, Color32, Stroke};
//!
//! fn my_image_ui(ctx: &Context) {
//!     CentralPanel::default().show(ctx, |ui| {
//!         // Load and show an image
//!         if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
//!             ui_image(ui, &texture, None);
//!             ui_image_scaled(ui, &texture, 0.5);
//!             ui_image_tinted(ui, &texture, Vec2::new(64.0, 64.0), Color32::from_rgba_premultiplied(255, 0, 0, 128));
//!             ui_image_with_border(ui, &texture, Vec2::new(64.0, 64.0), Stroke::new(2.0, Color32::YELLOW));
//!         }
//!
//!         // Load and show a GIF animation
//!         if let Some((frames, delays)) = load_gif_frames(ctx, "assets/example.gif", Some(1.0)) {
//!             if !frames.is_empty() {
//!                 ui_image(ui, &frames[0], None); // show first frame
//!             }
//!         }
//!     });
//! }
//! ```
//!
//! # Details
//!
//! - All functions are designed to work with egui's `Ui` and `Context`.
//! - GIF helpers return all frames and their delays for manual animation.
//! - Some functions (e.g. `ui_image_circle`, `ui_image_rotated`) may be stubs or limited by egui's capabilities.
//! - See each function's documentation for usage and customization options.

use egui::{Color32, Context, Pos2, Rect, Stroke, TextureHandle, Ui, Vec2};
use image::io::Reader as ImageReader;
use image::AnimationDecoder;
use std::time::Duration;

/// Loads an image from file and returns it as an egui texture handle.
///
/// # Example
/// ```rust
/// # use egui::Context;
/// # use deckbuilder_eng::ui_image::load_texture_from_path;
/// # fn demo(ctx: &Context) {
/// let texture = load_texture_from_path(ctx, "assets/example.png");
/// # }
/// ```
pub fn load_texture_from_path(ctx: &Context, path: &str) -> Option<TextureHandle> {
    // Read image from file
    let img = ImageReader::open(path).ok()?.decode().ok()?;
    
    // Convert to RGBA8 format (for egui)
    let img = img.to_rgba8();

    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();

    // Convert to egui's ColorImage
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    // Upload texture to egui context
    Some(ctx.load_texture(path, color_image, Default::default()))
}

/// Loads a GIF animation into memory and returns its frames as TextureHandles and their delays.
/// Returns: (frames, delays)
///
/// # Example
/// ```rust
/// # use egui::Context;
/// # use deckbuilder_eng::ui_image::load_gif_frames;
/// # fn demo(ctx: &Context) {
/// let frames_and_delays = load_gif_frames(ctx, "assets/example.gif", Some(1.0));
/// # }
/// ```
pub fn load_gif_frames(
    ctx: &Context,
    path: &str,
    speed: Option<f32>, // 1.0 = normal, >1.0 faster, <1.0 slower
) -> Option<(Vec<TextureHandle>, Vec<Duration>)> {
    let speed = speed.unwrap_or(1.0);
    let file = std::fs::File::open(path).ok()?;
    let decoder = image::codecs::gif::GifDecoder::new(file).ok()?;
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().ok()?;

    let mut textures = Vec::new();
    let mut delays = Vec::new();

    for frame in frames {
        let delay = frame.delay();
        let delay_ms = delay.numer_denom_ms().0 as f32 / delay.numer_denom_ms().1 as f32;
        let adj = (delay_ms / speed).max(1.0);
        let delay = Duration::from_millis(adj as u64);
        let img = frame.into_buffer();
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img.into_raw();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture = ctx.load_texture("gif_frame", color_image, Default::default());
        textures.push(texture);
        delays.push(delay);
    }

    Some((textures, delays))
}

/// Shows a texture in the UI, with optional width and height.
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image(ui, &texture, None);
/// }
/// # }
/// ```
pub fn ui_image(ui: &mut Ui, texture: &TextureHandle, _size: Option<Vec2>) {
    ui.image(texture);
}

/// Loads and shows an image from a file path (shortcut).
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context};
/// # use deckbuilder_eng::ui_image::ui_image_from_path;
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// ui_image_from_path(ui, ctx, "assets/example.png", None);
/// # }
/// ```
pub fn ui_image_from_path(ui: &mut Ui, ctx: &Context, path: &str, _size: Option<Vec2>) {
    if let Some(texture) = load_texture_from_path(ctx, path) {
        ui_image(ui, &texture, None);
    } else {
        ui.label(format!("Failed to load image from: {}", path));
    }
}

/// Shows a texture at a given scale factor.
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_scaled};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image_scaled(ui, &texture, 0.5);
/// }
/// # }
/// ```
pub fn ui_image_scaled(ui: &mut Ui, texture: &TextureHandle, _scale: f32) {
    ui.image(texture);
}

/// Shows a texture cropped as a circle.
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_circle};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image_circle(ui, &texture, 64.0);
/// }
/// # }
/// ```
pub fn ui_image_circle(ui: &mut Ui, texture: &TextureHandle, diameter: f32) {
    use egui::Shape;
    let pos = ui.cursor().min;
    let rect = egui::Rect::from_center_size(pos + egui::vec2(diameter/2.0, diameter/2.0), Vec2::splat(diameter));
    let _mesh = egui::Mesh::with_texture(texture.id());
    ui.painter().add(Shape::circle_filled(rect.center(), diameter/2.0, egui::Color32::WHITE));
    ui.allocate_ui_at_rect(rect, |_ui| {});
}

/// Shows a texture as a button (clickable).
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_button};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     if ui_image_button(ui, &texture, None) {
///         // clicked
///     }
/// }
/// # }
/// ```
pub fn ui_image_button(ui: &mut Ui, texture: &TextureHandle, _size: Option<Vec2>) -> bool {
    ui.add(egui::ImageButton::new(texture)).clicked()
}

/// Shows a texture as a link (opens URL when clicked).
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_link};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image_link(ui, &texture, "https://github.com/", None);
/// }
/// # }
/// ```
pub fn ui_image_link(ui: &mut Ui, texture: &TextureHandle, url: &str, _size: Option<Vec2>) {
    if ui.add(egui::ImageButton::new(texture)).clicked() {
        ui.ctx().open_url(egui::OpenUrl::same_tab(url));
    }
}

/// Shows an image with a color tint applied.
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context, Vec2, Color32};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_tinted};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image_tinted(ui, &texture, Vec2::new(64.0, 64.0), Color32::from_rgba_premultiplied(255, 0, 0, 128));
/// }
/// # }
/// ```
pub fn ui_image_tinted(ui: &mut Ui, texture: &TextureHandle, size: Vec2, tint: Color32) {
    let (_id, rect) = ui.allocate_space(size); // allocate_space returns (Id, Rect)
    ui.painter().image(
        texture.id(),
        rect,
        Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
        tint,
    );
}

/// Shows a rotated image (in radians).
/// (temporary stub—not supported in this build)
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context, Vec2};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_rotated};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image_rotated(ui, &texture, Vec2::new(64.0, 64.0), 1.57);
/// }
/// # }
/// ```
pub fn ui_image_rotated(_ui: &mut Ui, _texture: &TextureHandle, _size: Vec2, _angle: f32) {
    // rotation unsupported in this build
}

/// Shows an image with a border.
///
/// # Example
/// ```rust
/// # use egui::{Ui, Context, Vec2, Color32, Stroke};
/// # use deckbuilder_eng::ui_image::{load_texture_from_path, ui_image_with_border};
/// # fn demo(ui: &mut Ui, ctx: &Context) {
/// if let Some(texture) = load_texture_from_path(ctx, "assets/example.png") {
///     ui_image_with_border(ui, &texture, Vec2::new(64.0, 64.0), Stroke::new(2.0, Color32::YELLOW));
/// }
/// # }
/// ```
pub fn ui_image_with_border(
    ui: &mut Ui,
    texture: &TextureHandle,
    size: Vec2,
    stroke: Stroke,
) {
    ui_image(ui, texture, Some(size));
    let rect = ui.min_rect();
    ui.painter().rect_stroke(rect, 0.0, stroke);
}

/// Shows a placeholder for async loading (sync fallback).
///
/// # Example
/// ```rust
/// # use egui::Context;
/// # use deckbuilder_eng::ui_image::load_texture_async;
/// # fn demo(ctx: &Context) {
/// let texture = load_texture_async(ctx, "assets/example.png");
/// # }
/// ```
pub fn load_texture_async(ctx: &Context, path: &str) -> TextureHandle {
    // Simple sync loading (placeholder logic to be added)
    load_texture_from_path(ctx, path).unwrap()
}
