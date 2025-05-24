//! UI helpers for egui-based applications.
//!
//! Provides reusable functions for common egui widgets and layouts, such as buttons, sliders, checkboxes, combo boxes, images, and more.
//!
//! # Example
//!
//! ```rust
//! use deckbuilder_eng::ui::*;
//! use egui::{CentralPanel, Color32, TextStyle, Vec2, Context};
//!
//! fn my_ui(ctx: &Context) {
//!     let mut checkbox_value = false;
//!     let mut slider_value = 0.5;
//!     let mut drag_value = 10;
//!     let mut text_input = String::from("Hello");
//!     let mut text_area = String::from("Multiline\nText");
//!     let mut combo_selected = "First".to_string();
//!     let combo_options = vec!["First".to_string(), "Second".to_string(), "Third".to_string()];
//!     let mut color = Color32::from_rgb(128, 64, 200);
//!     let mut radio_selected = "A".to_string();
//!     let radio_options = vec!["A".to_string(), "B".to_string(), "C".to_string()];
//!     let mut popup_selected = String::from("None");
//!     let popup_options = ["Option 1", "Option 2", "Option 3"];
//!
//!     CentralPanel::default().show(ctx, |ui| {
//!         ui_heading(ui, "Deckbuilder UI Example");
//!         ui_separator(ui);
//!         ui_label(ui, "This is a label.");
//!         ui_bold(ui, "Bold text");
//!         ui_colored_text(ui, "Red text", Color32::RED);
//!         ui_custom_font(ui, "Large font", 32.0);
//!         ui_text_with_style(ui, "Monospace style", TextStyle::Monospace);
//!         ui_separator(ui);
//!
//!         ui_button(ui, "Click me", || println!("Button clicked!"));
//!         ui_sized_button(ui, "Big Button", Vec2::new(200.0, 40.0), || println!("Big button clicked!"));
//!         ui_separator(ui);
//!
//!         ui_checkbox(ui, "Checkbox", &mut checkbox_value);
//!         ui_label(ui, &format!("Checkbox value: {}", checkbox_value));
//!         let clicked = ui_checkbox_simple(ui, "Simple Checkbox", checkbox_value);
//!         if clicked {
//!             ui_label(ui, "Simple Checkbox clicked!");
//!         }
//!         ui_separator(ui);
//!
//!         ui_slider(ui, "Slider", &mut slider_value, 0.0..=100.0);
//!         ui_label(ui, &format!("Slider value: {:.2}", slider_value));
//!         ui_drag_value(ui, "DragValue", &mut drag_value);
//!         ui_label(ui, &format!("DragValue: {}", drag_value));
//!         ui_separator(ui);
//!
//!         ui_text_input(ui, "Text Input", &mut text_input);
//!         ui_text_area(ui, "Text Area", &mut text_area);
//!         ui_separator(ui);
//!
//!         ui_combo_box(ui, "ComboBox", &combo_options, &mut combo_selected);
//!         ui_label(ui, &format!("Selected: {}", combo_selected));
//!         ui_separator(ui);
//!
//!         ui_grid(ui, |grid| {
//!             grid.add(egui::Label::new("Grid 1"));
//!             grid.add(egui::Label::new("Grid 2"));
//!         });
//!         ui_separator(ui);
//!
//!         ui_scroll(ui, |inner| {
//!             for i in 0..5 {
//!                 ui_label(inner, &format!("Scroll item {}", i));
//!             }
//!         });
//!         ui_separator(ui);
//!
//!         ui_columns(ui, |left, right| {
//!             ui_label(left, "Left panel");
//!             ui_label(right, "Right panel");
//!         });
//!         ui_spacer(ui, 20.0);
//!         ui_separator(ui);
//!
//!         let _context = ui_get_context(ui);
//!
//!         ui_text_with_style(ui, "Styled text", TextStyle::Button);
//!         ui_separator(ui);
//!
//!         ui_radio_buttons(ui, "Radio Group", &radio_options, &mut radio_selected);
//!         ui_label(ui, &format!("Radio selected: {}", radio_selected));
//!         ui_separator(ui);
//!
//!         ui_color_picker(ui, "Color Picker", &mut color);
//!         ui_label(ui, &format!("Color: #{:02X}{:02X}{:02X}", color.r(), color.g(), color.b()));
//!         ui_separator(ui);
//!
//!         ui_progress_bar(ui, 0.42, Some("Progress 42%"));
//!         ui_separator(ui);
//!
//!         ui_tooltip(ui, "This is a tooltip!");
//!         ui_separator(ui);
//!
//!         ui_popup_menu(ui, "Popup Menu", &popup_options, &mut popup_selected);
//!         ui_label(ui, &format!("Popup selected: {}", popup_selected));
//!     });
//! }
//! ```
//!
//! # Details
//!
//! - All functions are designed to be ergonomic and composable with egui's `Ui`.
//! - Some helpers (like `ui_grid`, `ui_columns`) use unique IDs to avoid layout conflicts.
//! - See each function's documentation for usage and customization options.

