use macroquad::prelude::*;

const GRID: usize = 20;
const CELL: f32 = 2.0;
const FIELD_HALF: f32 = GRID as f32 * CELL / 2.0;

const GROW_TIME: f32 = 18.0;

const MOVE_LERP: f32 = 16.0;
const CAM_SMOOTH: f32 = 8.0;

// Bigger field needs a wider camera
const CAM_OFFSET: Vec3 = vec3(22.0, 28.0, 22.0);

const STEP_REPEAT: f32 = 0.18;

// Potato conversion station
const SEED_STATION_POS: Vec3 = vec3(-FIELD_HALF - 3.0, 0.0, 0.0);
const POTATO_TO_SEED: u32 = 4;


#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Grass,
    Plowed,
    Planted { growth: f32 },
}


struct DirtParticle {
    position: Vec3,
    velocity: Vec3,
    life: f32,
}


struct Farmer {
    grid_x: usize,
    grid_z: usize,

    position: Vec3,
    facing: f32,

    plowing: bool,
    step_cooldown: f32,
}


struct CameraState {
    position: Vec3,
    target: Vec3,
}


struct Game {

    field: [[CellState; GRID]; GRID],

    farmer: Farmer,

    camera: CameraState,

    dirt: Vec<DirtParticle>,

    seeds: u32,
    potatoes: u32,

    action_cooldown: f32,
}



impl Game {

    fn new() -> Self {

        let start_x = GRID / 2;
        let start_z = GRID / 2;

        let start_pos = Self::cell_center(start_x, start_z);

        let cam_target = start_pos + vec3(0.0, 0.8, 0.0);


        Self {

            field: [[CellState::Grass; GRID]; GRID],


            farmer: Farmer {

                grid_x: start_x,
                grid_z: start_z,

                position: start_pos,

                facing: 0.0,

                plowing: false,

                step_cooldown: 0.0,
            },


            camera: CameraState {

                position: cam_target + CAM_OFFSET,

                target: cam_target,
            },


            dirt: Vec::new(),


            // Starting seeds
            seeds: 24,

            potatoes: 0,


            action_cooldown: 0.0,
        }
    }



    fn cell_center(grid_x: usize, grid_z: usize) -> Vec3 {

        vec3(

            -FIELD_HALF 
                + grid_x as f32 * CELL
                + CELL / 2.0,

            0.0,

            -FIELD_HALF
                + grid_z as f32 * CELL
                + CELL / 2.0,

        )
    }



    fn near_seed_station(&self) -> bool {

        self.farmer
            .position
            .distance(SEED_STATION_POS)
            < 2.5
    }
}

impl Game {


    fn spawn_dirt(&mut self, pos: Vec3) {

        for _ in 0..4 {

            self.dirt.push(DirtParticle {

                position: pos + vec3(0.0, 0.15, 0.0),

                velocity: vec3(
                    rand::gen_range(-2.0, 2.0),
                    rand::gen_range(3.0, 6.0),
                    rand::gen_range(-2.0, 2.0),
                ),

                life: rand::gen_range(0.6, 1.2),
            });
        }
    }



    fn plow_cell(&mut self, gx: usize, gz: usize) {

        if self.field[gx][gz] == CellState::Grass {

            self.field[gx][gz] = CellState::Plowed;

            self.spawn_dirt(Self::cell_center(gx, gz));
        }
    }



    fn plant_cell(&mut self, gx: usize, gz: usize) -> bool {


        if self.seeds == 0 {
            return false;
        }


        if self.field[gx][gz] == CellState::Plowed {


            self.field[gx][gz] =
                CellState::Planted {
                    growth: 0.0
                };


            self.seeds -= 1;


            return true;
        }


        false
    }




    fn harvest_cell(&mut self, gx: usize, gz: usize) -> bool {


        if let CellState::Planted { growth } = self.field[gx][gz] {


            if growth >= 1.0 {


                self.field[gx][gz] = CellState::Plowed;


                self.potatoes += 1;


                self.spawn_dirt(
                    Self::cell_center(gx, gz)
                );


                return true;
            }
        }


        false
    }




    // Convert potatoes into seeds at station

    fn convert_potatoes(&mut self) -> bool {


        if self.potatoes == 0 {
            return false;
        }


        let converted =
            self.potatoes * POTATO_TO_SEED;


        self.seeds += converted;


        self.potatoes = 0;


        true
    }





