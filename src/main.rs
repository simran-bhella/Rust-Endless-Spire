//! Endless Spire Rougelike Game
//! Simranjit Bhella, Bruce Truong, John Asencio-Giron, Shihao Li, and Chentao Ma 2023
//!
//! This is the main file where the game is set up and run using ggez.

use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{conf, event, graphics, Context, GameResult};
use std::{env, path};

mod entities;
mod levels;

use entities::Enemy;
use entities::GridPosition;
use entities::Player;
use levels::Levels;

/// The MainState is the main driver for the game.
/// This struct holds all the relevent objects needed to run the game.
struct MainState {
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
    enemies1: Vec<Enemy>,
    enemies2: Vec<Enemy>,
    enemies3: Vec<Enemy>,
    enemies4: Vec<Enemy>,
    enemies5: Vec<Enemy>,

    /// The levels.
    levels: Levels,

    /// Booleans used to determine the current state of the game.
    start_screen: bool,
    level1: bool,
    level2: bool,
    level3: bool,
    level4: bool,
    level5: bool,
    dead: bool,
}

impl MainState {
    /// Create new MainState with objects prepared for the start of the game.
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
            level1: false,
            level2: false,
            level3: false,
            level4: false,
            level5: false,
            dead: false,
            player: Player::new(),
            enemies1: vec![
                Enemy::new(5, 19),
                Enemy::new(9, 24),
                Enemy::new(13, 13),
                Enemy::new(19, 11),
                Enemy::new(24, 18),
                Enemy::new(27, 24),
                Enemy::new(27, 6),
                Enemy::new(35, 10),
                Enemy::new(51, 6),
                Enemy::new(45, 19),
                Enemy::new(46, 25),
                Enemy::new(58, 21),
            ],
            enemies2: vec![
                Enemy::new(3, 10),
                Enemy::new(11, 16),
                Enemy::new(9, 10),
                Enemy::new(5, 21),
                Enemy::new(3, 26),
                Enemy::new(19, 20),
                Enemy::new(23, 22),
                Enemy::new(29, 14),
                Enemy::new(25, 9),
                Enemy::new(11, 6),
                Enemy::new(33, 19),
                Enemy::new(32, 10),
                Enemy::new(37, 17),
                Enemy::new(39, 11),
                Enemy::new(45, 24),
                Enemy::new(59, 16),
                Enemy::new(57, 8),
                Enemy::new(62, 20),
                Enemy::new(61, 14),
                Enemy::new(62, 4),
                Enemy::new(51, 8),
                Enemy::new(47, 11),
                Enemy::new(45, 10),
            ],
            enemies3: vec![
                Enemy::new(6, 7),
                Enemy::new(13, 5),
                Enemy::new(23, 8),
                Enemy::new(32, 7),
                Enemy::new(43, 7),
                Enemy::new(51, 7),
                Enemy::new(47, 8),
                Enemy::new(24, 14),
                Enemy::new(20, 16),
                Enemy::new(48, 15),
                Enemy::new(61, 10),
                Enemy::new(61, 22),
                Enemy::new(48, 23),
                Enemy::new(33, 22),
                Enemy::new(27, 23),
                Enemy::new(18, 23),
                Enemy::new(12, 23),
            ],
            enemies4: vec![
                Enemy::new(13, 23),
                Enemy::new(7, 15),
                Enemy::new(11, 7),
                Enemy::new(18, 18),
                Enemy::new(28, 22),
                Enemy::new(31, 13),
                Enemy::new(44, 25),
                Enemy::new(41, 15),
                Enemy::new(47, 7),
                Enemy::new(49, 51),
                Enemy::new(59, 20),
                Enemy::new(57, 8),
                Enemy::new(56, 12),
            ],
            enemies5: vec![
                Enemy::new(8, 15),
                Enemy::new(8, 19),
                Enemy::new(13, 18),
                Enemy::new(17, 15),
                Enemy::new(9, 24),
                Enemy::new(15, 8),
                Enemy::new(21, 16),
                Enemy::new(25, 16),
                Enemy::new(30, 16),
                Enemy::new(39, 20),
                Enemy::new(39, 11),
                Enemy::new(48, 22),
                Enemy::new(59, 23),
                Enemy::new(59, 8),
                Enemy::new(51, 9),
            ],
            levels: levels::Levels::new(),
        };
        Ok(s)
    }

    /// This function checks a GridPosition against the walls on the map. It returns false if the position is a wall, otherwise returns true.
    /// This is used to ensure that entity's do not move into walls.
    fn check_bounds(&self, pos: GridPosition) -> bool {
        let a = pos.x;
        let b = pos.y;
        if !self.level1 {
            if self.levels.map1.contains(&(a.into(), b.into())) {
                return false;
            }
        } else if !self.level2 {
            if self.levels.map2.contains(&(a.into(), b.into())) {
                return false;
            }
        } else if !self.level3 {
            if self.levels.map3.contains(&(a.into(), b.into())) {
                return false;
            }
        } else if !self.level4 {
            if self.levels.map4.contains(&(a.into(), b.into())) {
                return false;
            }
        } else if self.levels.map5.contains(&(a.into(), b.into())) {
            return false;
        }
        true
    }

    /// This function checks a GridPosition against the stairs on the map. It returns true if the position is a stair, otherwise returns false.
    /// This is used to faciliate moving up to the next level.
    fn check_stairs(&self, pos: GridPosition) -> bool {
        let a = pos.x;
        let b = pos.y;
        if !self.level1 {
            if self.levels.stair_12.contains(&(a.into(), b.into())) {
                return true;
            }
        } else if !self.level2 {
            if self.levels.stair_23.contains(&(a.into(), b.into())) {
                return true;
            }
        } else if !self.level3 {
            if self.levels.stair_34.contains(&(a.into(), b.into())) {
                return true;
            }
        } else if !self.level4 {
            if self.levels.stair_45.contains(&(a.into(), b.into())) {
                return true;
            }
        } else if self.levels.stair_end.contains(&(a.into(), b.into())) {
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
        if !self.level1 {
            while i < self.enemies1.len() {
                let enemy_pos = self.enemies1[i].move_pos();
                if MainState::check_bounds(self, enemy_pos) {
                    self.enemies1[i].pos = enemy_pos;
                }
                i += 1;
            }
        } else if !self.level2 {
            while i < self.enemies2.len() {
                let enemy_pos = self.enemies2[i].move_pos();
                if MainState::check_bounds(self, enemy_pos) {
                    self.enemies2[i].pos = enemy_pos;
                }
                i += 1;
            }
        } else if !self.level3 {
            while i < self.enemies3.len() {
                let enemy_pos = self.enemies3[i].move_pos();
                if MainState::check_bounds(self, enemy_pos) {
                    self.enemies3[i].pos = enemy_pos;
                }
                i += 1;
            }
        } else if !self.level4 {
            while i < self.enemies4.len() {
                let enemy_pos = self.enemies4[i].move_pos();
                if MainState::check_bounds(self, enemy_pos) {
                    self.enemies4[i].pos = enemy_pos;
                }
                i += 1;
            }
        } else {
            while i < self.enemies5.len() {
                let enemy_pos = self.enemies5[i].move_pos();
                if MainState::check_bounds(self, enemy_pos) {
                    self.enemies5[i].pos = enemy_pos;
                }
                i += 1;
            }
        }
    }

    /// This function is used to check if the player's position is overlapping with any of the enemy positions.
    /// Depending on which map is active, it will iterate through the appropraite vector of enemies collecting the enemy positions.
    /// It will then itterate through each position to check if the player's position matches any of the enemy positions.
    /// If it does, then the player's health will go down by 1. If the player's health reaches zero, it sets the dead bool to true.
    fn check_collision(&mut self) {
        if !self.level1 {
            let enemy_positions: Vec<GridPosition> =
                self.enemies1.iter().map(|enemy| enemy.pos).collect();
            for enemy_pos in enemy_positions {
                if self.player.pos == enemy_pos {
                    self.player.health -= 1.0;
                }
            }
        } else if !self.level2 {
            let enemy_positions: Vec<GridPosition> =
                self.enemies2.iter().map(|enemy| enemy.pos).collect();
            for enemy_pos in enemy_positions {
                if self.player.pos == enemy_pos {
                    self.player.health -= 1.0;
                }
            }
        } else if !self.level3 {
            let enemy_positions: Vec<GridPosition> =
                self.enemies3.iter().map(|enemy| enemy.pos).collect();
            for enemy_pos in enemy_positions {
                if self.player.pos == enemy_pos {
                    self.player.health -= 1.0;
                }
            }
        } else if !self.level4 {
            let enemy_positions: Vec<GridPosition> =
                self.enemies4.iter().map(|enemy| enemy.pos).collect();
            for enemy_pos in enemy_positions {
                if self.player.pos == enemy_pos {
                    self.player.health -= 1.0;
                }
            }
        } else {
            let enemy_positions: Vec<GridPosition> =
                self.enemies5.iter().map(|enemy| enemy.pos).collect();
            for enemy_pos in enemy_positions {
                if self.player.pos == enemy_pos {
                    self.player.health -= 1.0;
                }
            }
        }
        if self.player.health < 1.0 {
            self.dead = true;
        }
    }
}

