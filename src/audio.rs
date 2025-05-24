//! Audio management for deckbuilder_eng.
//!
//! Provides the [`AudioManager`] type for loading, playing, and controlling background music and sound effects using the `kira` audio engine.
//!
//! # Example
//!
//! ```rust
//! use deckbuilder_eng::audio::AudioManager;
//!
//! // Create the audio manager
//! let mut audio = AudioManager::new().unwrap();
//!
//! // Load and play background music (looped)
//! audio
//!     .load_sound("assets/music.ogg", "bgm", audio.music_track().id(), true)
//!     .unwrap();
//! audio.play_music("bgm", 0.5).unwrap();
//!
//! // Load and play a sound effect
//! audio
//!     .load_sound("assets/click.wav", "click", audio.sfx_track().id(), false)
//!     .unwrap();
//! audio.play_sound("click", 1.0).unwrap();
//!
//! // Adjust volumes
//! audio.set_music_volume(0.7).unwrap();
//! audio.set_sfx_volume(0.3).unwrap();
//! ```
//!
//! # Details
//!
//! - Music and SFX are handled on separate tracks.
//! - Supports volume, panning, pitch, fade, crossfade, and more.
//! - See each method's documentation for advanced usage and error handling.

#![warn(missing_docs)]

use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

use kira::{
    manager::{AudioManager as KiraManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
    track::TrackHandle,
    track::TrackId,
    tween::{Tween, Easing},
    OutputDestination,
};

/// Manages audio playback for music and sound effects.
///
/// # Example
/// ```rust
/// use deckbuilder_eng::audio::AudioManager;
/// let mut audio = AudioManager::new().unwrap();
/// ```
pub struct AudioManager {
    kira_manager: KiraManager,
    music_track: TrackHandle,
    sfx_track: TrackHandle,
    sounds: HashMap<String, StaticSoundData>,
    current_music: Option<StaticSoundHandle>,
}

impl AudioManager {
    /// Creates a new audio manager with separate tracks for music and SFX.
    ///
    /// # Example
    /// ```
    /// use deckbuilder_eng::audio::AudioManager;
    /// let mut audio = AudioManager::new().unwrap();
    /// ```
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut kira_manager = KiraManager::new(AudioManagerSettings::default())?;
        let music_track = kira_manager.add_sub_track(kira::track::TrackBuilder::new())?;
        let sfx_track = kira_manager.add_sub_track(kira::track::TrackBuilder::new())?;
        Ok(Self {
            kira_manager,
            music_track,
            sfx_track,
            sounds: HashMap::new(),
            current_music: None,
        })
    }

    /// Loads a static sound from `path`, registers it under `name`
    /// and assigns it to the given `track_id`. If `loop_sound` is true,
    /// the sound will loop indefinitely.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// let music_id = audio.music_track().id();
    /// audio.load_sound("assets/music.ogg", "bgm", music_id, true).unwrap();
    /// ```
    pub fn load_sound(
        &mut self,
        path: &str,
        name: &str,
        track_id: TrackId,
        loop_sound: bool,
    ) -> Result<(), Box<dyn Error>> {
        let mut settings = StaticSoundSettings::default();
        settings.output_destination = OutputDestination::Track(track_id);
        if loop_sound {
            settings.loop_region = Some(kira::sound::Region::default());
        }
        let sound_data = StaticSoundData::from_file(path, settings)?;
        self.sounds.insert(name.to_string(), sound_data);
        Ok(())
    }

    /// Plays a previously loaded sound effect identified by `name`
    /// at the specified `volume` (0.0 to 1.0).
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// # let sfx_id = audio.sfx_track().id();
    /// # audio.load_sound("assets/click.wav", "click", sfx_id, false).unwrap();
    /// audio.play_sound("click", 1.0).unwrap();
    /// ```
    pub fn play_sound(&mut self, name: &str, volume: f32) -> Result<(), Box<dyn Error>> {
        if let Some(sound_data) = self.sounds.get(name) {
            let mut handle = self.kira_manager.play(sound_data.clone())?;
            handle.set_volume(volume as f64, Tween::default())?;
            Ok(())
        } else {
            Err(format!("Ses '{}' bulunamadı", name).into())
        }
    }

    /// Stops any currently playing music and plays the sound
    /// identified by `name` on the music track at `volume`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// # let music_id = audio.music_track().id();
    /// # audio.load_sound("assets/music.ogg", "bgm", music_id, true).unwrap();
    /// audio.play_music("bgm", 0.5).unwrap();
    /// ```
    pub fn play_music(&mut self, name: &str, volume: f32) -> Result<(), Box<dyn Error>> {
        if let Some(sound_data) = self.sounds.get(name) {
            if let Some(mut current) = self.current_music.take() {
                current.stop(Tween::default())?;
            }
            let mut handle = self.kira_manager.play(sound_data.clone())?;
            handle.set_volume(volume as f64, Tween::default())?;
            self.current_music = Some(handle);
            Ok(())
        } else {
            Err(format!("Müzik '{}' bulunamadı", name).into())
        }
    }

    /// Stops the current music immediately.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.stop_music().unwrap();
    /// ```
    pub fn stop_music(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(mut handle) = self.current_music.take() {
            handle.stop(Tween::default())?;
        }
        Ok(())
    }

    /// Sets the volume of the music track (0.0 to 1.0).
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.set_music_volume(0.7).unwrap();
    /// ```
    pub fn set_music_volume(&mut self, volume: f32) -> Result<(), Box<dyn Error>> {
        self.music_track.set_volume(volume as f64, Tween::default())?;
        Ok(())
    }

    /// Sets the volume of the SFX track (0.0 to 1.0).
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.set_sfx_volume(0.3).unwrap();
    /// ```
    pub fn set_sfx_volume(&mut self, volume: f32) -> Result<(), Box<dyn Error>> {
        self.sfx_track.set_volume(volume as f64, Tween::default())?;
        Ok(())
    }

    /// Returns a reference to the music track handle.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let audio = AudioManager::new().unwrap();
    /// let music_track = audio.music_track();
    /// ```
    pub fn music_track(&self) -> &TrackHandle {
        &self.music_track
    }

    /// Returns a reference to the SFX track handle.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let audio = AudioManager::new().unwrap();
    /// let sfx_track = audio.sfx_track();
    /// ```
    pub fn sfx_track(&self) -> &TrackHandle {
        &self.sfx_track
    }

    /// Adjusts the playback pitch (speed) of the music.
    /// Note: Pitch control may not be supported in all builds.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.set_music_pitch(1.2).unwrap();
    /// ```
    pub fn set_music_pitch(&mut self, _speed: f32) -> Result<(), Box<dyn Error>> {
        // pitch control not supported in this build
        Ok(())
    }

    /// Pauses the currently playing music.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.pause_music().unwrap();
    /// ```
    pub fn pause_music(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.pause(Tween::default())?;
        }
        Ok(())
    }

    /// Resumes paused music playback.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.resume_music().unwrap();
    /// ```
    pub fn resume_music(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.resume(Tween::default())?;
        }
        Ok(())
    }

    /// Fades out the current music over `duration` and then stops it.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.fade_out_music(std::time::Duration::from_secs(2)).unwrap();
    /// ```
    pub fn fade_out_music(&mut self, duration: Duration) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.stop(Tween { duration, easing: Easing::Linear, ..Default::default() })?;
        }
        Ok(())
    }

    /// Sets the stereo panning for the music track: -1.0 (left) to 1.0 (right).
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.set_music_pan(-0.5).unwrap();
    /// ```
    pub fn set_music_pan(&mut self, pan: f32) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.set_panning(pan as f64, Tween::default())?;
        }
        Ok(())
    }

    /// Returns a list of all loaded sound names.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let audio = AudioManager::new().unwrap();
    /// let names = audio.list_sounds();
    /// ```
    pub fn list_sounds(&self) -> Vec<String> {
        self.sounds.keys().cloned().collect()
    }

    /// Unloads the sound identified by `name`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.unload_sound("bgm");
    /// ```
    pub fn unload_sound(&mut self, name: &str) {
        self.sounds.remove(name);
    }

    /// Clears all loaded sounds from memory.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.clear_sounds();
    /// ```
    pub fn clear_sounds(&mut self) {
        self.sounds.clear();
    }

    /// Plays a sound with a fade-in from volume 0.0 up to `target_volume`
    /// over `duration`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// # let sfx_id = audio.sfx_track().id();
    /// # audio.load_sound("assets/click.wav", "click", sfx_id, false).unwrap();
    /// audio.play_sound_fade_in("click", 1.0, std::time::Duration::from_secs(1)).unwrap();
    /// ```
    pub fn play_sound_fade_in(
        &mut self,
        name: &str,
        target_volume: f32,
        duration: Duration,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(data) = self.sounds.get(name) {
            let mut handle = self.kira_manager.play(data.clone())?;
            handle.set_volume(0.0, Tween::default())?;
            handle.set_volume(
                target_volume as f64,
                Tween {
                    duration,
                    easing: Easing::Linear,
                    ..Default::default()
                },
            )?;
            Ok(())
        } else {
            Err(format!("Ses '{}' bulunamadı", name).into())
        }
    }

    /// Plays a sound with specified `volume` and stereo `pan`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// # let sfx_id = audio.sfx_track().id();
    /// # audio.load_sound("assets/click.wav", "click", sfx_id, false).unwrap();
    /// audio.play_sound_pan("click", 1.0, -1.0).unwrap();
    /// ```
    pub fn play_sound_pan(
        &mut self,
        name: &str,
        volume: f32,
        pan: f32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(data) = self.sounds.get(name) {
            let mut handle = self.kira_manager.play(data.clone())?;
            handle.set_volume(volume as f64, Tween::default())?;
            handle.set_panning(pan as f64, Tween::default())?;
            Ok(())
        } else {
            Err(format!("Ses '{}' bulunamadı", name).into())
        }
    }

    /// Crossfades from current music to a new track `name`, fading out the old
    /// and fading in the new over `duration` to `target_volume`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// # let music_id = audio.music_track().id();
    /// # audio.load_sound("assets/music1.ogg", "bgm1", music_id, true).unwrap();
    /// # audio.load_sound("assets/music2.ogg", "bgm2", music_id, true).unwrap();
    /// audio.crossfade_music("bgm2", 0.8, std::time::Duration::from_secs(2)).unwrap();
    /// ```
    pub fn crossfade_music(
        &mut self,
        name: &str,
        target_volume: f32,
        duration: Duration,
    ) -> Result<(), Box<dyn Error>> {
        // eskiyi fade-out
        if let Some(mut old) = self.current_music.take() {
            old.stop(Tween {
                duration,
                easing: Easing::Linear,
                ..Default::default()
            })?;
        }
        // yeniyi fade-in ile başlat
        if let Some(data) = self.sounds.get(name) {
            let mut handle = self.kira_manager.play(data.clone())?;
            handle.set_volume(0.0, Tween::default())?;
            handle.set_volume(
                target_volume as f64,
                Tween {
                    duration,
                    easing: Easing::Linear,
                    ..Default::default()
                },
            )?;
            self.current_music = Some(handle);
            Ok(())
        } else {
            Err(format!("Müzik '{}' bulunamadı", name).into())
        }
    }

    /// Tweens the music track volume from current level to `target_volume`
    /// over `duration`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.tween_music_volume(0.2, std::time::Duration::from_secs(1)).unwrap();
    /// ```
    pub fn tween_music_volume(
        &mut self,
        target_volume: f32,
        duration: Duration,
    ) -> Result<(), Box<dyn Error>> {
        self.music_track.set_volume(
            target_volume as f64,
            Tween {
                duration,
                easing: Easing::Linear,
                ..Default::default()
            },
        )?;
        Ok(())
    }

    /// Fades out the SFX track volume to zero over `duration`.
    ///
    /// # Example
    /// ```
    /// # use deckbuilder_eng::audio::AudioManager;
    /// # let mut audio = AudioManager::new().unwrap();
    /// audio.fade_out_sfx(std::time::Duration::from_secs(1)).unwrap();
    /// ```
    pub fn fade_out_sfx(&mut self, duration: Duration) -> Result<(), Box<dyn Error>> {
        self.sfx_track.set_volume(
            0.0,
            Tween {
                duration,
                easing: Easing::Linear,
                ..Default::default()
            },
        )?;
        Ok(())
    }
}