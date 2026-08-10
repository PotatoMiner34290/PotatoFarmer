use macroquad::audio::{load_sound, play_sound, play_sound_once, Sound, PlaySoundParams};

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
        }
    }

    /// Loads every sound from the `sounds/` folder, then immediately starts
    /// looping the background music track (if present).
    ///
    /// Each slot tries `.ogg` → `.wav` → `.flac` in order — whichever file
    /// exists first wins. Missing / unsupported files are silently skipped.
    pub async fn load() -> Self {
        let sfx = Self {
            music:               try_load("sounds/music").await,
            turret_fire:         try_load("sounds/turret_fire").await,
            jet_flyby:           try_load("sounds/jet_flyby").await,
            jet_shoot:           try_load("sounds/jet_shoot").await,
            iron_dome_intercept: try_load("sounds/iron_dome_intercept").await,
            boat_engine:         try_load("sounds/boat_engine").await,
            thief_giggle:        try_load("sounds/thief_giggle").await,
        };

        // Start background music immediately, looped at 80% volume.
        sfx.play_music();

        sfx
    }

    // ── Playback helpers ────────────────────────────────────────────────────

    /// Play a sound once at full volume (fire-and-forget).
    pub fn play_once(sound: &Option<Sound>) {
        if let Some(s) = sound {
            play_sound_once(s);
        }
    }

    /// Play a sound with full control over volume / looping.
    pub fn play(sound: &Option<Sound>, looped: bool, volume: f32) {
        if let Some(s) = sound {
            play_sound(s, PlaySoundParams { looped, volume });
        }
    }

    // ── Convenience methods (one per slot) ─────────────────────────────────

    /// Start (or restart) the background music loop.
    pub fn play_music(&self)               { Self::play(&self.music, true, 0.8); }

    pub fn play_turret_fire(&self)         { Self::play_once(&self.turret_fire); }
    pub fn play_jet_flyby(&self)           { Self::play(&self.jet_flyby, false, 1.0); }
    pub fn play_jet_shoot(&self)           { Self::play_once(&self.jet_shoot); }
    pub fn play_iron_dome_intercept(&self) { Self::play_once(&self.iron_dome_intercept); }
    pub fn play_boat_engine(&self)         { Self::play(&self.boat_engine, true, 0.6); }
    pub fn play_thief_giggle(&self)        { Self::play_once(&self.thief_giggle); }
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Try to load `base.ogg`, then `base.wav`, then `base.flac`.
/// Returns the first one that succeeds, or `None` if none exist / are readable.
/// MP3 is intentionally excluded — quad-snd panics on it (UnsupportedFormat).
async fn try_load(base: &str) -> Option<Sound> {
    for ext in &["ogg", "wav", "flac"] {
        let path = format!("{}.{}", base, ext);
        if std::path::Path::new(&path).exists() {
            match load_sound(&path).await {
                Ok(s)  => return Some(s),
                Err(e) => eprintln!("[audio] failed to decode {}: {:?}", path, e),
            }
        }
    }
    None
}
