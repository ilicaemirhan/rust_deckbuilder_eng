use egui::{Context, TextureHandle, Ui, Vec2};
use image::io::Reader as ImageReader;
use image::AnimationDecoder;
use std::time::Duration;

/// Görseli dosyadan yükler ve egui texture handle olarak döner
pub fn load_texture_from_path(ctx: &Context, path: &str) -> Option<TextureHandle> {
    // Dosyadan görseli oku
    let img = ImageReader::open(path).ok()?.decode().ok()?;
    
    // RGBA8 formata dönüştür (egui için)
    let img = img.to_rgba8();

    let size = [img.width() as usize, img.height() as usize];
    let pixels = img.into_raw();

    // Egui'nin ColorImage yapısına dönüştür
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

    // Texture'u ctx'e yükle
    Some(ctx.load_texture(path, color_image, Default::default()))
}

/// GIF animasyonunu belleğe yükler ve karelerini TextureHandle olarak döndürür.
/// Dönüş: (kareler, gecikmeler)
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

/// Texture'u UI'da gösterir, opsiyonel genişlik ve yükseklik verilebilir
pub fn ui_image(ui: &mut Ui, texture: &TextureHandle, _size: Option<Vec2>) {
    ui.image(texture);
}

/// Hem yükleme hem gösterme fonksiyonu (kısa yol)
pub fn ui_image_from_path(ui: &mut Ui, ctx: &Context, path: &str, _size: Option<Vec2>) {
    if let Some(texture) = load_texture_from_path(ctx, path) {
        ui_image(ui, &texture, None);
    } else {
        ui.label(format!("Failed to load image from: {}", path));
    }
}

/// Texture'u belirli bir oranda gösterir (scale)
pub fn ui_image_scaled(ui: &mut Ui, texture: &TextureHandle, _scale: f32) {
    ui.image(texture);
}

/// Texture'u daire şeklinde kırparak gösterir
pub fn ui_image_circle(ui: &mut Ui, texture: &TextureHandle, diameter: f32) {
    use egui::Shape;
    let pos = ui.cursor().min;
    let rect = egui::Rect::from_center_size(pos + egui::vec2(diameter/2.0, diameter/2.0), Vec2::splat(diameter));
    let _mesh = egui::Mesh::with_texture(texture.id());
    ui.painter().add(Shape::circle_filled(rect.center(), diameter/2.0, egui::Color32::WHITE));
    ui.allocate_ui_at_rect(rect, |_ui| {});
}

/// Texture'u bir buton olarak gösterir (tıklanabilir)
pub fn ui_image_button(ui: &mut Ui, texture: &TextureHandle, _size: Option<Vec2>) -> bool {
    ui.add(egui::ImageButton::new(texture)).clicked()
}

/// Texture'u bir link olarak gösterir (tıklanınca url açar)
pub fn ui_image_link(ui: &mut Ui, texture: &TextureHandle, url: &str, _size: Option<Vec2>) {
    if ui.add(egui::ImageButton::new(texture)).clicked() {
        ui.ctx().open_url(egui::OpenUrl::same_tab(url));
    }
}
