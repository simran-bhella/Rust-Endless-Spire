//! Endless Spire Rougelike Game 
//! **(TEST)**
//! Simranjit Bhella, Bruce Truong, John Asencio-Giron, Shihao Li, and Chentao Ma 2023

use ggez::conf;
use ggez::graphics::Rect;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{event, graphics, Context, GameResult};
use rand::Rng;
use std::{env, path};

/// Wall and stair_image locations for each map.
struct Levels {
    map_test: Vec<(i32, i32)>,

    stair_test:Vec<(i32, i32)>,

}

impl Levels {
    /// Creates the Levels object which stores the locations of the walls and stairs.
    fn new() -> Self {
        Levels {
            map_test: vec![(2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3), (20, 3), (21, 3), (22, 3), (23, 3), (24, 3), (25, 3), (26, 3), (27, 3), (28, 3), (29, 3), (30, 3), (31, 3), (32, 3), (33, 3), (34, 3), (35, 3), (36, 3), (37, 3), (38, 3), (39, 3), (40, 3), (41, 3), (42, 3), (43, 3), (44, 3), (45, 3), (46, 3), (47, 3), (48, 3), (49, 3), (50, 3), (51, 3), (52, 3), (53, 3), (54, 3), (55, 3), (56, 3), (57, 3), (58, 3), (59, 3), (60, 3), (61, 3), (62, 3), (63, 3), (2, 4), (14, 4), (37, 4), (63, 4), (2, 5), (14, 5), (37, 5), (63, 5), (2, 6), (14, 6), (37, 6), (63, 6), (2, 7), (14, 7), (19, 7), (20, 7), (22, 7), (23, 7), (28, 7), (29, 7), (31, 7), (32, 7), (37, 7), (63, 7), (2, 8), (14, 8), (19, 8), (23, 8), (28, 8), (32, 8), (37, 8), (63, 8), (2, 9), (14, 9), (37, 9), (63, 9), (2, 10), (14, 10), (19, 10), (23, 10), (28, 10), (32, 10), (37, 10), (63, 10), (2, 11), (19, 11), (20, 11), (22, 11), (23, 11), (28, 11), (29, 11), (31, 11), (32, 11), (63, 11), (2, 12), (14, 12), (37, 12), (63, 12), (2, 13), (63, 13), (2, 14), (14, 14), (37, 14), (63, 14), (2, 15), (63, 15), (2, 16), (14, 16), (37, 16), (63, 16), (2, 17), (63, 17), (2, 18), (14, 18), (37, 18), (63, 18), (2, 19), (19, 19), (20, 19), (22, 19), (23, 19), (28, 19), (29, 19), (31, 19), (32, 19), (63, 19), (2, 20), (14, 20), (19, 20), (23, 20), (28, 20), (32, 20), (37, 20), (63, 20), (2, 21), (14, 21), (37, 21), (63, 21), (2, 22), (14, 22), (19, 22), (23, 22), (28, 22), (32, 22), (37, 22), (63, 22), (2, 23), (14, 23), (19, 23), (20,23), (22, 23), (23, 23), (28, 23), (29, 23), (31, 23), (32, 23), (37, 23), (63, 23), (2, 24), (14, 24), (37, 24), (63, 24), (2, 25), (14, 25), (37, 25), (63, 25), (2, 26), (14, 26), (37, 26), (63, 26), (2, 27), (3, 27), (4, 27), (5, 27), (6, 27), (7, 27), (9, 27), (10, 27), (11, 27), (12, 27), (13, 27), (14, 27), (15, 27), (16, 27), (17, 27), (18, 27), (19, 27), (20, 27), (21, 27), (22, 27), (23, 27), (24, 27), (25, 27), (26, 27), (27, 27), (28, 27), (29, 27), (30, 27), (31, 27), (32, 27), (33, 27), (34, 27), (35, 27), (36, 27), (37, 27), (38, 27), (39, 27), (40, 27), (41, 27), (42, 27), (43, 27), (44, 27), (45, 27), (46, 27), (47, 27), (48, 27), (49, 27), (50, 27), (51, 27), (52, 27), (53, 27), (54, 27), (55, 27), (56, 27), (57, 27), (58, 27), (59, 27), (60, 27), (61, 27), (62, 27), (63, 27)],
            
            stair_test:vec![(62,15)],

        }
    }
}