    fn at_cell_center(&self) -> bool {

        let target =
            Self::cell_center(
                self.farmer.grid_x,
                self.farmer.grid_z
            );


        self.farmer.position.distance(target) < 0.08
    }





    fn try_step(&mut self, dx: i32, dz: i32) -> bool {


        let nx =
            self.farmer.grid_x as i32 + dx;


        let nz =
            self.farmer.grid_z as i32 + dz;



        if nx < 0
            || nz < 0
            || nx >= GRID as i32
            || nz >= GRID as i32
        {
            return false;
        }



        self.farmer.grid_x = nx as usize;

        self.farmer.grid_z = nz as usize;



        self.farmer.facing =
            (dx as f32).atan2(dz as f32);



        self.farmer.step_cooldown =
            STEP_REPEAT;



        true
    }





    fn handle_movement_input(&mut self) {


        if !self.at_cell_center() {
            return;
        }



        if self.farmer.step_cooldown > 0.0 {
            return;
        }



        let mut dx = 0;

        let mut dz = 0;



        if is_key_down(KeyCode::W)
            || is_key_down(KeyCode::Up)
        {
            dz -= 1;
        }



        if is_key_down(KeyCode::S)
            || is_key_down(KeyCode::Down)
        {
            dz += 1;
        }



        if is_key_down(KeyCode::A)
            || is_key_down(KeyCode::Left)
        {
            dx -= 1;
        }



        if is_key_down(KeyCode::D)
            || is_key_down(KeyCode::Right)
        {
            dx += 1;
        }



        // Prevent diagonal movement

        if dx != 0 && dz != 0 {
            dx = 0;
        }



        if dx != 0 || dz != 0 {

            self.try_step(dx, dz);
        }
    }






    fn update(&mut self, dt: f32) {


        self.action_cooldown =
            (self.action_cooldown - dt)
            .max(0.0);



        self.farmer.step_cooldown =
            (self.farmer.step_cooldown - dt)
            .max(0.0);



        self.handle_movement_input();




        let target =
            Self::cell_center(
                self.farmer.grid_x,
                self.farmer.grid_z
            );



        self.farmer.position =
            self.farmer.position.lerp(
                target,
                dt * MOVE_LERP
            );



        self.farmer.position.y = 0.0;




        self.farmer.plowing =
            is_key_down(KeyCode::Space);





        if self.farmer.plowing
            && self.at_cell_center()
        {

            self.plow_cell(
                self.farmer.grid_x,
                self.farmer.grid_z
            );
        }






        if is_key_pressed(KeyCode::E)
            && self.action_cooldown <= 0.0
        {



            // Station interaction

            if self.near_seed_station() {


                if self.convert_potatoes() {

                    self.action_cooldown = 0.5;
                }


            }



            // Field interaction

            else if self.at_cell_center() {


                let gx =
                    self.farmer.grid_x;


                let gz =
                    self.farmer.grid_z;



                let planted =
                    self.plant_cell(gx, gz);



                let harvested =
                    if !planted {

                        self.harvest_cell(
                            gx,
                            gz
                        )

                    } else {
                        false
                    };



                if planted || harvested {

                    self.action_cooldown = 0.25;
                }
            }
        }






        // Camera

        let desired_target =
            self.farmer.position
            + vec3(0.0, 0.8, 0.0);



        let t =
            1.0
            - (-CAM_SMOOTH * dt).exp();



        self.camera.target =
            self.camera.target.lerp(
                desired_target,
                t
            );



        self.camera.position =
            self.camera.target
            + CAM_OFFSET;







        // Grow crops

        for row in self.field.iter_mut() {

            for cell in row.iter_mut() {


                if let CellState::Planted { growth } = cell {


                    *growth =
                        (*growth
                        + dt / GROW_TIME)
                        .min(1.0);
                }
            }
        }






        // Dirt physics

        for particle in self.dirt.iter_mut() {


            particle.velocity.y -=
                12.0 * dt;



            particle.position +=
                particle.velocity * dt;



            particle.life -= dt;
        }



        self.dirt.retain(
            |p|
            p.life > 0.0
            && p.position.y > -0.5
        );
    }
}

fn soil_color(state: CellState) -> Color {

    match state {

        CellState::Grass =>
            Color::from_rgba(70, 130, 55, 255),


        CellState::Plowed =>
            Color::from_rgba(95, 60, 30, 255),


        CellState::Planted { .. } =>
            Color::from_rgba(80, 50, 25, 255),
    }
}





