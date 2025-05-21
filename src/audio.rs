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

pub struct AudioManager {
    kira_manager: KiraManager,
    music_track: TrackHandle,
    sfx_track: TrackHandle,
    sounds: HashMap<String, StaticSoundData>,
    current_music: Option<StaticSoundHandle>,
}

impl AudioManager {
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

    pub fn play_sound(&mut self, name: &str, volume: f32) -> Result<(), Box<dyn Error>> {
        if let Some(sound_data) = self.sounds.get(name) {
            let mut handle = self.kira_manager.play(sound_data.clone())?;
            handle.set_volume(volume as f64, Tween::default())?;
            Ok(())
        } else {
            Err(format!("Ses '{}' bulunamadı", name).into())
        }
    }

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

    pub fn stop_music(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(mut handle) = self.current_music.take() {
            handle.stop(Tween::default())?;
        }
        Ok(())
    }

    pub fn set_music_volume(&mut self, volume: f32) -> Result<(), Box<dyn Error>> {
        self.music_track.set_volume(volume as f64, Tween::default())?;
        Ok(())
    }

    pub fn set_sfx_volume(&mut self, volume: f32) -> Result<(), Box<dyn Error>> {
        self.sfx_track.set_volume(volume as f64, Tween::default())?;
        Ok(())
    }

    pub fn music_track(&self) -> &TrackHandle {
        &self.music_track
    }

    pub fn sfx_track(&self) -> &TrackHandle {
        &self.sfx_track
    }

    /// Müziğin playback hızını (pitch) ayarlar.
    /// pitch = 1.0 normal, >1.0 hızlı, <1.0 yavaş.
    pub fn set_music_pitch(&mut self, _speed: f32) -> Result<(), Box<dyn Error>> {
        // pitch control not supported in this build
        Ok(())
    }

    /// Müziği duraklatır (fade için Tween verin veya Tween::default())
    pub fn pause_music(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.pause(Tween::default())?;
        }
        Ok(())
    }

    /// Müziği devam ettirir
    pub fn resume_music(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.resume(Tween::default())?;
        }
        Ok(())
    }

    /// Verilen süre boyunca lineer fade-out yapıp durdurur
    pub fn fade_out_music(&mut self, duration: Duration) -> Result<(), Box<dyn Error>> {
        if let Some(handle) = &mut self.current_music {
            handle.stop(Tween { duration, easing: Easing::Linear, ..Default::default() })?;
        }
        Ok(())
    }
}