/// Keep track of locations of entities on a defined grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    /// Takes in an x and y coord and returns a new GridPosition object
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}

/// Keep track of information relevent to the player entity.
struct Player {
    /// Player Health
    health: f32,

    /// Direction player is facing
    dir: i16,

    /// Player location on grid
    pos: GridPosition,
}

impl Player {
    /// Create new Player
    pub fn new() -> Self {
        Player {
            health: 3.0,
            pos: GridPosition::new(3, 13),
            dir: 0,
        }
    }

    /// This function takes in the 'canvas' (the screen), a GridPosition, and an image.
    /// It will then draw the player on the canvas using the given image on the given GridPosition.
    /// This function also takes into account the player's direction, and draws them facing in the direction last moved.
    fn draw(&self, canvas: &mut graphics::Canvas, pos: GridPosition, pic: &mut graphics::Image) {
        let dst2 = ggez::glam::Vec2::new(pos.x as f32 * 25.0 - 27.5, pos.y as f32 * 25.0 + 93.0);

        match self.dir {
            0 => {
                let source_rect = Rect::new(0.35, 0.5, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            1 => {
                let source_rect = Rect::new(0.0, 0.5, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            2 => {
                let source_rect = Rect::new(0.7, 0.5, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            3 => {
                let source_rect = Rect::new(0.35, 0.0, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            4 => {
                let source_rect = Rect::new(0.7, 0.0, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            5 => {
                let source_rect = Rect::new(0.0, 0.0, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            6 => {
                let source_rect = Rect::new(0.35, 0.75, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            7 => {
                let source_rect = Rect::new(0.7, 0.75, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            8 => {
                let source_rect = Rect::new(0.0, 0.75, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            9 => {
                let source_rect = Rect::new(0.35, 0.25, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            10 => {
                let source_rect = Rect::new(0.7, 0.25, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            11 => {
                let source_rect = Rect::new(0.0, 0.25, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            _ => {}
        }
    }
}

/// Keep track of information relevent to the enemy entity.
struct Enemy {
    /// Enemy location on grid
    pos: GridPosition,
}

impl Enemy {
    /// Create new Enemy with passed in position. This allows the enemy's starting position to be decided during creation
    pub fn new(x: i16, y: i16) -> Self {
        Enemy {
            /// Here we use GridPosition::New() in order to turn the (x,y) coords into a GridPosition object.
            pos: GridPosition::new(x, y),
        }
    }

    /// This function will take in the canvas, a GridPosition, a Direction, and an Image.
    /// It will then draw the player on the canvas using the given image, GridPosition, and Direction.
    fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        pos: GridPosition,
        dir: i16,
        pic: &mut graphics::Image,
    ) {
        let dst2 = ggez::glam::Vec2::new(pos.x as f32 * 25.0 - 27.5, pos.y as f32 * 25.0 + 93.0);

        match dir {
            0 => {
                let source_rect = Rect::new(0.35, 0.5, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            1 => {
                let source_rect = Rect::new(0.0, 0.5, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            2 => {
                let source_rect = Rect::new(0.7, 0.5, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            3 => {
                let source_rect = Rect::new(0.35, 0.0, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            4 => {
                let source_rect = Rect::new(0.7, 0.0, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            5 => {
                let source_rect = Rect::new(0.0, 0.0, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            6 => {
                let source_rect = Rect::new(0.35, 0.75, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            7 => {
                let source_rect = Rect::new(0.7, 0.75, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            8 => {
                let source_rect = Rect::new(0.0, 0.75, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            9 => {
                let source_rect = Rect::new(0.35, 0.25, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            10 => {
                let source_rect = Rect::new(0.7, 0.25, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            11 => {
                let source_rect = Rect::new(0.0, 0.25, 0.3, 0.26);
                canvas.draw(pic, graphics::DrawParam::new().dest(dst2).src(source_rect));
            }
            _ => {}
        }
    }

    /// This function returns a random GridPosition for the enemy to move to.
    /// By using rand, we are able to randomly select one of the 4 directions and have the enemy move there.
    fn move_pos(&self) -> GridPosition {
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(1..5);

        match dir {
            1 => GridPosition::new(self.pos.x, self.pos.y - 1),
            2 => GridPosition::new(self.pos.x, self.pos.y + 1),
            3 => GridPosition::new(self.pos.x - 1, self.pos.y),
            4 => GridPosition::new(self.pos.x + 1, self.pos.y),
            _ => self.pos,
        }
    }
}

/// This struct allows us to toggle full screen.
struct WindowSettings {
    toggle_fullscreen: bool,
}

/// The MainState is the main driver for the game.
/// This struct holds all the relevent objects needed to run the game.
struct MainState {
    /// Settings for the window.
    window_settings: WindowSettings,

    /// Image used for the background.
    background_img: graphics::Image,

    /// Image used for the floor.
    floor_img: graphics::Image,

    /// Image used for the wall.
    wall_img: graphics::Image,

    /// Image used for player.
    sprite: graphics::Image,

    /// Image used for Enemy type 1
    enemy_sprite1: graphics::Image,

    /// Image used for Enemy type 2
    enemy_sprite2: graphics::Image,

    /// Image used for stairs
    stair_image: graphics::Image,

    /// The player.
    player: Player,

    /// The enemy vectors. Enemies are seperated into 5 vectors for the 5 maps.
    enemies_part2: Vec<Enemy>,
    enemies_part3: Vec<Enemy>,


    /// The levels.
    levels: Levels,

    /// Booleans used to determine the current state of the game.
    start_screen: bool,

}

impl MainState {
    /// Create new MainState with inital state of the game.
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let background_img = graphics::Image::from_path(ctx, "/background.png")?;
        let floor_img = graphics::Image::from_path(ctx, "/shot.png")?;
        let wall_img = graphics::Image::from_path(ctx, "/tile.png")?;
        let player_img = graphics::Image::from_path(ctx, "/warrior_m.png")?;
        let enemy_image1 = graphics::Image::from_path(ctx, "/mage_m.png")?;
        let enemy_image2 = graphics::Image::from_path(ctx, "/ninja_m.png")?;
        let stair_image = graphics::Image::from_path(ctx, "/stair.png")?;
        let s = MainState {
            background_img,
            floor_img,
            wall_img,
            sprite: player_img,
            stair_image,
            enemy_sprite1: enemy_image1,
            enemy_sprite2: enemy_image2,
            start_screen: true,

            player: Player::new(),
            enemies_part2: vec![
                Enemy::new(21, 9),
                Enemy::new(21, 21),
                Enemy::new(30, 9),
                Enemy::new(30, 21),
            ],
            enemies_part3: vec![
                Enemy::new(51, 6),
                Enemy::new(45, 19),
                Enemy::new(46, 25),
                Enemy::new(58, 21),
            ],
            window_settings: WindowSettings {
                toggle_fullscreen: false,
            },
            levels: Levels::new(),
        };
        Ok(s)
    }

    /// This function checks a GridPosition against the walls on the map. It returns false if the position is a wall, otherwise returns true.
    /// This is used to ensure that entity's do not move into walls.
    fn check_bounds(&self, pos: GridPosition) -> bool {
        let a = pos.x;
        let b = pos.y;

        if self.levels.map_test.contains(&(a.into(), b.into())) {
            return false;
        }

        true
    }

    /// This function checks a GridPosition against the stairs on the map. It returns true if the position is a stair, otherwise returns false.
    /// This is used to faciliate moving up to the next level.
    fn check_stairs(&self, pos: GridPosition) -> bool {
        let a = pos.x;
        let b = pos.y;
        
        if self.levels.stair_test.contains(&(a.into(), b.into())) {
            return true;
        }
        
        false
    }

    /// This function is used to move all enemies within an enemy vector.
    /// Depending on which map is active, it will iterate through the appropraite vector of enemies
    /// and have each one call its .move_pos() function for a random move.
    /// It also verifies that the enemy is not moving into a wall by using the check_bounds() function.
    fn move_enemy(&mut self) {
        let mut i = 0;
        while i < self.enemies_part3.len() {
            let enemy_pos = self.enemies_part3[i].move_pos();
            if MainState::check_bounds(self, enemy_pos) {
                self.enemies_part3[i].pos = enemy_pos;
            }
            i += 1;
        }
        
    }

    /// This function is used to check if the player's position is overlapping with any of the enemy positions.
    /// Depending on which map is active, it will iterate through the appropraite vector of enemies collecting the enemy positions.
    /// It will then itterate through each position to check if the player's position matches any of the enemy positions.
    /// If it does, then the player's health will go down by 1. If the player's health reaches zero, it sets the dead bool to true.
    fn check_collision(&mut self) {
        
        let enemy_positions1: Vec<GridPosition> =
            self.enemies_part2.iter().map(|enemy| enemy.pos).collect();
        let enemy_positions2: Vec<GridPosition> =
            self.enemies_part3.iter().map(|enemy| enemy.pos).collect();
        for enemy_pos in enemy_positions1 {
            if self.player.pos == enemy_pos {
                self.player.health -= 1.0;
            }
        }
        for enemy_pos in enemy_positions2 {
            if self.player.pos == enemy_pos {
                self.player.health -= 1.0;
            }
        }
    }
}

/// The EventHandler is called automatically by ggez when events occur.
impl event::EventHandler<ggez::GameError> for MainState {
    /// The update function will occur on every frame before it's drawn.
    /// This uses ggez's built in timer to decide when to update.
    /// This function is where the values are reset if the player is dead.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let start_press = &ctx.keyboard;
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if start_press.is_key_pressed(KeyCode::Space) {
                self.start_screen = false;
                self.window_settings.toggle_fullscreen = false;
            }
        }
        Ok(())
    }

    /// The draw function is where the game state is displayed on the screen.
    /// Within this function, the start screen, win screen, and lose screen can all be drawn depending on
    /// the state of the game managed by the boolean's in MainState. This function will also draw out the map
    /// and enemies depending on what level we are on. It will also draw out the player.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.start_screen {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));
            let dst = ggez::glam::Vec2::new(0.0, -120.0);
            let tit = ggez::glam::Vec2::new(170.0, 225.0);
            let text = ggez::glam::Vec2::new(220.0, 375.0);

            canvas.draw(&self.background_img, graphics::DrawParam::new().dest(dst));
            canvas.draw(
                graphics::Text::new("Endless Spire (Test version)").set_scale(75.),
                graphics::DrawParam::default().dest(tit),
            );
            canvas.draw(
                graphics::Text::new("Press Space to Continue").set_scale(35.),
                graphics::DrawParam::default().dest(text),
            );

            canvas.finish(ctx)?;
        } else {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));

            let health_bar = ggez::glam::Vec2::new(50.0, 850.0);
            let text = format!("Health: {}", self.player.health);
            canvas.draw(
                graphics::Text::new(text).set_scale(35.),
                graphics::DrawParam::default().dest(health_bar),
            );

            let scale1 = ggez::glam::Vec2::new(0.125, 0.125);
            let scale2 = ggez::glam::Vec2::new(5.0, 5.0);
            let scale3 = ggez::glam::Vec2::new(0.1, 0.025);

            for x in 2..64 {
                for y in 3..28 {
                    let dst =
                        ggez::glam::Vec2::new(x as f32 * 25.0 - 25.0, y as f32 * 25.0 + 100.0);
                    let dst2 = ggez::glam::Vec2::new(
                        x as f32 * 25.0 - 27.5 - 25.0,
                        y as f32 * 25.0 - 27.5 + 100.0,
                    );
                    let dst3 =
                        ggez::glam::Vec2::new(x as f32 * 25.0 - 75.0, y as f32 * 25.0 + 100.0);

                    
                        if self.levels.map_test.contains(&(x, y)) {
                            canvas.draw(
                                &self.wall_img,
                                graphics::DrawParam::new().dest(dst).scale(scale1),
                            );
                        } else if self.levels.stair_test.contains(&(x, y)) {
                            canvas.draw(
                                &self.stair_image,
                                graphics::DrawParam::new().dest(dst3).scale(scale3),
                            );
                        } else {
                            canvas.draw(
                                &self.floor_img,
                                graphics::DrawParam::new().dest(dst2).scale(scale2),
                            );
                        }
                    
                }
            }

            self.player
                .draw(&mut canvas, self.player.pos, &mut self.sprite);

            
                for enemy in &self.enemies_part2 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite1,
                    );
                }
          
                for enemy in &self.enemies_part3 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite2,
                    );
                }
            
            canvas.finish(ctx)?;
            ggez::timer::yield_now();
        }
        Ok(())
    }

    /// This function gets called if a key is pressed. It is responsible for the player's movement.
    /// Within this function, check_bounds() and check_stairs() are both called on the player's new GridPosition.
    /// If check_bounds returns true, then the player's position will be updated, otherwise it will stay the same.
    /// If check_stairs() returns true, then the map will be updated to the next level and the player's position will be set to the new start point.
    /// Whenever a key down event occurs, the enemies' positions are also updated via the move_enemy() function.
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Up) => {
                let pos = GridPosition::new(self.player.pos.x, self.player.pos.y - 1);
                if self.player.dir >= 5 || self.player.dir <= 2 {
                    self.player.dir = 3;
                } else {
                    self.player.dir += 1;
                }
                if MainState::check_bounds(self, pos) {
                    self.check_collision();
                    self.player.pos = pos;
                }

                if MainState::check_stairs(self, pos) {
                    
                    self.player.pos = GridPosition::new(5, 15);
                    
                }
            }

            Some(KeyCode::Down) => {
                let pos = GridPosition::new(self.player.pos.x, self.player.pos.y + 1);
                if self.player.dir < 3 {
                    self.player.dir += 1;
                }
                if self.player.dir >= 3 {
                    self.player.dir = 0;
                }
                if MainState::check_bounds(self, pos) {
                    self.check_collision();

                    self.player.pos = pos;
                }

                if MainState::check_stairs(self, pos) {
                    
                    self.player.pos = GridPosition::new(5, 15);
                    
                }
            }

            Some(KeyCode::Left) => {
                let pos = GridPosition::new(self.player.pos.x - 1, self.player.pos.y);
                if self.player.dir >= 8 || self.player.dir <= 5 {
                    self.player.dir = 6;
                } else {
                    self.player.dir += 1;
                }
                if MainState::check_bounds(self, pos) {
                    self.check_collision();

                    self.player.pos = pos;
                }

                if MainState::check_stairs(self, pos) {
                    
                    self.player.pos = GridPosition::new(5, 15);
                    
                }
            }

            Some(KeyCode::Right) => {
                let pos = GridPosition::new(self.player.pos.x + 1, self.player.pos.y);
                if self.player.dir >= 11 || self.player.dir <= 8 {
                    self.player.dir = 9;
                } else {
                    self.player.dir += 1;
                }
                if MainState::check_bounds(self, pos) {
                    self.check_collision();
                    self.player.pos = pos;
                }

                if MainState::check_stairs(self, pos) {
                    
                    self.player.pos = GridPosition::new(5, 15);
                    
                }
            }
            _ => (),
        }
        self.move_enemy();

        Ok(())
    }
}

/// In the main function, the MainState is created and the game is run using event::run().
/// The ggez::ContextBuilder::new() function is called in order to set up configuration settings
/// like window title and dimendions, as well as resource path.
pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Endless Spire", "me")
        .add_resource_path(resource_dir)
        .window_mode(conf::WindowMode::default().dimensions(1600.0, 900.0));

    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Endless Spire");

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state)
}



//Unit test template
/* 
#[cfg(test)]
mod tests {
    use super::*;
    use ggez::{
        conf::{Conf, WindowMode},
        event::EventsLoop,
        ContextBuilder,
    };

    #[test]
    fn test_game() -> GameResult {
        // Create a mock or test-specific context
        let (mut ctx, event_loop) = create_test_context()?;
        
        // Initialize the game state
        let mut game = MainState::new(&mut ctx)?;

        // Run the game loop for a few frames
        for _ in 0..10 {
            update_game(&mut game)?;
            draw_game(&mut ctx, &game)?;
        }

        // Add assertions here to verify the expected behavior of the game

        // Example assertions:
        assert_eq!(game.level1, true);
        assert_eq!(game.player.pos, GridPosition::new(3, 4));
        // ...

        // Add more assertions to validate the behavior of the game

        Ok(())
    }

    // Helper function to create a mock or test-specific context
    fn create_test_context() -> GameResult<(ggez::Context, EventsLoop)> {
        let cb = ContextBuilder::new("Endless Spire", "me")
            .window_mode(WindowMode::default().dimensions(1600.0, 900.0));

        let (ctx, event_loop) = cb.build()?;
        Ok((ctx, event_loop))
    }

    // Helper function to update the game state
    fn update_game(game: &mut MainState) -> GameResult {
        // Implement the necessary game update logic
        // ...

        Ok(())
    }

    // Helper function to draw the game
    fn draw_game(ctx: &mut ggez::Context, game: &MainState) -> GameResult {
        // Implement the necessary game drawing logic
        // ...

        Ok(())
    }
}
*/