fn draw_field(game: &Game) {


    for gx in 0..GRID {

        for gz in 0..GRID {


            let center =
                Game::cell_center(
                    gx,
                    gz
                );



            let state =
                game.field[gx][gz];



            draw_cube(

                center + vec3(0.0, -0.08, 0.0),

                vec3(
                    CELL * 0.96,
                    0.16,
                    CELL * 0.96
                ),

                None,

                soil_color(state),
            );




            if gx != game.farmer.grid_x
                || gz != game.farmer.grid_z
            {

                draw_cube_wires(

                    center + vec3(0.0, 0.01, 0.0),

                    vec3(
                        CELL * 0.98,
                        0.02,
                        CELL * 0.98
                    ),

                    Color::from_rgba(
                        255,
                        255,
                        255,
                        40
                    ),
                );
            }




            if let CellState::Planted { growth } = state {

                draw_potato_plant(
                    center,
                    growth
                );
            }
        }
    }
}






fn draw_potato_plant(center: Vec3, growth: f32) {


    let height =
        0.15 + growth * 1.1;



    draw_cylinder(

        center
            + vec3(
                0.0,
                height / 2.0 + 0.08,
                0.0
            ),


        0.06,

        0.06,

        height,


        None,


        Color::from_rgba(
            45,
            120,
            40,
            255
        ),
    );




    if growth > 0.25 {


        draw_sphere(

            center
                + vec3(
                    -0.2,
                    0.45 + growth * 0.4,
                    0.0
                ),

            0.15 + growth * 0.15,

            None,


            Color::from_rgba(
                55,
                160,
                50,
                255
            ),
        );



        draw_sphere(

            center
                + vec3(
                    0.2,
                    0.5 + growth * 0.4,
                    0.0
                ),

            0.15 + growth * 0.15,

            None,


            Color::from_rgba(
                50,
                140,
                45,
                255
            ),
        );
    }






    if growth > 0.85 {


        let potato =
            Color::from_rgba(
                170,
                125,
                70,
                255
            );



        draw_sphere(

            center + vec3(
                -0.15,
                0.12,
                0.12
            ),

            0.12,

            None,

            potato,
        );



        draw_sphere(

            center + vec3(
                0.15,
                0.1,
                -0.1
            ),

            0.11,

            None,

            potato,
        );
    }
}








fn draw_farmer_3d(farmer: &Farmer) {


    let pos =
        farmer.position;



    let forward =
        vec3(
            farmer.facing.sin(),
            0.0,
            farmer.facing.cos()
        );



    let right =
        vec3(
            forward.z,
            0.0,
            -forward.x
        );





    // Legs

    draw_cylinder(

        pos
            + right * 0.12
            + vec3(0.0,0.25,0.0),

        0.09,

        0.09,

        0.5,

        None,

        DARKBLUE
    );



    draw_cylinder(

        pos
            - right * 0.12
            + vec3(0.0,0.25,0.0),

        0.09,

        0.09,

        0.5,

        None,

        DARKBLUE
    );






    // Body shirt

    draw_cylinder(

        pos + vec3(0.0,0.75,0.0),

        0.28,

        0.24,

        0.8,

        None,

        Color::from_rgba(
            120,
            70,
            35,
            255
        ),
    );







    // Head (African-inspired darker skin tone)

    draw_sphere(

        pos + vec3(0.0,1.35,0.0),

        0.25,

        None,

        Color::from_rgba(
            95,
            55,
            35,
            255
        ),
    );






    // Nose

    draw_sphere(

        pos
            + forward * 0.22
            + vec3(0.0,1.35,0.0),

        0.04,

        None,

        Color::from_rgba(
            80,
            45,
            25,
            255
        ),
    );






    // Arms

    draw_cylinder(

        pos
            + right * 0.35
            + vec3(0.0,0.85,0.0),

        0.07,

        0.07,

        0.55,

        None,

        Color::from_rgba(
            95,
            55,
            35,
            255
        ),
    );






    // Straw Chinese farmer hat

    draw_cylinder(
        pos + vec3(0.0, 1.65, 0.0),
    
        0.15,
        0.48,
    
        0.35,
    
        None,
    
        Color::from_rgba(
            210,
            170,
            80,
            255
        ),
    );


    draw_cylinder(

        pos + vec3(0.0,1.52,0.0),

        0.5,

        0.5,

        0.06,

        None,

        Color::from_rgba(
            230,
            190,
            100,
            255
        ),
    );






    // Tool handle

    draw_line_3d(

        pos
            + right * 0.35
            + vec3(0.0,0.8,0.0),

        pos
            + forward * 0.8
            + vec3(0.0,0.3,0.0),

        DARKGRAY
    );





    if farmer.plowing {


        draw_cube(

            pos
                + forward * 0.9
                + vec3(0.0,0.15,0.0),

            vec3(
                0.5,
                0.12,
                0.35
            ),

            None,

            Color::from_rgba(
                60,
                60,
                65,
                255
            ),
        );
    }
}







