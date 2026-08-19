# 🌾 AFRICAN GUN RUNNERS: Start Menu & Custom Background Guide 🖼️

## 1. Start Menu Features
When starting up the game, you are greeted with the main Start Menu featuring:
- **▶ NEW GAME**: Start a fresh farming and defense campaign.
- **💾 CONTINUE / LOAD GAME**: Resume your saved game progress (if a save file exists).
- **⌨ HOW TO PLAY & CONTROLS**: Full guide to movement, farming, market trading, defense turrets, Iron Dome, and weapons.
- **🖼️ CUSTOM BACKGROUND SYNTAX**: View the status of your active main menu background.
- **🚪 QUIT GAME**: Exit to desktop.

---

## 2. How to Add Your Own Custom Background Image
You can drop any custom background image of your choice directly into the game!

### 📂 Directory Location
Place your image file in either:
- The **`assets/`** folder (recommended)
- Or directly in the **main game root folder** (alongside `Cargo.toml`).

### 🏷️ Supported Filenames & Formats
Name your image file using **any** of the following names (in PNG or JPG format):
- `menu_bg.png`
- `menu_bg.jpg`
- `background.png`
- `background.jpg`
- `menu_background.png`
- `menu_background.jpg`

### ⚡ Automatic Detection
Launch or restart the game! The engine will automatically detect, load, and scale your image across the start menu with an anti-glare dark tint to ensure crisp text readability.

If no custom image file is provided, the game automatically falls back to an **animated cinematic 3D live background** orbiting your farm!
