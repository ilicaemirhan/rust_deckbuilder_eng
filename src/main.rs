use deckbuilder_eng::ui::*;
use deckbuilder_eng::ui_image::*;
use deckbuilder_eng::audio::AudioManager;
use egui::{Color32, TextStyle, Vec2};
use std::time::{Instant, Duration};

pub struct DeckbuilderApp {
    checkbox_value: bool,
    slider_value: f32,
    drag_value: i32,
    text_input: String,
    text_area: String,
    combo_selected: String,
    texture: Option<egui::TextureHandle>,
    gif_frames: Option<(Vec<egui::TextureHandle>, Vec<std::time::Duration>)>,
    gif_index: usize,
    gif_last: Instant,
    audio_manager: AudioManager,
    music_volume: f32,
    sfx_volume: f32,
    music_pitch: f32,
    gif_speed: f32, // new: playback speed for GIF
}

impl Default for DeckbuilderApp {
    fn default() -> Self {
        Self {
            checkbox_value: false,
            slider_value: 0.0,
            drag_value: 0,
            text_input: String::new(),
            text_area: String::new(),
            combo_selected: "Birinci".to_string(),
            texture: None,
            gif_frames: None,
            gif_index: 0,
            gif_last: Instant::now(),
            audio_manager: AudioManager::new().expect("Ses başlatılamadı"),
            music_volume: 0.5,
            sfx_volume: 0.5,
            music_pitch: 1.0,
            gif_speed: 1.0,
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Deckbuilder Engine - UI Demo",
        options,
        Box::new(|_cc| {
            // önce app’i oluştur
            let mut app = DeckbuilderApp::default();

            // — Sesleri yükle — 
            // 1) TrackId’leri ayrı satırlarda alın (immutable borrow hemen biter)
            let music_id = app.audio_manager.music_track().id();
            let sfx_id   = app.audio_manager.sfx_track().id();

            // 2) Ardından mutable load_sound çağrılarını yapın
            app.audio_manager
                .load_sound(
                    "assets/music/background.ogg",
                    "background",
                    music_id,
                    true,
                )
                .expect("Arka plan müziği yüklenemedi");

            app.audio_manager
                .load_sound(
                    "assets/sfx/click.wav",
                    "click",
                    sfx_id,
                    false,
                )
                .expect("Tıklama sesi yüklenemedi");

            // 3) App’i UI döngüsüne ver
            Box::new(app)
        }),
    )
    .expect("Failed to start Deckbuilder Engine");
}

impl eframe::App for DeckbuilderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui_heading(ui, "Deckbuilder Engine - Tüm UI Fonksiyonları");

                ui_separator(ui);

                ui_label(ui, "Normal etiket");
                ui_bold(ui, "Kalın metin");
                ui_colored_text(ui, "Renkli metin", Color32::from_rgb(200, 50, 50));
                ui_custom_font(ui, "Büyük font", 32.0);
                ui_text_with_style(ui, "TextStyle::Monospace", TextStyle::Monospace);

                ui_separator(ui);

                ui_button(ui, "Buton", || {
                    println!("Butona tıklandı!");
                });

                ui_sized_button(ui, "Büyük Buton", Vec2::new(200.0, 40.0), || {
                    println!("Büyük butona tıklandı!");
                });

                ui_separator(ui);

                ui_checkbox(ui, "Checkbox", &mut self.checkbox_value);
                ui_label(ui, &format!("Checkbox durumu: {}", self.checkbox_value));

                let clicked = ui_checkbox_simple(ui, "Basit Checkbox", self.checkbox_value);
                if clicked {
                    ui_label(ui, "Basit Checkbox tıklandı!");
                }

                ui_separator(ui);

                ui_slider(ui, "Slider", &mut self.slider_value, 0.0..=100.0);
                ui_label(ui, &format!("Slider değeri: {:.2}", self.slider_value));

                ui_drag_value(ui, "DragValue", &mut self.drag_value);
                ui_label(ui, &format!("DragValue: {}", self.drag_value));

                ui_separator(ui);

                ui_text_input(ui, "Text Input", &mut self.text_input);
                ui_text_area(ui, "Text Area", &mut self.text_area);

                ui_separator(ui);

                let combo_options = vec!["Birinci".to_string(), "İkinci".to_string(), "Üçüncü".to_string()];
                ui_combo_box(ui, "ComboBox", &combo_options, &mut self.combo_selected);
                ui_label(ui, &format!("Seçili: {}", self.combo_selected));

