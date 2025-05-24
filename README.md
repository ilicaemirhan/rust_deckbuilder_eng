# deckbuilder_eng

A modular engine for deck-builder games, written in Rust.  
Provides reusable components for card/deck logic, game context, audio playback, and egui-based UI.

## Features

- **Card & Deck System:**  
  - Card struct, deck draw/discard mechanics, search, mill, and more.
- **Game Context:**  
  - Player/enemy health, energy, turn tracking, and extensible game logic.
- **Audio Management:**  
  - Background music and sound effects via [kira](https://github.com/tesselode/kira).
  - Volume, panning, fade, crossfade, and pitch controls.
- **UI Helpers:**  
  - Ready-to-use egui widget functions for buttons, sliders, checkboxes, combo boxes, color pickers, and more.
- **Image & GIF Support:**  
  - Load and display images and GIF animations in egui.

## Example

```rust
use deckbuilder_eng::card::*;
use deckbuilder_eng::audio::AudioManager;
use deckbuilder_eng::ui::*;
use deckbuilder_eng::ui_image::*;
use egui::{CentralPanel, Context};

fn my_ui(ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui_heading(ui, "Deckbuilder Example");
        // ... use other UI helpers ...
    });
}

fn main() {
    // Audio
    let mut audio = AudioManager::new().unwrap();
    audio.load_sound("assets/music.ogg", "bgm", audio.music_track().id(), true).unwrap();
    audio.play_music("bgm", 0.5).unwrap();

    // Cards & Deck
    let card = Card::new(1, "Strike", "Deal 6 damage", 1, CardType::Attack);
    let mut deck = Deck::new(vec![card]);
    let drawn = deck.draw();

    // Game context
    let mut ctx = GameContext::new(30, 30);
    ctx.deal_damage(5);
}
```

## Modules

- `card` – Card, deck, and game context types and logic.
- `audio` – AudioManager for music and SFX.
- `ui` – egui widget helpers.
- `ui_image` – Image and GIF helpers for egui.

## Requirements

- Rust 2021 or newer
- [egui](https://github.com/emilk/egui)
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- [kira](https://github.com/tesselode/kira)
- [image](https://github.com/image-rs/image)

## License

MIT OR Apache-2.0

## Repository

[https://github.com/ilicaemirhan/rust_game_engine](https://github.com/ilicaemirhan/rust_game_engine)
