use macroquad::prelude::*;
use std::collections::HashMap;

pub fn parse_mtl(mtl_data: &str) -> HashMap<String, Color> {
    let mut materials = HashMap::new();
    let mut cur_name = String::new();

    for line in mtl_data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        let cmd = match parts.next() {
            Some(c) => c,
            None => continue,
        };

        match cmd {
            "newmtl" => {
                if let Some(name) = parts.next() {
                    cur_name = name.to_string();
                }
            }
            "Kd" => {
                if !cur_name.is_empty() {
                    let r: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                    let g: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                    let b: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);

                    let gamma = |c: f32| -> f32 {
                        if c <= 0.0 { 0.0 }
                        else if c >= 1.0 { 1.0 }
                        else { c.powf(1.0 / 2.2) }
                    };

                    let color = Color {
                        r: gamma(r).clamp(0.0, 1.0),
                        g: gamma(g).clamp(0.0, 1.0),
                        b: gamma(b).clamp(0.0, 1.0),
                        a: 1.0,
                    };
                    materials.insert(cur_name.clone(), color);
                }
            }
            _ => {}
        }
    }

    materials
}

pub fn parse_obj_with_mtl(obj_data: &str, mtl_map: Option<&HashMap<String, Color>>) -> Vec<Mesh> {
    let mut raw_positions: Vec<Vec3> = Vec::new();
    let mut raw_uvs: Vec<Vec2> = Vec::new();

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    let mut meshes: Vec<Mesh> = Vec::new();

    let mut index_map: HashMap<(usize, usize, u8, u8, u8, u8), u16> = HashMap::new();
    let mut current_color = Color::from_rgba(180, 190, 200, 255);

    for line in obj_data.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.split_whitespace();
        let cmd = match parts.next() {
            Some(c) => c,
            None => continue,
        };

        match cmd {
            "usemtl" => {
                let mat_name = parts.next().unwrap_or("");
                let mut found = false;

                if let Some(map) = mtl_map {
                    if let Some(&col) = map.get(mat_name) {
                        current_color = col;
                        found = true;
                    } else {
                        let lower = mat_name.to_lowercase();
                        for (key, &col) in map.iter() {
                            if key.to_lowercase() == lower {
                                current_color = col;
                                found = true;
                                break;
                            }
                        }
                    }
                }

                if !found {
                    let mat_lower = mat_name.to_lowercase();
                    if mat_lower.contains("red") {
                        current_color = Color::from_rgba(235, 45, 45, 255);
                    } else if mat_lower.contains("metaldark") {
                        current_color = Color::from_rgba(50, 55, 62, 255);
                    } else if mat_lower.contains("dark") {
                        current_color = Color::from_rgba(28, 30, 34, 255);
                    } else if mat_lower.contains("metal") {
                        current_color = Color::from_rgba(180, 190, 200, 255);
                    } else {
                        current_color = Color::from_rgba(160, 170, 180, 255);
                    }
                }
            }
            "v" => {
                let x: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let y: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let z: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                raw_positions.push(vec3(x, y, z));
            }
            "vt" => {
                let u: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                let v: f32 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
                raw_uvs.push(vec2(u, v));
            }
            "f" => {
                let r = (current_color.r * 255.0) as u8;
                let g = (current_color.g * 255.0) as u8;
                let b = (current_color.b * 255.0) as u8;
                let a = (current_color.a * 255.0) as u8;

                let mut face_verts = Vec::new();
                for token in parts {
                    let mut sub = token.split('/');
                    let v_str = sub.next().unwrap_or("");
                    if v_str.is_empty() { continue; }
                    let v_idx: i32 = v_str.parse().unwrap_or(0);
                    let pos_idx = if v_idx < 0 {
                        (raw_positions.len() as i32 + v_idx) as usize
                    } else if v_idx > 0 {
                        (v_idx - 1) as usize
                    } else { 0 };

                    let vt_str = sub.next().unwrap_or("");
                    let vt_idx = if !vt_str.is_empty() {
                        let idx: i32 = vt_str.parse().unwrap_or(0);
                        if idx < 0 { (raw_uvs.len() as i32 + idx) as usize }
                        else if idx > 0 { (idx - 1) as usize }
                        else { 0 }
                    } else { 0 };

                    let key = (pos_idx, vt_idx, r, g, b, a);
                    let idx = if let Some(&i) = index_map.get(&key) {
                        i
                    } else {
                        let pos = raw_positions.get(pos_idx).copied().unwrap_or(vec3(0.0, 0.0, 0.0));
                        let uv = raw_uvs.get(vt_idx).copied().unwrap_or(vec2(0.0, 0.0));
                        let vertex = Vertex::new2(pos, uv, current_color);
                        let i = vertices.len() as u16;
                        vertices.push(vertex);
                        index_map.insert(key, i);
                        i
                    };
                    face_verts.push(idx);
                }

                for i in 1..face_verts.len().saturating_sub(1) {
                    indices.push(face_verts[0]);
                    indices.push(face_verts[i]);
                    indices.push(face_verts[i + 1]);
                }
            }
            _ => {}
        }
    }

    if !vertices.is_empty() && !indices.is_empty() {
        meshes.push(Mesh {
            vertices,
            indices,
            texture: None,
        });
    }

    meshes
}

pub fn parse_obj(obj_data: &str) -> Vec<Mesh> {
    parse_obj_with_mtl(obj_data, None)
}
