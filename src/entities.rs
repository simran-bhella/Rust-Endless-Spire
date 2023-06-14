//! Endless Spire Rougelike Game
//! Simranjit Bhella, Bruce Truong, John Asencio-Giron, Shihao Li, and Chentao Ma 2023
//!
//! In this file, the GridPosition, Player, and Enemy objects are set up.

use ggez::graphics;
use ggez::graphics::Rect;
use rand::Rng;

/// Keep track of locations of entities on a defined grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    /// Takes in an x and y coord and returns a new GridPosition object
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }
}

/// Keep track of information relevent to the player entity.
pub struct Player {
    /// Player Health
    pub health: f32,

    /// Direction player is facing
    pub dir: i16,

    /// Player location on grid
    pub pos: GridPosition,
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
    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        pos: GridPosition,
        pic: &mut graphics::Image,
    ) {
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
pub struct Enemy {
    /// Enemy location on grid
    pub pos: GridPosition,
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
    pub fn draw(
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
    pub fn move_pos(&self) -> GridPosition {
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
