use macroquad::audio::{load_sound, play_sound, stop_sound, set_sound_volume, Sound, PlaySoundParams};

/// Sound effects and music. Each slot maps 1-to-1 with a file in `sounds/`.
/// See `sounds/README.md` for the full reference table.
///
/// Supported formats: OGG Vorbis (.ogg), WAV (.wav), FLAC (.flac)
/// ⚠ MP3 is NOT supported by macroquad's audio backend — convert to OGG first.
///
/// | Filename (any of .ogg / .wav / .flac) | Plays when…                              | Notes             |
/// |---------------------------------------|------------------------------------------|-------------------|
/// | `music`                               | Game starts                              | **Loops forever** |
/// | `turret_fire`                         | A turret fires a bullet                  |                   |
/// | `jet_flyby`                           | A jet flies over the field               |                   |
/// | `jet_shoot`                           | A jet fires at the player                |                   |
/// | `iron_dome_intercept`                 | An Iron Dome missile intercepts a rocket |                   |
/// | `boat_engine`                         | A gunboat is active                      | Loops             |
/// | `thief_giggle`                        | A thief child steals a potato            |                   |
pub struct SoundEffects {
    pub music:               Option<Sound>,
    pub turret_fire:         Option<Sound>,
    pub jet_flyby:           Option<Sound>,
    pub jet_shoot:           Option<Sound>,
    pub iron_dome_intercept: Option<Sound>,
    pub boat_engine:         Option<Sound>,
    pub thief_giggle:        Option<Sound>,
    pub footstep:            Option<Sound>,
    pub slave_talk:          Option<Sound>,
    pub slave_work:          Option<Sound>,
    pub volume:              f32,
    pub is_music_muted:      bool,
}

impl SoundEffects {
    /// Returns an empty `SoundEffects` with all slots set to `None`.
    pub fn empty() -> Self {
        Self {
            music:               None,
            turret_fire:         None,
            jet_flyby:           None,
            jet_shoot:           None,
            iron_dome_intercept: None,
            boat_engine:         None,
            thief_giggle:        None,
            footstep:            None,
            slave_talk:          None,
            slave_work:          None,
            volume:              1.0,
            is_music_muted:      false,
        }
    }

    /// Loads every sound from the `sounds/` folder, then immediately starts
    /// looping the background music track (if present).
    pub async fn load() -> Self {
        let sfx = Self {
            music:               try_load("sounds/music").await,
            turret_fire:         try_load("sounds/turret_fire").await,
            jet_flyby:           try_load("sounds/jet_flyby").await,
            jet_shoot:           try_load("sounds/jet_shoot").await,
            iron_dome_intercept: try_load("sounds/iron_dome_intercept").await,
            boat_engine:         try_load("sounds/boat_engine").await,
            thief_giggle:        try_load("sounds/thief_giggle").await,
            footstep:            try_load("sounds/footstep").await,
            slave_talk:          try_load("sounds/slave_talk").await,
            slave_work:          try_load("sounds/slave_work").await,
            volume:              1.0,
            is_music_muted:      false,
        };

        // Start background music immediately, looped at 80% volume.
        sfx.play_music();

        sfx
    }

    pub fn toggle_music_mute(&mut self) {
        self.is_music_muted = !self.is_music_muted;
        let music_vol = if self.is_music_muted { 0.0 } else { 0.8 * self.volume };
        if let Some(ref m) = self.music {
            set_sound_volume(m, music_vol);
        }
    }

    pub fn set_volume(&mut self, new_vol: f32) {
        self.volume = new_vol.clamp(0.0, 1.0);
        let music_vol = if self.is_music_muted { 0.0 } else { 0.8 * self.volume };
        if let Some(ref m) = self.music {
            set_sound_volume(m, music_vol);
        }
        if let Some(ref b) = self.boat_engine {
            set_sound_volume(b, 0.6 * self.volume);
        }
    }

    // ── Playback helpers ────────────────────────────────────────────────────

    /// Play a sound once with scaled volume (fire-and-forget).
    pub fn play_once(&self, sound: &Option<Sound>, base_volume: f32) {
        let final_vol = (base_volume * self.volume).clamp(0.0, 1.0);
        if final_vol <= 0.001 { return; }
        if let Some(s) = sound {
            play_sound(s, PlaySoundParams { looped: false, volume: final_vol });
        }
    }

    /// Play a sound with full control over volume / looping.
    pub fn play(&self, sound: &Option<Sound>, looped: bool, volume: f32) {
        let final_vol = (volume * self.volume).clamp(0.0, 1.0);
        if final_vol <= 0.001 && !looped { return; }
        if let Some(s) = sound {
            play_sound(s, PlaySoundParams { looped, volume: final_vol });
        }
    }

    // ── Convenience methods (one per slot) ─────────────────────────────────

    /// Start (or restart) the background music loop.
    pub fn play_music(&self) {
        if let Some(ref m) = self.music {
            stop_sound(m);
            let music_vol = if self.is_music_muted { 0.0 } else { 0.8 * self.volume };
            play_sound(m, PlaySoundParams { looped: true, volume: music_vol });
        }
    }

    pub fn play_turret_fire(&self)         { self.play_once(&self.turret_fire, 1.0); }
    pub fn play_jet_flyby(&self)           { self.play(&self.jet_flyby, false, 1.0); }
    pub fn play_jet_shoot(&self)           { self.play_once(&self.jet_shoot, 1.0); }
    pub fn play_iron_dome_intercept(&self) { self.play_once(&self.iron_dome_intercept, 1.0); }
    pub fn play_boat_engine(&self)         { self.play(&self.boat_engine, true, 0.6); }
    pub fn play_thief_giggle(&self)        { self.play_once(&self.thief_giggle, 1.0); }
    pub fn play_footstep(&self)            { self.play_once(&self.footstep, 0.5); }
    pub fn play_slave_talk(&self) {
        if self.slave_talk.is_some() {
            self.play_once(&self.slave_talk, 0.85);
        } else {
            self.play_thief_giggle();
        }
    }
    pub fn play_slave_work(&self) {
        if self.slave_work.is_some() {
            self.play_once(&self.slave_work, 0.8);
        } else if self.slave_talk.is_some() {
            self.play_once(&self.slave_talk, 0.7);
        } else {
            self.play_thief_giggle();
        }
    }
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Try to load `base.ogg`, then `base.wav`, then `base.flac`.
/// Returns the first one that succeeds, or `None` if none exist / are readable.
/// MP3 is intentionally excluded — quad-snd panics on it (UnsupportedFormat).
async fn try_load(base: &str) -> Option<Sound> {
    let prefixes = ["assets/", ""];
    let exts = ["ogg", "wav", "flac"];

    for prefix in &prefixes {
        for ext in &exts {
            let path = format!("{}{}.{}", prefix, base, ext);
            if std::path::Path::new(&path).exists() {
                match load_sound(&path).await {
                    Ok(s) => return Some(s),
                    Err(e) => eprintln!("[audio] failed to decode {}: {:?}", path, e),
                }
            }
        }
    }
    None
}
