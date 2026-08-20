use macroquad::prelude::*;
use crate::constants::*;
use crate::game::Game;

pub fn draw_main_menu(game: &Game) {
    let sw = screen_width();
    let sh = screen_height();

    // 1. Header Title Banner Box
    let title_w = 720.0;
    let title_h = 110.0;
    let title_x = (sw - title_w) / 2.0;
    let title_y = 35.0;

    draw_rectangle(title_x, title_y, title_w, title_h, Color::from_rgba(15, 20, 28, 240));
    draw_rectangle_lines(title_x, title_y, title_w, title_h, 3.0, GOLD);

    draw_text("AFRICAN GUN RUNNERS", title_x + 95.0, title_y + 48.0, 42.0, GOLD);
    draw_text("🌾 POTATO & WEAPON FARMING TYCOON 💣", title_x + 115.0, title_y + 85.0, 22.0, SKYBLUE);

    // 2. Main Menu Options Box
    let menu_w = 560.0;
    let menu_h = 345.0;
    let menu_x = (sw - menu_w) / 2.0;
    let menu_y = title_y + title_h + 25.0;

    draw_rectangle(menu_x, menu_y, menu_w, menu_h, Color::from_rgba(10, 15, 22, 240));
    draw_rectangle_lines(menu_x, menu_y, menu_w, menu_h, 2.5, Color::from_rgba(80, 120, 160, 255));

    let has_save = std::path::Path::new(SAVE_FILE).exists();

    let items = [
        ("▶ NEW GAME [Press N / Enter]", "Start a fresh farming & defense campaign"),
        (if has_save { "💾 CONTINUE / LOAD GAME [Press L]" } else { "💾 CONTINUE / LOAD GAME (No Save Found)" }, if has_save { "Resume your saved progress" } else { "No savefile available yet" }),
        ("⌨ HOW TO PLAY & CONTROLS [Press C]", "View game mechanics, controls & keybindings"),
        ("🚪 QUIT GAME [Press Q]", "Exit to desktop"),
    ];

    let (mx, my) = mouse_position();
    let btn_w = menu_w - 40.0;
    let btn_h = 52.0;
    let start_btn_y = menu_y + 20.0;

    for (i, (label, desc)) in items.iter().enumerate() {
        let cur_y = start_btn_y + i as f32 * 63.0;
        let btn_x = menu_x + 20.0;
        let is_hover = mx >= btn_x && mx <= btn_x + btn_w && my >= cur_y && my <= cur_y + btn_h;
        let is_disabled = i == 1 && !has_save;

        let bg_col = if is_disabled {
            Color::from_rgba(25, 25, 30, 180)
        } else if is_hover {
            Color::from_rgba(45, 80, 120, 255)
        } else {
            Color::from_rgba(20, 30, 42, 220)
        };

        let border_col = if is_disabled {
            DARKGRAY
        } else if is_hover {
            GOLD
        } else {
            Color::from_rgba(70, 95, 120, 255)
        };

        let text_col = if is_disabled {
            GRAY
        } else if is_hover {
            WHITE
        } else {
            LIGHTGRAY
        };

        draw_rectangle(btn_x, cur_y, btn_w, btn_h, bg_col);
        draw_rectangle_lines(btn_x, cur_y, btn_w, btn_h, if is_hover { 2.5 } else { 1.5 }, border_col);

        draw_text(label, btn_x + 15.0, cur_y + 26.0, 20.0, if is_disabled { GRAY } else if is_hover { GOLD } else { WHITE });
        draw_text(desc, btn_x + 15.0, cur_y + 44.0, 14.0, text_col);
    }

    // 3. Custom Background Banner / Syntax Indicator at Bottom
    let banner_w = sw - 60.0;
    let banner_h = 45.0;
    let banner_x = 30.0;
    let banner_y = sh - banner_h - 20.0;

    draw_rectangle(banner_x, banner_y, banner_w, banner_h, Color::from_rgba(12, 18, 26, 240));
    draw_rectangle_lines(banner_x, banner_y, banner_w, banner_h, 2.0, SKYBLUE);

    if let Some(ref path) = game.background_file_name {
        let msg = format!("🟢 CUSTOM BACKGROUND ACTIVE: [{}]  (Drop images into 'assets/menu_bg.png' to change)", path);
        draw_text(&msg, banner_x + 20.0, banner_y + 28.0, 17.0, GREEN);
    } else {
        let msg = "🖼️ CUSTOM BACKGROUND SYNTAX: Drop 'menu_bg.png' or 'background.png' into 'assets/' folder or root! Press [B] for Info";
        draw_text(msg, banner_x + 20.0, banner_y + 28.0, 17.0, YELLOW);
    }
}