use egui::{
    Button, Color32, ComboBox, Context, DragValue, FontId, Grid, RichText, ScrollArea,
    Separator, Slider, TextEdit, TextStyle, Ui, Vec2,
};

/// Heading (large text)
///
/// # Example
/// ```rust
/// # use egui::{Ui};
/// # use deckbuilder_eng::ui::ui_heading;
/// # fn demo(ui: &mut Ui) {
/// ui_heading(ui, "My Heading");
/// # }
/// ```
pub fn ui_heading(ui: &mut Ui, text: &str) {
    ui.heading(text);
}

/// Label
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_label;
/// # fn demo(ui: &mut Ui) {
/// ui_label(ui, "Label text");
/// # }
/// ```
pub fn ui_label(ui: &mut Ui, text: &str) {
    ui.label(text);
}

/// Bold text
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_bold;
/// # fn demo(ui: &mut Ui) {
/// ui_bold(ui, "Bold text");
/// # }
/// ```
pub fn ui_bold(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).strong());
}

/// Colored text
///
/// # Example
/// ```rust
/// # use egui::{Ui, Color32};
/// # use deckbuilder_eng::ui::ui_colored_text;
/// # fn demo(ui: &mut Ui) {
/// ui_colored_text(ui, "Red text", Color32::RED);
/// # }
/// ```
pub fn ui_colored_text(ui: &mut Ui, text: &str, color: Color32) {
    ui.label(RichText::new(text).color(color));
}

/// Custom font text (e.g. large size)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_custom_font;
/// # fn demo(ui: &mut Ui) {
/// ui_custom_font(ui, "Large font", 32.0);
/// # }
/// ```
pub fn ui_custom_font(ui: &mut Ui, text: &str, size: f32) {
    use egui::FontFamily;
    let font = RichText::new(text).font(FontId::new(size, FontFamily::Proportional));
    ui.label(font);
}

/// Button (runs callback when clicked)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_button;
/// # fn demo(ui: &mut Ui) {
/// ui_button(ui, "Click me", || println!("Button clicked!"));
/// # }
/// ```
pub fn ui_button<F: FnOnce()>(ui: &mut Ui, label: &str, on_click: F) {
    if ui.add(Button::new(label)).clicked() {
        on_click();
    }
}

/// Checkbox / Toggle
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_checkbox;
/// # fn demo(ui: &mut Ui) {
/// let mut checked = false;
/// ui_checkbox(ui, "Check me", &mut checked);
/// # }
/// ```
pub fn ui_checkbox(ui: &mut Ui, label: &str, checked: &mut bool) {
    ui.checkbox(checked, label);
}

/// Slider (f32)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_slider;
/// # fn demo(ui: &mut Ui) {
/// let mut value = 0.5;
/// ui_slider(ui, "Value", &mut value, 0.0..=1.0);
/// # }
/// ```
pub fn ui_slider(ui: &mut Ui, label: &str, value: &mut f32, range: std::ops::RangeInclusive<f32>) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(Slider::new(value, range));
    });
}

/// DragValue (increment/decrement number)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_drag_value;
/// # fn demo(ui: &mut Ui) {
/// let mut value = 10;
/// ui_drag_value(ui, "Drag", &mut value);
/// # }
/// ```
pub fn ui_drag_value(ui: &mut Ui, label: &str, value: &mut i32) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(DragValue::new(value));
    });
}

/// Text input (single line)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_text_input;
/// # fn demo(ui: &mut Ui) {
/// let mut text = String::from("Hello");
/// ui_text_input(ui, "Input", &mut text);
/// # }
/// ```
pub fn ui_text_input(ui: &mut Ui, label: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value);
    });
}