                ui_separator(ui);

                ui_grid(ui, |grid| {
                    grid.add(egui::Label::new("Grid 1"));
                    grid.add(egui::Label::new("Grid 2"));
                });

                ui_separator(ui);

                ui_scroll(ui, |inner| {
                    for i in 0..10 {
                        ui_label(inner, &format!("Scroll item {}", i));
                    }
                });

                ui_separator(ui);

                ui_columns(ui, |left, right| {
                    ui_label(left, "Sol panel");
                    ui_label(right, "Sağ panel");
                });

                ui_spacer(ui, 20.0);

                ui_label(ui, "Context örneği:");
                let _context = ui_get_context(ui);

                ui_separator(ui);

                let image_path = "assets/example.gif";

                if self.texture.is_none() {
                    self.texture = load_texture_from_path(ctx, image_path);
                }

                if let Some(texture) = &self.texture {
                    ui_label(ui, "Görsel (orijinal boyut):");
                    ui_image(ui, texture, None);

                    ui_label(ui, "Görsel (200x100):");
                    ui_image(ui, texture, Some(Vec2::new(200.0, 100.0)));

                    ui_label(ui, "Görsel (scale 0.5):");
                    ui_image_scaled(ui, texture, 0.5);

                    ui_label(ui, "Görsel buton:");
                    if ui_image_button(ui, texture, Some(Vec2::new(64.0, 64.0))) {
                        ui_label(ui, "Görsel butona tıklandı!");
                    }

                    ui_label(ui, "Görsel link:");
                    ui_image_link(ui, texture, "https://github.com/", Some(Vec2::new(64.0, 64.0)));
                } else {
                    ui_label(ui, &format!("Görsel yüklenemedi: {}", image_path));
                }

                ui_separator(ui);
                ui_label(ui, "GIF animasyon örneği:");
                ui_slider(ui, "GIF Hızı (düşük= yavaş, yüksek= hızlı)", &mut self.gif_speed, 0.1..=20.0);

                if self.gif_frames.is_none() {
                    self.gif_frames = load_gif_frames(ctx, image_path, Some(self.gif_speed));
                    self.gif_index = 0;
                    self.gif_last = Instant::now();
                }

                if let Some((frames, delays)) = &self.gif_frames {
                    if !frames.is_empty() {
                        let orig = delays[self.gif_index];
                        let adj = Duration::from_secs_f32(orig.as_secs_f32() / self.gif_speed);
                        if self.gif_last.elapsed() > adj {
                            self.gif_index = (self.gif_index + 1) % frames.len();
                            self.gif_last = Instant::now();
                        }
                        ui_image(ui, &frames[self.gif_index], None);
                    } else {
                        ui_label(ui, "GIF kareleri yüklenemedi.");
                    }
                } else {
                    ui_label(ui, "GIF yüklenemedi.");
                }

                ui_separator(ui);
                ui_label(ui, "Ses Kontrolleri:");

                ui_button(ui, "Müzik Çal", || {
                    let _ = self.audio_manager.play_music("background", 1.0);
                });
                ui_button(ui, "Müzik Durdur", || {
                    let _ = self.audio_manager.stop_music();
                });
                ui_button(ui, "Tıkla (click)", || {
                    let _ = self.audio_manager.play_sound("click", 1.0);
                });
                ui_slider(ui, "Müzik Sesi", &mut self.music_volume, 0.0..=1.0);
                let _ = self.audio_manager.set_music_volume(self.music_volume);
                ui_slider(ui, "SFX Sesi", &mut self.sfx_volume, 0.0..=1.0);
                let _ = self.audio_manager.set_sfx_volume(self.sfx_volume);

                ui_slider(ui, "Müzik Pitch", &mut self.music_pitch, 0.5..=2.0);
                let _ = self.audio_manager.set_music_pitch(self.music_pitch);
                ui_button(ui, "Pause Müzik", || {
                    let _ = self.audio_manager.pause_music();
                });
                ui.add_space(8.0);
                ui_button(ui, "Resume Müzik", || {
                    let _ = self.audio_manager.resume_music();
                });
                ui.add_space(8.0);
                ui_button(ui, "Fade Out (2s)", || {
                    let _ = self.audio_manager.fade_out_music(Duration::from_secs(2));
                });
            });
        });
    }
}