/// The EventHandler is called automatically by ggez when events occur.
impl event::EventHandler<ggez::GameError> for MainState {
    /// The update function will occur on every frame before it's drawn.
    /// This uses ggez's built in timer to decide when to update.
    /// This function is where the values are reset if the player is dead or wins.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let start_press = &ctx.keyboard;
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if start_press.is_key_pressed(KeyCode::Space) {
                self.start_screen = false;
                if self.level5 || self.dead {
                    self.dead = false;
                    self.level1 = false;
                    self.level2 = false;
                    self.level3 = false;
                    self.level4 = false;
                    self.level5 = false;
                    self.player.pos = GridPosition::new(3, 13);
                    self.player.health = 3.0;
                }
            }
        }
        Ok(())
    }

    /// The draw function is where the game state is displayed on the screen.
    /// Within this function, the start screen, win screen, and lose screen can all be drawn depending on
    /// the state of the game managed by the boolean's in MainState. This function will also draw out the map
    /// and enemies depending on what level we are on. It will also draw out the player.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let bg = ggez::glam::Vec2::new(0.0, 0.0);
        let text1 = ggez::glam::Vec2::new(170.0, 225.0);
        let text2 = ggez::glam::Vec2::new(220.0, 375.0);

        if self.start_screen {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));

            canvas.draw(&self.background_img, graphics::DrawParam::new().dest(bg));
            canvas.draw(
                graphics::Text::new("Endless Spire").set_scale(75.),
                graphics::DrawParam::default().dest(text1),
            );
            canvas.draw(
                graphics::Text::new("Press Space to Continue").set_scale(35.),
                graphics::DrawParam::default().dest(text2),
            );

            canvas.finish(ctx)?;
        } else if self.level5 {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));
            canvas.draw(&self.background_img, graphics::DrawParam::new().dest(bg));
            canvas.draw(
                graphics::Text::new("Win!").set_scale(75.),
                graphics::DrawParam::default().dest(text1),
            );
            canvas.draw(
                graphics::Text::new("Press Space to Restart").set_scale(35.),
                graphics::DrawParam::default().dest(text2),
            );

            canvas.finish(ctx)?;
        } else if self.dead {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));
            canvas.draw(&self.background_img, graphics::DrawParam::new().dest(bg));
            canvas.draw(
                graphics::Text::new("You are DEAD!").set_scale(75.),
                graphics::DrawParam::default().dest(text1),
            );
            canvas.draw(
                graphics::Text::new("Press Space to Restart").set_scale(35.),
                graphics::DrawParam::default().dest(text2),
            );

            canvas.finish(ctx)?;
        } else {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));

            let health_bar = ggez::glam::Vec2::new(50.0, 850.0);
            let text2 = format!("Health: {}", self.player.health);
            canvas.draw(
                graphics::Text::new(text2).set_scale(35.),
                graphics::DrawParam::default().dest(health_bar),
            );

            let scale1 = ggez::glam::Vec2::new(0.125, 0.125);
            let scale2 = ggez::glam::Vec2::new(5.0, 5.0);
            let scale3 = ggez::glam::Vec2::new(0.1, 0.025);

            for x in 2..64 {
                for y in 3..28 {
                    let bg = ggez::glam::Vec2::new(x as f32 * 25.0 - 25.0, y as f32 * 25.0 + 100.0);
                    let dst2 = ggez::glam::Vec2::new(
                        x as f32 * 25.0 - 27.5 - 25.0,
                        y as f32 * 25.0 - 27.5 + 100.0,
                    );
                    let dst3 =
                        ggez::glam::Vec2::new(x as f32 * 25.0 - 75.0, y as f32 * 25.0 + 100.0);

                    if !self.level1 {
                        if self.levels.map1.contains(&(x, y)) {
                            canvas.draw(
                                &self.wall_img,
                                graphics::DrawParam::new().dest(bg).scale(scale1),
                            );
                        } else if self.levels.stair_12.contains(&(x, y)) {
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
                    } else if !self.level2 {
                        if self.levels.map2.contains(&(x, y)) {
                            canvas.draw(
                                &self.wall_img,
                                graphics::DrawParam::new().dest(bg).scale(scale1),
                            );
                        } else if self.levels.stair_23.contains(&(x, y)) {
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
                    } else if !self.level3 {
                        if self.levels.map3.contains(&(x, y)) {
                            canvas.draw(
                                &self.wall_img,
                                graphics::DrawParam::new().dest(bg).scale(scale1),
                            );
                        } else if self.levels.stair_34.contains(&(x, y)) {
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
                    } else if !self.level4 {
                        if self.levels.map4.contains(&(x, y)) {
                            canvas.draw(
                                &self.wall_img,
                                graphics::DrawParam::new().dest(bg).scale(scale1),
                            );
                        } else if self.levels.stair_45.contains(&(x, y)) {
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
                    } else if self.levels.map5.contains(&(x, y)) {
                        canvas.draw(
                            &self.wall_img,
                            graphics::DrawParam::new().dest(bg).scale(scale1),
                        );
                    } else if self.levels.stair_end.contains(&(x, y)) {
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

            if !self.level1 {
                for enemy in &self.enemies1 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite1,
                    );
                }
            } else if !self.level2 {
                for enemy in &self.enemies2 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite1,
                    );
                }
            } else if !self.level3 {
                for enemy in &self.enemies3 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite1,
                    );
                }
            } else if !self.level4 {
                for enemy in &self.enemies4 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite2,
                    );
                }
            } else {
                for enemy in &self.enemies5 {
                    enemy.draw(
                        &mut canvas,
                        enemy.pos,
                        self.player.dir,
                        &mut self.enemy_sprite2,
                    );
                }
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
                    if !self.level1 {
                        self.level1 = true;
                        self.player.pos = GridPosition::new(3, 4);
                    } else if !self.level2 {
                        self.level2 = true;
                        self.player.pos = GridPosition::new(5, 15);
                    } else if !self.level3 {
                        self.level3 = true;
                        self.player.pos = GridPosition::new(4, 26);
                    } else if !self.level4 {
                        self.level4 = true;
                        self.player.pos = GridPosition::new(3, 15);
                    } else {
                        self.level5 = true;
                    }
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
                    if !self.level1 {
                        self.level1 = true;
                        self.player.pos = GridPosition::new(3, 4);
                    } else if !self.level2 {
                        self.level2 = true;
                        self.player.pos = GridPosition::new(5, 15);
                    } else if !self.level3 {
                        self.level3 = true;
                        self.player.pos = GridPosition::new(4, 26);
                    } else if !self.level4 {
                        self.level4 = true;
                        self.player.pos = GridPosition::new(3, 15);
                    } else {
                        self.level5 = true;
                    }
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
                    if !self.level1 {
                        self.level1 = true;
                        self.player.pos = GridPosition::new(3, 4);
                    } else if !self.level2 {
                        self.level2 = true;
                        self.player.pos = GridPosition::new(5, 15);
                    } else if !self.level3 {
                        self.level3 = true;
                        self.player.pos = GridPosition::new(4, 26);
                    } else if !self.level4 {
                        self.level4 = true;
                        self.player.pos = GridPosition::new(3, 15);
                    } else {
                        self.level5 = true;
                    }
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
                    if !self.level1 {
                        self.level1 = true;
                        self.player.pos = GridPosition::new(3, 4);
                    } else if !self.level2 {
                        self.level2 = true;
                        self.player.pos = GridPosition::new(5, 15);
                    } else if !self.level3 {
                        self.level3 = true;
                        self.player.pos = GridPosition::new(4, 26);
                    } else if !self.level4 {
                        self.level4 = true;
                        self.player.pos = GridPosition::new(3, 15);
                    } else {
                        self.level5 = true;
                    }
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
