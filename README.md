# 🥔 Potato Farmer (African Gun Runner & Defense Tycoon Edition)

An action-packed low-poly 3D potato farming and defense simulator built in Rust with `macroquad` and `rayon`.

Till rich soil, plant and harvest potatoes, trade at local markets, defend against thief children and Cold War African Rebel gunboat river raids, deploy automated defense turrets and Israeli Iron Dome batteries, shoot down B-2 Stealth Bombers, collect secret crashed loot, hire AI worker slaves, and wield heavy automated miniguns!

---

## 🎮 How to Play & Controls

| Control | Action |
| --- | --- |
| **WASD / Arrow Keys** | Move step-by-step across the grid & surrounding village |
| **Hold SPACE** | Till & plow soil rows |
| **Press E** | Plant seed (on plowed soil) / Harvest mature potatoes |
| **Press M or E (at Market)** | Open/Close Dedicated Market & Revolutionary Worker Shop GUI |
| **Market Key 1** | Sell Panther Statues ($2,500 Cash) |
| **Market Key 2** | Sell Blood Diamonds ($1,500 Cash) |
| **Market Key 3** | Sell Gold Bars ($200 Cash) |
| **Market Key 4** | Trade 1 Potato $\rightarrow$ 4 Seeds |
| **Market Key 5** | Hire AI Worker Slave (150 Potatoes / $500 Cash) |
| **Market Key 6** | Toggle AI Worker Mode (Plant & Harvest vs Plant Only) |
| **Market Key 7** | Buy +100 Minigun Bullets ($300 Cash / 40 Potatoes) |
| **Press T (at Market)** | Buy Automated Defense Turret (50 Potatoes) |
| **Press Y (at Market)** | Buy Israeli Iron Dome Air Defense Battery (120 Potatoes) |
| **Press B** | Place Defense Turret down from inventory at current position |
| **Press I** | Deploy Iron Dome Battery down from inventory |
| **Hold LMB / F / M** | Manual override to fire Heavy Minigun (Auto-targets when unlocked!) |
| **Press TAB / ESC / V** | Open / Close Inventory & Currency GUI |
| **F5 / K** | Save game (`savegame.json`) |
| **F9 / L** | Load saved game |

---

## ⚡ Real Features

- **Dynamic Crop Simulation**: Multi-threaded parallel crop growth powered by `rayon`.
- **Israeli Iron Dome Missile Battery**: Intercepts overhead B-2 Stealth Bombers in real-time with surface-to-air guided missiles.
- **Crashing B-2 Stealth Bombers & Ground Drops**: Shot-down B-2 bombers plummet, tumble, and explode on farm ground, dropping rare secret loot items right where they crash.
- **Hidden Inventory & Special Currencies**: Unlock Blood Diamonds, Cash, Panther Statues, Gold Bars, Bullets, and Heavy Miniguns. Item slots stay hidden in your inventory until shot down and picked up!
- **Automated Heavy Minigun**: Automatically targets and shreds incoming threats (thief children, African rebels, and gunboats).
- **Cold War African Rebel Gunboat Raids**: Gunboats patrol the river every 30s and disembark armed AK-47 rebels to attack your farm.
- **Dedicated Market GUI**: Dedicated shop overlay to sell precious artifacts/loot for Cash, buy ammo, and hire workers.
- **AI Farmer Slaves**: Hire automated AI farm workers that roam your farm to plow, plant, and harvest crops automatically.
- **Thief Children Defense**: Tanky thief children attempt to steal mature crops; defend your farm with turrets, miniguns, and Iron Domes.
- **Persistent Save & Load**: Fully integrated JSON saving (`savegame.json`).

---

## 🛠️ Building & Running

Requires [Rust & Cargo](https://rustup.rs/):

```bash
cargo run --release
```