/// TextArea (multi-line)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_text_area;
/// # fn demo(ui: &mut Ui) {
/// let mut text = String::from("Multiline\nText");
/// ui_text_area(ui, "Text Area", &mut text);
/// # }
/// ```
pub fn ui_text_area(ui: &mut Ui, label: &str, value: &mut String) {
    ui.label(label);
    ui.add(TextEdit::multiline(value).desired_rows(5));
}

/// Separator line
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_separator;
/// # fn demo(ui: &mut Ui) {
/// ui_separator(ui);
/// # }
/// ```
pub fn ui_separator(ui: &mut Ui) {
    ui.add(Separator::default());
}

/// ComboBox (dropdown)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_combo_box;
/// # fn demo(ui: &mut Ui) {
/// let options = vec!["A".to_string(), "B".to_string()];
/// let mut selected = options[0].clone();
/// ui_combo_box(ui, "Combo", &options, &mut selected);
/// # }
/// ```
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

/// Grid (table view)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_grid;
/// # fn demo(ui: &mut Ui) {
/// ui_grid(ui, |grid| {
///     grid.label("Cell 1");
///     grid.label("Cell 2");
/// });
/// # }
/// ```
pub fn ui_grid<F: FnOnce(&mut Ui)>(ui: &mut Ui, f: F) {
    // Give a different id for each call (e.g. random, non-static string or via parameter)
    use std::sync::atomic::{AtomicUsize, Ordering};
    static GRID_ID: AtomicUsize = AtomicUsize::new(0);
    let id = GRID_ID.fetch_add(1, Ordering::Relaxed);
    let grid = Grid::new(format!("custom_grid_{}", id));
    grid.show(ui, |ui_grid| {
        f(ui_grid);
    });
}

/// Scrollable vertical area
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_scroll;
/// # fn demo(ui: &mut Ui) {
/// ui_scroll(ui, |inner| {
///     for i in 0..3 {
///         inner.label(format!("Item {}", i));
///     }
/// });
/// # }
/// ```
pub fn ui_scroll<F: FnOnce(&mut Ui)>(ui: &mut Ui, inner: F) {
    ScrollArea::vertical().show(ui, |inner_ui| {
        inner(inner_ui);
    });
}

/// Two-column layout
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_columns;
/// # fn demo(ui: &mut Ui) {
/// ui_columns(ui, |left, right| {
///     left.label("Left");
///     right.label("Right");
/// });
/// # }
/// ```
pub fn ui_columns<F: FnOnce(&mut Ui, &mut Ui)>(ui: &mut Ui, content: F) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COL_ID: AtomicUsize = AtomicUsize::new(0);
    let id = COL_ID.fetch_add(1, Ordering::Relaxed);
    ui.columns(2, |columns| {
        let (left, right) = columns.split_at_mut(1);
        // Even if we don't use id, it ensures unique call for columns
        let _ = id;
        content(&mut left[0], &mut right[0]);
    });
}

/// Spacer (vertical space)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_spacer;
/// # fn demo(ui: &mut Ui) {
/// ui_spacer(ui, 16.0);
/// # }
/// ```
pub fn ui_spacer(ui: &mut Ui, height: f32) {
    ui.add_space(height);
}

/// Sized button
///
/// # Example
/// ```rust
/// # use egui::{Ui, Vec2};
/// # use deckbuilder_eng::ui::ui_sized_button;
/// # fn demo(ui: &mut Ui) {
/// ui_sized_button(ui, "Big", Vec2::new(200.0, 40.0), || println!("Big button!"));
/// # }
/// ```
pub fn ui_sized_button<F: FnOnce()>(ui: &mut Ui, label: &str, size: Vec2, on_click: F) {
    if ui
        .add_sized(size, Button::new(label))
        .clicked()
    {
        on_click();
    }
}

/// Simple checkbox (returns true if clicked)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_checkbox_simple;
/// # fn demo(ui: &mut Ui) {
/// let checked = false;
/// let clicked = ui_checkbox_simple(ui, "Simple", checked);
/// # }
/// ```
pub fn ui_checkbox_simple(ui: &mut Ui, label: &str, checked: bool) -> bool {
    ui.checkbox(&mut checked.clone(), label).clicked()
}