pub fn draw_controls_overlay() {
    let sw = screen_width();
    let sh = screen_height();

    let box_w = 740.0;
    let box_h = 490.0;
    let box_x = (sw - box_w) / 2.0;
    let box_y = (sh - box_h) / 2.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::from_rgba(12, 18, 25, 250));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 3.0, GOLD);

    draw_text("=== HOW TO PLAY & GAME CONTROLS ===", box_x + 30.0, box_y + 40.0, 24.0, GOLD);

    let controls = [
        ("MOVEMENT", "WASD or Arrow Keys to walk around farm field & river markets"),
        ("PLOWING SOIL", "Hold [SPACE] while moving on grass to plow soil rows"),
        ("PLANT / HARVEST", "Press [E] on plowed soil to plant seeds or harvest mature crops"),
        ("DEDICATED MARKET", "Press [M] near Market structure to buy Worker Slaves & gear"),
        ("DEFENSE TURRETS", "Press [B] to place Automated Defense Turrets (costs potatoes/cash)"),
        ("IRON DOME", "Press [I] to deploy Iron Dome Missile Anti-Air Defense Battery"),
        ("PICKUP STRUCTURE", "Press [P] while near a turret or iron dome to reclaim it"),
        ("WEAPONS / COMBAT", "Minigun auto-fires at incoming threats (Thieves, Gunboats, Jets)"),
        ("SAVE / LOAD", "Press [F5] / [K] to Save Game  |  Press [F9] / [L] to Load Game"),
        ("VOLUME & PAUSE", "Ctrl + '+' / '-' for Volume  |  [ESC] or [TAB] for Pause Menu"),
    ];

    let mut start_y = box_y + 80.0;
    for (category, desc) in controls.iter() {
        draw_text(category, box_x + 30.0, start_y, 16.0, SKYBLUE);
        draw_text(desc, box_x + 220.0, start_y, 16.0, WHITE);
        start_y += 34.0;
    }

    let close_btn_w = 320.0;
    let close_btn_h = 42.0;
    let close_btn_x = box_x + (box_w - close_btn_w) / 2.0;
    let close_btn_y = box_y + box_h - 55.0;

    let (mx, my) = mouse_position();
    let is_hover = mx >= close_btn_x && mx <= close_btn_x + close_btn_w && my >= close_btn_y && my <= close_btn_y + close_btn_h;

    draw_rectangle(close_btn_x, close_btn_y, close_btn_w, close_btn_h, if is_hover { Color::from_rgba(60, 90, 130, 255) } else { Color::from_rgba(30, 45, 65, 230) });
    draw_rectangle_lines(close_btn_x, close_btn_y, close_btn_w, close_btn_h, 2.0, if is_hover { GOLD } else { WHITE });

    draw_text("RETURN TO MENU [ESC / SPACE]", close_btn_x + 20.0, close_btn_y + 27.0, 18.0, if is_hover { GOLD } else { WHITE });
}

pub fn draw_bg_info_overlay(game: &Game) {
    let sw = screen_width();
    let sh = screen_height();

    let box_w = 740.0;
    let box_h = 480.0;
    let box_x = (sw - box_w) / 2.0;
    let box_y = (sh - box_h) / 2.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::from_rgba(12, 18, 25, 250));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 3.0, GOLD);

    draw_text("=== CUSTOM BACKGROUND IMAGE SYNTAX ===", box_x + 30.0, box_y + 40.0, 24.0, GOLD);

    let info_lines = [
        "You can easily set any custom background image for the start menu!",
        "",
        "SYNTAX & STEPS TO ADD YOUR OWN BACKGROUND:",
        "1. Save any PNG or JPG image of your choice.",
        "2. Drop it into the 'assets/' folder (or into the main game root folder).",
        "3. Name the image file as any of the following supported names:",
        "    • menu_bg.png   or   menu_bg.jpg",
        "    • background.png   or   background.jpg",
        "    • menu_background.png   or   menu_background.jpg",
        "4. Launch or restart the game! The engine automatically detects,",
        "   loads, and scales your background image to fit the menu screen.",
        "",
    ];

    let mut start_y = box_y + 80.0;
    for line in info_lines.iter() {
        if line.starts_with("SYNTAX") {
            draw_text(line, box_x + 30.0, start_y, 18.0, SKYBLUE);
        } else if line.trim_start().starts_with("•") {
            draw_text(line, box_x + 30.0, start_y, 17.0, GREEN);
        } else {
            draw_text(line, box_x + 30.0, start_y, 16.0, WHITE);
        }
        start_y += 24.0;
    }

    // Current Active Status
    let status_box_y = start_y + 10.0;
    draw_rectangle(box_x + 30.0, status_box_y, box_w - 60.0, 45.0, Color::from_rgba(20, 30, 45, 230));
    draw_rectangle_lines(box_x + 30.0, status_box_y, box_w - 60.0, 45.0, 1.5, SKYBLUE);

    if let Some(ref path) = game.background_file_name {
        let msg = format!("Current Active Image: [{}]", path);
        draw_text(&msg, box_x + 45.0, status_box_y + 28.0, 18.0, GREEN);
    } else {
        draw_text("Current Active Image: None (Using Procedural 3D Atmosphere)", box_x + 45.0, status_box_y + 28.0, 18.0, YELLOW);
    }

    let close_btn_w = 320.0;
    let close_btn_h = 42.0;
    let close_btn_x = box_x + (box_w - close_btn_w) / 2.0;
    let close_btn_y = box_y + box_h - 55.0;

    let (mx, my) = mouse_position();
    let is_hover = mx >= close_btn_x && mx <= close_btn_x + close_btn_w && my >= close_btn_y && my <= close_btn_y + close_btn_h;

    draw_rectangle(close_btn_x, close_btn_y, close_btn_w, close_btn_h, if is_hover { Color::from_rgba(60, 90, 130, 255) } else { Color::from_rgba(30, 45, 65, 230) });
    draw_rectangle_lines(close_btn_x, close_btn_y, close_btn_w, close_btn_h, 2.0, if is_hover { GOLD } else { WHITE });

    draw_text("RETURN TO MENU [ESC / SPACE]", close_btn_x + 20.0, close_btn_y + 27.0, 18.0, if is_hover { GOLD } else { WHITE });
}
