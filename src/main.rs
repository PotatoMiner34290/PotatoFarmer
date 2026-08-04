use macroquad::prelude::*;

struct DirtParticle {
    position: Vec2,
    velocity: Vec2,
    life: f32,
}

struct Farmer {
    position: Vec2,
    plowing: bool,
}

fn spawn_dirt(particles: &mut Vec<DirtParticle>, pos: Vec2) {
    for _ in 0..5 {
        particles.push(DirtParticle {
            position: pos,
            velocity: vec2(
                rand::gen_range(-50., 50.),
                rand::gen_range(-120., -40.)
            ),
            life: 1.0,
        });
    }
}

fn draw_farmer(farmer: &Farmer) {
    let x = farmer.position.x;
    let y = farmer.position.y;

    // head
    draw_circle(x, y - 40., 10., BLACK);

    // body
    draw_line(x, y - 30., x, y, 3., BLACK);

    // arms
    draw_line(x - 15., y - 15., x + 15., y - 15., 3., BLACK);

    // legs
    draw_line(x, y, x - 10., y + 25., 3., BLACK);
    draw_line(x, y, x + 10., y + 25., 3., BLACK);

    // plow
    if farmer.plowing {
        draw_line(
            x + 15.,
            y - 15.,
            x + 50.,
            y + 20.,
            5.,
            DARKGRAY
        );
    }
}

#[macroquad::main("Nigga Farmer")]
async fn main() {
    let mut farmer = Farmer {
        position: vec2(400., 300.),
        plowing: false,
    };

    let mut dirt: Vec<DirtParticle> = Vec::new();

    loop {
        clear_background(Color::from_rgba(150, 220, 255, 255));

        // ground
        draw_rectangle(
            0.,
            350.,
            screen_width(),
            200.,
            Color::from_rgba(120, 80, 40, 255),
        );

        farmer.plowing = is_key_down(KeyCode::Space);

        if farmer.plowing {
            spawn_dirt(
                &mut dirt,
                vec2(
                    farmer.position.x + 40.,
                    farmer.position.y + 20.,
                ),
            );
        }

        // update dirt
        for particle in dirt.iter_mut() {
            particle.velocity.y += 200. * get_frame_time();
            particle.position += particle.velocity * get_frame_time();
            particle.life -= get_frame_time();
        }

        dirt.retain(|p| p.life > 0.);

        // draw dirt
        for particle in &dirt {
            draw_circle(
                particle.position.x,
                particle.position.y,
                3.,
                BROWN,
            );
        }

        draw_farmer(&farmer);

        draw_text(
            "Hold SPACE to plow",
            20.,
            30.,
            30.,
            BLACK,
        );

        next_frame().await;
    }
}