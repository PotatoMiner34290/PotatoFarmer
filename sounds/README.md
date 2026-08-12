# sounds/

Drop audio files here with the **exact base name** shown below. The game tries
`.ogg` → `.wav` → `.flac` for each slot — whichever exists first wins.
Missing slots are silently skipped.

> ⚠ **MP3 is NOT supported.** macroquad's audio backend (`quad-snd`) will hard-crash
> on MP3 files. Convert your MP3s to OGG Vorbis first (free tools below).

| Base name               | Plays when…                              | Notes             |
|-------------------------|------------------------------------------|-------------------|
| `music`                 | Game starts                              | **Loops forever** |
| `turret_fire`           | A turret fires a bullet                  |                   |
| `jet_flyby`             | A jet flies over the field               |                   |
| `jet_shoot`             | A jet fires at the player                |                   |
| `iron_dome_intercept`   | An Iron Dome missile intercepts a rocket |                   |
| `boat_engine`           | A gunboat is active                      | Loops             |
| `thief_giggle`          | A thief child steals a potato            |                   |

## How to convert MP3 → OGG (free)

**FFmpeg** (command line):
```
ffmpeg -i music.mp3 music.ogg
```

**Audacity** (GUI): File → Export → Export as OGG Vorbis

**Online**: https://cloudconvert.com/mp3-to-ogg

Once converted, drop `music.ogg` (or `music.wav`) here and relaunch the game.