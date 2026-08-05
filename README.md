# 🥔 Potato Farmer (African Gun Runner Edition)

just a low-poly 3D potato farming simulator built in Rust with `macroquad`.

you till soil, plant potatoes, harvest them, trade with markets, avoid getting raided by local thief children, and build automated laser defense turrets. also there's B-2 stealth bombers flying overhead sometimes because why not.

---

## 🎮 How to Play

| Control | Action |
| --- | --- |
| **WASD / Arrow Keys** | Walk 1 square at a time across the grid |
| **Hold SPACE** | Till & plow soil rows |
| **Press E** | Plant seed (on plowed soil) / Harvest crop (when mature) |
| **Press E (at Market)** | Trade 1 Potato $\rightarrow$ 4 Seeds |
| **Press T (at Market)** | Buy Automated Defense Turret (50 Potatoes) |
| **Press B** | Place Turret down from inventory on your current plot |
| **F5 / K** | Save game (`savegame.json`) |
| **F9 / L** | Load saved game |

---

## ⚡ Features

- **Dynamic Potato Economy**: Till soil, sow seeds, watch potatoes grow in real-time with multi-core parallel simulation (`rayon`).
- **Placeable Defense Turrets**: Buy turrets from the local market, carry them in your inventory, and place them anywhere on your land to defend your crops.
- **Tanky Thief Children**: Fast local thief children raid your mature potato fields. They carry 3 HP health bars and take multiple turret laser hits to eliminate before they swipe your harvest.
- **Air Raids**: B-2 Stealth Bombers and jet fighters occasionally dogfight in the sky overhead.
- **Auto Save & Load**: Persistent JSON saving so you never lose your farm progress.

---

## 🛠️ Building & Running

Requires [Rust & Cargo](https://rustup.rs/):

```bash
cargo run --release
```
