use egui::{
    Button, Color32, ComboBox, Context, DragValue, FontId, Grid, RichText, ScrollArea,
    Separator, Slider, TextEdit, TextStyle, Ui, Vec2,
};

/// Başlık (büyük metin)
pub fn ui_heading(ui: &mut Ui, text: &str) {
    ui.heading(text);
}

/// Etiket
pub fn ui_label(ui: &mut Ui, text: &str) {
    ui.label(text);
}

/// Vurgulu metin (bold)
pub fn ui_bold(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).strong());
}

/// Renkli metin
pub fn ui_colored_text(ui: &mut Ui, text: &str, color: Color32) {
    ui.label(RichText::new(text).color(color));
}

/// Özel font ile metin (örneğin büyük boy)
pub fn ui_custom_font(ui: &mut Ui, text: &str, size: f32) {
    use egui::FontFamily;
    let font = RichText::new(text).font(FontId::new(size, FontFamily::Proportional));
    ui.label(font);
}

/// Buton (tıklanınca callback çalıştırır)
pub fn ui_button<F: FnOnce()>(ui: &mut Ui, label: &str, on_click: F) {
    if ui.add(Button::new(label)).clicked() {
        on_click();
    }
}

/// Checkbox / Toggle
pub fn ui_checkbox(ui: &mut Ui, label: &str, checked: &mut bool) {
    ui.checkbox(checked, label);
}

/// Slider (f32)
pub fn ui_slider(ui: &mut Ui, label: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(Slider::new(value, range));
    });
}

/// DragValue (sayı arttırma/azaltma)
pub fn ui_drag_value(ui: &mut Ui, label: &str, value: &mut i32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(DragValue::new(value));
    });
}

/// Text input (tek satır)
pub fn ui_text_input(ui: &mut Ui, label: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value);
    });
}

/// TextArea (çok satır)
pub fn ui_text_area(ui: &mut Ui, label: &str, value: &mut String) {
    ui.label(label);
    ui.add(TextEdit::multiline(value).desired_rows(5));
}

/// Ayırıcı çizgi
pub fn ui_separator(ui: &mut Ui) {
    ui.add(Separator::default());
}

/// ComboBox (dropdown)
pub fn ui_combo_box<T: ToString + PartialEq + Clone>(
    ui: &mut Ui,
    label: &str,
    options: &[T],
    selected: &mut T,
) {
    ComboBox::from_label(label).show_ui(ui, |ui| {
        for option in options {
            ui.selectable_value(selected, option.clone(), option.to_string());
        }
    });
}

/// Grid (tablo görünümü)
pub fn ui_grid<F: FnOnce(&mut Ui)>(ui: &mut Ui, f: F) {
    // Her çağrıda farklı id verin (ör: rastgele, sabit olmayan bir string veya parametre ile)
    use std::sync::atomic::{AtomicUsize, Ordering};
    static GRID_ID: AtomicUsize = AtomicUsize::new(0);
    let id = GRID_ID.fetch_add(1, Ordering::Relaxed);
    let grid = Grid::new(format!("custom_grid_{}", id));
    grid.show(ui, |ui_grid| {
        f(ui_grid);
    });
}

/// Scrollable dikey alan
pub fn ui_scroll<F: FnOnce(&mut Ui)>(ui: &mut Ui, inner: F) {
    ScrollArea::vertical().show(ui, |inner_ui| {
        inner(inner_ui);
    });
}

/// Kolonlu görünüm (2 sütun)
pub fn ui_columns<F: FnOnce(&mut Ui, &mut Ui)>(ui: &mut Ui, content: F) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COL_ID: AtomicUsize = AtomicUsize::new(0);
    let id = COL_ID.fetch_add(1, Ordering::Relaxed);
    ui.columns(2, |columns| {
        let (left, right) = columns.split_at_mut(1);
        // id'yi kullanmasak da columns fonksiyonu için benzersiz çağrı sağlanmış olur
        let _ = id;
        content(&mut left[0], &mut right[0]);
    });
}

/// Boşluk (spacer)
pub fn ui_spacer(ui: &mut Ui, height: f32) {
    ui.add_space(height);
}

/// Özel boyutlu buton
pub fn ui_sized_button<F: FnOnce()>(ui: &mut Ui, label: &str, size: Vec2, on_click: F) {
    if ui
        .add_sized(size, Button::new(label))
        .clicked()
    {
        on_click();
    }
}


/// Checkbox (işaretli mi, değil mi)
pub fn ui_checkbox_simple(ui: &mut Ui, label: &str, checked: bool) -> bool {
    ui.checkbox(&mut checked.clone(), label).clicked()
}

/// Context bilgisini döndür (örnek: tema, clipboard vs.)
pub fn ui_get_context(ui: &Ui) -> &Context {
    ui.ctx()
}

/// Belirli bir TextStyle ile metin göster
pub fn ui_text_with_style(ui: &mut Ui, text: &str, style: TextStyle) {
    ui.label(RichText::new(text).text_style(style));
}