/// Returns the context (e.g. theme, clipboard, etc.)
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_get_context;
/// # fn demo(ui: &Ui) {
/// let ctx = ui_get_context(ui);
/// # }
/// ```
pub fn ui_get_context(ui: &Ui) -> &Context {
    ui.ctx()
}

/// Show text with a specific TextStyle
///
/// # Example
/// ```rust
/// # use egui::{Ui, TextStyle};
/// # use deckbuilder_eng::ui::ui_text_with_style;
/// # fn demo(ui: &mut Ui) {
/// ui_text_with_style(ui, "Styled", TextStyle::Button);
/// # }
/// ```
pub fn ui_text_with_style(ui: &mut Ui, text: &str, style: TextStyle) {
    ui.label(RichText::new(text).text_style(style));
}

/// Radio button group under a label
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_radio_buttons;
/// # fn demo(ui: &mut Ui) {
/// let options = vec!["A".to_string(), "B".to_string()];
/// let mut selected = options[0].clone();
/// ui_radio_buttons(ui, "Radio", &options, &mut selected);
/// # }
/// ```
pub fn ui_radio_buttons<T: ToString + PartialEq + Clone>(
    ui: &mut Ui,
    label: &str,
    options: &[T],
    selected: &mut T,
) {
    ui.vertical(|ui| {
        ui.label(label);
        for opt in options {
            let is_selected = *selected == *opt;
            if ui.radio(is_selected, opt.to_string()).clicked() {
                *selected = opt.clone();
            }
        }
    });
}

/// RGBA color-picker using four sliders
///
/// # Example
/// ```rust
/// # use egui::{Ui, Color32};
/// # use deckbuilder_eng::ui::ui_color_picker;
/// # fn demo(ui: &mut Ui) {
/// let mut color = Color32::from_rgb(128, 64, 200);
/// ui_color_picker(ui, "Color", &mut color);
/// # }
/// ```
pub fn ui_color_picker(ui: &mut Ui, label: &str, color: &mut Color32) {
    ui.label(label);
    let mut r = color.r() as f32;
    let mut g = color.g() as f32;
    let mut b = color.b() as f32;
    let mut a = color.a() as f32;

    ui.horizontal(|ui| {
        ui.label("R:"); ui.add(Slider::new(&mut r, 0.0..=255.0));
        ui.label("G:"); ui.add(Slider::new(&mut g, 0.0..=255.0));
    });
    ui.horizontal(|ui| {
        ui.label("B:"); ui.add(Slider::new(&mut b, 0.0..=255.0));
        ui.label("A:"); ui.add(Slider::new(&mut a, 0.0..=255.0));
    });
    *color = Color32::from_rgba_unmultiplied(r as u8, g as u8, b as u8, a as u8);
}

/// Progress bar with optional centered text
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_progress_bar;
/// # fn demo(ui: &mut Ui) {
/// ui_progress_bar(ui, 0.5, Some("Halfway"));
/// # }
/// ```
pub fn ui_progress_bar(ui: &mut Ui, fraction: f32, text: Option<&str>) {
    let bar = egui::ProgressBar::new(fraction).text(text.unwrap_or(""));
    ui.add(bar);
}

/// Simple info tooltip marker
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_tooltip;
/// # fn demo(ui: &mut Ui) {
/// ui_tooltip(ui, "This is a tooltip!");
/// # }
/// ```
pub fn ui_tooltip(ui: &mut Ui, hover_text: &str) {
    ui.label("ⓘ").on_hover_text(hover_text);
}

/// Popup menu helper via a menu button
///
/// # Example
/// ```rust
/// # use egui::Ui;
/// # use deckbuilder_eng::ui::ui_popup_menu;
/// # fn demo(ui: &mut Ui) {
/// let mut selected = String::from("None");
/// let options = ["Option 1", "Option 2"];
/// ui_popup_menu(ui, "Menu", &options, &mut selected);
/// # }
/// ```
pub fn ui_popup_menu(ui: &mut Ui, button_label: &str, options: &[&str], selected: &mut String) {
    ui.menu_button(button_label, |ui| {
        for &opt in options {
            if ui.button(opt).clicked() {
                *selected = opt.to_string();
                ui.close_menu();
            }
        }
    });
}