fn draw_seed_station() {


    // Base

    draw_cube(

        SEED_STATION_POS
            + vec3(0.0,0.3,0.0),

        vec3(
            1.5,
            0.6,
            1.5
        ),

        None,

        Color::from_rgba(
            150,
            100,
            45,
            255
        ),
    );



    // Machine top

    draw_cylinder(

        SEED_STATION_POS
            + vec3(0.0,0.9,0.0),

        0.45,

        0.45,

        0.7,

        None,

        GOLD
    );



    draw_text(

        "SEED MILL",

        screen_width()/2.0 - 50.0,

        50.0,

        20.0,

        DARKGREEN
    );
}

fn draw_current_tile_marker(game: &Game) {

    let center =
        Game::cell_center(
            game.farmer.grid_x,
            game.farmer.grid_z
        );


    draw_cube_wires(

        center + vec3(0.0,0.5,0.0),

        vec3(
            CELL * 0.92,
            1.0,
            CELL * 0.92
        ),

        YELLOW
    );
}







fn draw_scene(game: &Game) {


    clear_background(
        Color::from_rgba(
            135,
            200,
            245,
            255
        )
    );



    set_camera(&Camera3D {


        position:
            game.camera.position,


        up:
            vec3(
                0.0,
                1.0,
                0.0
            ),


        target:
            game.camera.target,


        fovy:
            25.0,


        projection:
            Projection::Orthographics,


        ..Default::default()

    });





    draw_grid(

        GRID as u32,

        CELL,

        Color::from_rgba(
            40,
            40,
            40,
            80
        ),

        GRAY
    );



    draw_field(game);



    // Dirt particles

    for particle in &game.dirt {


        let alpha =
            (particle.life * 255.0)
            as u8;



        draw_sphere(

            particle.position,

            0.08,

            None,

            Color::from_rgba(
                120,
                75,
                35,
                alpha
            ),
        );
    }





    draw_seed_station();



    draw_current_tile_marker(game);



    draw_farmer_3d(
        &game.farmer
    );



    set_default_camera();
}








fn draw_hud(game: &Game) {


    draw_text(

        "WASD / Arrows - Move",

        16.0,

        28.0,

        22.0,

        BLACK
    );



    draw_text(

        "SPACE - Plow",

        16.0,

        54.0,

        22.0,

        BLACK
    );



    draw_text(

        "E - Plant / Harvest / Use station",

        16.0,

        80.0,

        22.0,

        BLACK
    );



    draw_text(

        &format!(
            "Seeds: {}   Potatoes: {}",
            game.seeds,
            game.potatoes
        ),

        16.0,

        112.0,

        26.0,

        DARKBLUE
    );




    if game.near_seed_station()
    {

        draw_text(

            "Seed Mill: Press E to convert potatoes",

            16.0,

            150.0,

            24.0,

            DARKGREEN
        );
    }




    let gx =
        game.farmer.grid_x;

    let gz =
        game.farmer.grid_z;



    let status =
        match game.field[gx][gz]
        {


            CellState::Grass =>
                "Grass - hold SPACE to plow"
                .to_string(),



            CellState::Plowed =>
                "Plowed - press E to plant"
                .to_string(),



            CellState::Planted { growth }
                if growth >= 1.0 =>

                "Ready - press E to harvest"
                .to_string(),



            CellState::Planted { growth } =>

                format!(
                    "Growing... {}%",
                    (growth * 100.0) as u32
                ),
        };



    draw_text(

        &status,

        16.0,

        180.0,

        22.0,

        DARKGREEN
    );
}









#[macroquad::main("African Nigga Potatoe Farmer")]
async fn main() {


    let mut game =
        Game::new();



    loop {


        let dt =
            get_frame_time();



        game.update(dt);



        draw_scene(
            &game
        );



        draw_hud(
            &game
        );



        next_frame().await;
    }
}