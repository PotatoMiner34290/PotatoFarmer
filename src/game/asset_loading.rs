use macroquad::prelude::*;
use super::Game;
use super::obj_parser::*;

impl Game {
    pub async fn load_background(&mut self) {
        let candidate_paths = [
            "assets/menu_bg.png",
            "assets/menu_bg.jpg",
            "assets/menu_background.png",
            "assets/menu_background.jpg",
            "assets/background.png",
            "assets/background.jpg",
            "menu_bg.png",
            "menu_bg.jpg",
            "menu_background.png",
            "menu_background.jpg",
            "background.png",
            "background.jpg",
        ];

        for path in candidate_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(tex) = load_texture(path).await {
                    tex.set_filter(FilterMode::Linear);
                    self.menu_background = Some(tex);
                    self.background_file_name = Some(path.to_string());
                    println!("Loaded custom main menu background from: {}", path);
                    break;
                }
            }
        }
    }

    pub async fn load_turret_model(&mut self) {
        let candidate_paths = ["assets/Turret.obj", "Turret.obj"];
        for path in candidate_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let meshes = parse_obj(&content);
                    if !meshes.is_empty() {
                        println!("Loaded custom turret OBJ model from: {}", path);
                        self.turret_meshes = meshes;
                        break;
                    }
                }
            }
        }
    }

    pub async fn load_iron_dome_model(&mut self) {
        let mtl_paths = ["assets/Iron_Dome.mtl", "Iron_Dome.mtl"];
        let mut mtl_map = None;
        for path in mtl_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let map = parse_mtl(&content);
                    if !map.is_empty() {
                        println!("Loaded Iron_Dome MTL material definitions from: {}", path);
                        mtl_map = Some(map);
                        break;
                    }
                }
            }
        }

        let obj_paths = ["assets/Iron_Dome.obj", "Iron_Dome.obj"];
        for path in obj_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let meshes = parse_obj_with_mtl(&content, mtl_map.as_ref());
                    if !meshes.is_empty() {
                        println!("Loaded Iron_Dome OBJ model from: {}", path);
                        self.iron_dome_meshes = meshes;
                        break;
                    }
                }
            }
        }
    }

    pub async fn load_iron_dome_missile_model(&mut self) {
        let mtl_paths = ["assets/missile_iron_dome.mtl", "missile_iron_dome.mtl"];
        let mut mtl_map = None;
        for path in mtl_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let map = parse_mtl(&content);
                    if !map.is_empty() {
                        println!("Loaded missile_iron_dome MTL material definitions from: {}", path);
                        mtl_map = Some(map);
                        break;
                    }
                }
            }
        }

        let obj_paths = ["assets/missile_iron_dome.obj", "missile_iron_dome.obj"];
        for path in obj_paths {
            if std::path::Path::new(path).exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let meshes = parse_obj_with_mtl(&content, mtl_map.as_ref());
                    if !meshes.is_empty() {
                        println!("Loaded missile_iron_dome OBJ model from: {}", path);
                        self.iron_dome_missile_meshes = meshes;
                        break;
                    }
                }
            }
        }
    }
}
