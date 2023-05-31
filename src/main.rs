use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::conf;
use ggez::{event, graphics, Context, GameResult};
use std::{env, path};

const MAX_HEALTH: f32 = 100.0;

struct WindowSettings {
    pub fullscreen_type: conf::FullscreenType,
    toggle_fullscreen: bool,
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * 10 as i32,
            pos.y as i32 * 10 as i32,
            10 as i32,
            10 as i32,
        )
    }
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition {x, y}
    }

}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    health: f32,
    //player_img: graphics::Image,
    //i think i could add in the weapon they are holding here
    //just have another struct defining weapon types and such

    pos: GridPosition,
    dir: Direction,

}

impl Player {
    pub fn new(pos: GridPosition) -> Self {
        Player {
            health: 100.0,
            pos: GridPosition::new(20,20),
            dir: Direction::Right,

        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.pos.into())
                .color([1.0, 0.5, 0.0, 1.0]),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}


//sim/ added background
struct MainState {
        window_settings: WindowSettings,
    frames: f64,
    background_img: graphics::Image,
    image1: graphics::Image,
    image2: graphics::Image,
    image3: graphics::Image,
    stair: graphics::Image,
    start_screen: bool,
    player: Player,
}

impl MainState {
    //sim/ added ctx as a param, and background
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let background_img = graphics::Image::from_path(ctx, "/background.png")?;
        let image1 = graphics::Image::from_path(ctx, "/shot.png")?;
        let image2 = graphics::Image::from_path(ctx, "/tile.png")?;
        let image3 = graphics::Image::from_path(ctx, "/wabbit_alpha.png")?;
        let image4 = graphics::Image::from_path(ctx, "/wabbit_alpha.png")?;
        let stair = graphics::Image::from_path(ctx, "/stair.png")?;

        let s = MainState {
            frames: 0.0,
            background_img: background_img,
            image1: image1,
            image2: image2,
            image3: image3,
            stair:  stair,
            start_screen: true,
            player: Player::new(GridPosition::new(20,20)),
            window_settings: WindowSettings {
                fullscreen_type: conf::FullscreenType::Windowed,
                toggle_fullscreen: false,
            },
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let start_press = &ctx.keyboard;
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if start_press.is_key_pressed(KeyCode::Space) {
                //start the next event so i think run function that would run game
                self.start_screen = false;
                self.window_settings.toggle_fullscreen = true;
                ctx.gfx.set_fullscreen(self.window_settings.fullscreen_type)?;
                self.window_settings.toggle_fullscreen = true;
            }

            
        }
                    
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.start_screen {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));
            let dst = ggez::glam::Vec2::new(0.0, -120.0);

            let tit = ggez::glam::Vec2::new(170.0, 225.0);
            let text = ggez::glam::Vec2::new(220.0, 375.0);
            let display = ggez::glam::Vec2::new(800.0, 600.0);

            canvas.draw(&self.background_img, graphics::DrawParam::new().dest(dst));
            canvas.draw(
                graphics::Text::new("Endless Spire").set_scale(75.),
                graphics::DrawParam::default().dest(tit),
            );
            canvas.draw(
                graphics::Text::new("Press Any Key to Continue").set_scale(35.),
                graphics::DrawParam::default().dest(text),
            );

            canvas.draw(&self.image3, graphics::DrawParam::new().dest(display));
            
            self.frames = 200.;
            canvas.finish(ctx)?;
            
        }
        else {
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

            
            //trying to display player health
            let health_bar = ggez::glam::Vec2::new(50.0, 850.0); 
            canvas.draw(
                graphics::Text::new("Health: ").set_scale(35.),
                graphics::DrawParam::default().dest(health_bar),
            );

            let spawn = ggez::glam::Vec2::new(100.0, 200.0);

            let scale1=ggez::glam::Vec2::new (0.125,0.125);
            let scale2=ggez::glam::Vec2::new (5.0,5.0);
            let scale3=ggez::glam::Vec2::new (0.1,0.025);
            //"Bruce" here, "e" is little ugly, Shihao's fault!
            /*let tuples = vec![(7,6),(8,6),(9,6),(10,6),(11,6),(12,6),(5, 7),(7,7),(8,7),(9,7),(10,7),(11,7),(12,7),(5, 6),(6, 6), (5, 7), (6, 7), (5, 8), (6, 8), (5, 9), (6, 9), (5, 10), (6, 10), (5, 11), (6, 11), (5, 12), (6, 12), (5, 13), (6, 13), (5, 14), (6, 14), (5, 15), (6, 15), (5, 16), (6, 16), (5, 17), (6, 17), (5, 18), (6, 18), (5, 19), (6, 19), (5, 20), (6, 20), (5, 21), (6, 21), (5, 22), (6, 22), (5, 23), (6, 23), (5, 24), (6, 24), (7, 14), (7, 15), (8, 14), (8, 15), (9, 14), (9, 15), (10, 14), (10, 15), (11, 14), (11, 15), (12, 14), (12, 15), (13, 6), (13, 7), (14, 7), (14, 8), (15, 8), (15, 9), (15, 10), (15, 11), (14, 11), (14, 12), (13, 12), (13, 13), (12, 13), (7, 23), (7, 24), (8, 23), (8, 24), (9, 23), (9, 24), (10, 23),
                                                (10, 24), (11, 23), (11, 24), (12, 23), (12, 16), (13, 16), (13, 17), (14, 17), (14, 18), (15, 18), (15, 19), (15, 20), (15, 21), (14, 21), (14, 22), (13, 22), (13, 23), (12, 23), (12, 24), (16, 20), (17, 20), (18, 20), (19, 20), (20, 20), (19, 19), (20, 19), (19, 18), (20, 18), (19, 17), (20, 17), (19, 16), (20, 16), (19, 15), (20, 15), (19, 14), (20, 14), (21, 18), (21, 17), (22, 17), (22, 16), (23, 16), (23, 15), (24, 15), (24, 14), (25, 14), (26, 14), (26, 15), (27, 15), (28, 15), (29, 15), (30, 14), (31, 14), (30, 15), (31, 15), (30, 16), (31, 16), (30, 17), (31, 17), (30, 18), (31, 18), (30, 19), (31, 19), (30, 20), (31, 20), (30, 21), (31, 21), (31, 22), (32, 22), (32, 23), (33, 23), (33, 24), (34, 24), (35, 24), (35, 23), (36, 23), (36, 22), (37, 22),
                                                (37, 21), (38, 21), (37, 20), (38, 20), (37, 19), (38, 19), (37, 18), (38, 18), (37, 17), (38, 17), (37, 16), (38, 16), (37, 15), (38, 15), (37, 14), (38, 14), (38, 22), (38, 23), (39, 23), (39, 24), (40, 24), (41, 24), (42, 24), (43, 24), (42, 23), (41, 22), (42, 22), (41, 21), (41, 20), (44, 24), (45, 24), (46, 23), (47, 23), (48, 22), (47, 22), (45, 23), (41, 19), (41, 18), (41, 17), (41, 16), (41, 15), (42, 15), (42, 14), (42, 13), (43, 13), (44, 13), (45, 13), (45, 13), (46, 13), (46, 14), (47, 14), (47, 15), (48, 15), (49, 15), (50, 15), (51, 15), (52, 15), (52, 14), (53, 14), (53, 13), (54, 13), (55, 13), (56, 13), (57, 13), (58, 13), (58, 14), (59, 14), (60, 15), (59, 15), (60, 16), (60, 17), (59, 17), (58, 17), (57, 17), (56, 17), (55, 17), (54, 17), (53, 17), (52, 17), (52, 16), (52, 18), (52, 19), (52, 20), (53, 20), (53, 21), (54, 21), (55, 21), (56, 21), (57, 21), (58, 21), (59, 21), (59, 20)
                                                ,(19,21),(19,22),(19,23),(19,24),(20,21),(20,22),(20,23),(20,24)];
            */
            
            /*let tuples = vec![(2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9), (2, 10), (2, 11), (2, 12), (2, 13), (2, 14), (2, 15), (2, 16), (2, 17), (2, 18), (2, 19), (2, 20), (2, 21), (2, 22), (2, 23), (2, 24), (2, 25), (2, 26), (2, 27), (2, 28),
                                               (63, 3), (63, 4), (63, 5), (63, 6), (63, 7), (63, 8), (63, 9), (63, 10), (63, 11), (63, 12), (63, 13), (63, 14), (63, 15), (63, 16), (63, 17), (63, 18), (63, 19), (63, 20), (63, 21), (63, 22), (63, 23), (63, 24), (63, 25), (63, 26), (63, 27), (63, 28),
                                               (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3), (20, 3), (21, 3), (22, 3), (23, 3), (24, 3), (25, 3), (26, 3), (27, 3), (28, 3), (29, 3), (30, 3), (31, 3), (32, 3), (33, 3), (34, 3), (35, 3), (36, 3), (37, 3), (38, 3), (39, 3), (40, 3), (41, 3), (42, 3), (43, 3), (44, 3), (45, 3), (46, 3), (47, 3), (48, 3), (49, 3), (50, 3), (51, 3), (52, 3), (53, 3), (54, 3), (55, 3), (56, 3), (57, 3), (58, 3), (59, 3), (60, 3), (61, 3), (62, 3), (63, 3), (64, 3),
                                               (2, 27), (3, 27), (4, 27), (5, 27), (6, 27), (7, 27), (8, 27), (9, 27), (10, 27), (11, 27), (12, 27), (13, 27), (14, 27), (15, 27), (16, 27), (17, 27), (18, 27), (19, 27), (20, 27), (21, 27), (22, 27), (23, 27), (24, 27), (25, 27), (26, 27), (27, 27), (28, 27), (29, 27), (30, 27), (31, 27), (32, 27), (33, 27), (34, 27), (35, 27), (36, 27), (37, 27), (38, 27), (39, 27), (40, 27), (41, 27), (42, 27), (43, 27), (44, 27), (45, 27), (46, 27), (47, 27), (48, 27), (49, 27), (50, 27), (51, 27), (52, 27), (53, 27), (54, 27), (55, 27), (56, 27), (57, 27), (58, 27), (59, 27), (60, 27), (61, 27), (62, 27), (63, 27), (64, 27),
                                               (2, 12), (3, 12), (4, 12), (6, 12), (7, 12), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (14, 12), (15, 12), (16, 12), (17, 12), (18, 12), (19, 12), (20, 12), (21, 12), (22, 12), (23, 12), (24, 12), (25, 12), (26, 12), (27, 12), (28, 12), (29, 12), (30, 12), (31, 12), (33, 12), (34, 12), (35, 12), (36, 12), (37, 12), (38, 12), (39, 12), (40, 12), (41, 12), (42, 12), (43, 12), (44, 12), (45, 12), (46, 12), (47, 12), (48, 12), (49, 12), (50, 12), (51, 12), (52, 12), (53, 12), (54, 12), (55, 12), (56, 12), (57, 12), (58, 12), (59, 12), (60, 12), (62, 12), (63, 12), (64, 12),
                                               (2, 18), (3, 18), (4, 18), (5, 18), (6, 18), (7, 18), (8, 18), (9, 18), (10, 18), (11, 18), (12, 18), (13, 18), (14, 18), (15, 18), (16, 18), (17, 18), (18, 18), (19, 18), (20, 18), (21, 18), (22, 18), (23, 18), (24, 18), (25, 18), (26, 18), (27, 18), (28, 18), (29, 18), (30, 18), (31, 18), (32, 18), (33, 18), (34, 18), (35, 18), (36, 18), (37, 18), (38, 18), (39, 18), (40, 18), (41, 18), (42, 18), (43, 18), (44, 18), (45, 18), (46, 18), (47, 18), (48, 18), (49, 18), (50, 18), (51, 18), (52, 18), (53, 18), (54, 18), (55, 18), (56, 18), (57, 18), (58, 18), (59, 18), (60, 18), (62, 18), (63, 18), (64, 18),
                                               (8, 12), (8, 13), (8, 14), (8, 15), (8, 16), (8, 17), (8, 18),
                                               (5,6),(6,6),(7,6),(5,7),(5,8),(5,9),(6,9),(7,9),
                                               (10,4),(10,5),(10,6),(10,7),(10,8),(10,9),(10,10),
                                               (13,6),(13,7),(13,8),(13,9),(13,10),(13,11),
                                               (16,4),(16,5),(16,6),(16,7),(16,8),(16,9),(16,10),
                                               (19,6),(20,6),(21,6),(24,6),(25,6),(26,6),(19,9),(20,9),(21,9),(24,9),(25,9),(26,9),
                                               (29,5),(29,6),(29,7),(29,8),(29,9),(29,10),(29,11),
                                               (31,6),(31,7),(31,8),(32,6),(33,6),(33,7),(33,8),(31,10),(32,10),(33,10),
                                               (35,5),(35,6),(35,7),(35,8),(35,9),(35,10),(35,11),
                                               (39,6),(40,6),(41,7),(42,8),(43,9),(44,9),(45,8),(46,7),(47,6),(48,7),(49,8),(50,9),(51,9),(52,8),(53,7),(54,6),(55,6),
                                               (59,5),(59,6),(59,7),(59,8),(59,9),(59,10),(59,11),
                                               (60,5),(62,5),(60,7),(62,7),(60,9),(62,9),(60,11),(62,11),
                                               (14,13),(14,14),(14,16),(14,17),
                                               (17, 15), (18, 15), (19, 15), (20, 15), (21, 15), (22, 15), (23, 15), (24, 15), (25, 15), (26, 15), (27, 15), (28, 15), (29, 15),
                                               (32,14),(32,15),(32,16),(32,17),
                                               (35,14),(36,14),(37,14),(37,13),(35,16),(36,16),(37,16),(37,17),(40,14),(41,14),(42,14),(42,13),(40,16),(41,16),(42,16),(42,17),(45,14),(46,14),(47,14),(47,13),(45,16),(46,16),(47,16),(47,17),(50,14),(51,14),(52,14),(52,13),(50,16),(51,16),(52,16),(52,17),(55,14),(56,14),(57,14),(57,13),(55,16),(56,16),(57,16),(57,17),(58,14),(59,14),(58,16),(59,16),
                                               (3,19),(4,20),(5,21),(6,22),(6,23),(5,24),(4,25),(3,26),(7,21),(8,20),(9,19),(7,24),(8,25),(9,26),(8,21),(9,21),(10,21),(10,20),(10,19),(8,24),(9,24),(10,24),(10,25),(10,26),
                                               (13,20),(13,21),(14,21),(15,21),(16,21),(17,21),(17,20),(15,22),(15,23),(13,25),(13,24),(14,24),(15,24),(16,24),(17,24),(17,25),
                                               (20,21),(21,21),(22,22),(23,22),(24,21),(25,21),(24,21),(25,21),(20,24),(21,24),(22,23),(23,23),(24,24),(25,24),(24,24),(25,24),
                                               (28, 20), (28, 21), (29, 21), (30, 21), (31, 21), (32, 21), (32, 20), (30, 22), (30, 23), (28, 25), (28, 24), (29, 24), (30, 24), (31, 24), (32, 24), (32, 25),
                                               (35, 21), (36, 21), (37, 22), (38, 22), (39, 21), (40, 21), (39, 21), (40, 21), (35, 24), (36, 24), (37, 23), (38, 23), (39, 24), (40, 24), (39, 24), (40, 24),
                                               (43, 20), (43, 21), (44, 21), (45, 21), (46, 21), (47, 21), (47, 20), (45, 22), (45, 23), (43, 25), (43, 24), (44, 24), (45, 24), (46, 24), (47, 24), (47, 25),
                                               (50, 21), (51, 21), (52, 22), (53, 22), (54, 21), (55, 21), (54, 21), (55, 21), (50, 24), (51, 24), (52, 23), (53, 23), (54, 24), (55, 24), (54, 24), (55, 24),
                                               (60,19),(60,20),(60,21),(59,21),(58,21),(58,24),(59,24),(60,24),(61,24),(62,24)
                                                ];
            */
            /*let tuples = vec![(24, 3), (38, 3), (55, 3), (60, 3), (3, 4), (4, 4), (5, 4), (9, 4), (24, 4), (55, 4), (60, 4), (61, 4), (3, 5), (9, 5), (24, 5), (38, 5), (41, 5), (42, 5), (43, 5), (44, 5), (45, 5), (49, 5), (50, 5), (51, 5), (52, 5), (53, 5), (55, 5), (58, 5), (59, 5), (60, 5), (61, 5), (62, 5), (3, 6), (9, 6), (14, 6), (15, 6), (16, 6), (21, 6), (24, 6), (25, 6), (26, 6), (27, 6), (28, 6), (29, 6), (30, 6), (31, 6), (32, 6), (33, 6), (34, 6), (35, 6), (36, 6), (37, 6), (38, 6), (41, 6), (49, 6), 
(55, 6), (60, 6), (61, 6), (3, 7), (6, 7), (9, 7), (15, 7), (16, 7), (21, 7), (24, 7), (38, 7), (41, 7), (49, 7), (55, 7), (60, 7), (3, 8), (6, 8), (9, 8), (14, 8), (16, 8), (21, 8), (24, 8), (38, 8), (41, 8), (45, 8), (49, 8), (53, 8), (55, 8), (3, 9), (4, 9), (5, 9), (6, 9), (9, 9), (10, 9), (11, 9), (13, 9), (21, 9), (24, 9), (27, 9), (28, 9), (29, 9), (30, 9), (31, 9), (32, 9), (33, 9), (34, 9), (35, 9), (38, 9), (41, 9), (45, 9), (49, 9), (53, 9), (55, 9), (59, 9), (61, 9), (12, 10), (21, 10), (24, 10), (27, 10), (35, 10), (38, 10), (41, 10), (45, 10), (49, 10), (53, 10), (55, 10), (59, 10), (61, 10), (7, 11), (8, 11), (11, 11), (21, 11), (24, 11), (27, 11), (35, 11), (38, 11), (41, 11), (45, 11), (47, 11), (49, 11), 
(53, 11), (55, 11), (59, 11), (61, 11), (6, 12), (9, 12), (10, 12), (14, 12), (21, 12), (24, 12), (27, 12), (35, 12), (38, 12), (41, 12), (42, 12), (43, 12), (44, 12), (45, 12), (47, 12), (49, 12), (50, 12), (51, 12), (52, 12), (53, 12), (55, 12), (59, 12), (61, 12), (5, 13), (9, 13), (10, 13), (15, 13), (21, 13), (24, 13), (27, 13), (35, 13), (38, 13), (47, 13), (55, 13), (59, 13), (60, 13), (61, 13), (4, 14), (9, 14), (10, 14), (15, 14), (21, 14), (24, 14), (27, 14), (35, 14), (38, 14), (47, 14), (55, 14), (59, 14), (60, 14), (61, 14), (62, 14), (4, 15), (15, 15), (21, 15), (24, 15), (27, 15), (35, 15), (38, 15), (44, 15), (45, 15), (46, 15), (47, 15), (48, 15), (49, 15), (50, 15), (51, 15), (55, 15), (59, 15), (60, 15), (61, 15), (62, 15), (4, 16), (15, 16), (21, 16), (24, 16), (27, 16), (28, 16), (29, 16), (30, 16), (31, 16), (35, 16), (38, 16), (55, 16), (59, 16), (60, 16), (61, 16), (62, 16), (5, 17), (14, 17), (21, 17), (24, 17), (35, 17), (38, 17), (54, 
17), (4, 18), (6, 18), (13, 18), (21, 18), (24, 18), (35, 18), (38, 18), (43, 18), (46, 18), (49, 18), (50, 18), (51, 18), (52, 18), (53, 18), (3, 19), (7, 19), (12, 19), (21, 19), (24, 19), (35, 19), (38, 19), (43, 19), (46, 19), (62, 19), (2, 20), (8, 20), (11, 20), (21, 20), (24, 20), (25, 20), (26, 20), (27, 20), (28, 20), (29, 20), (30, 20), (31, 20), (32, 20), (33, 20), (34, 20), (35, 20), (38, 20), (43, 20), (46, 20), (62, 20), (4, 21), (5, 21), (6, 21), (7, 21), (8, 21), (12, 21), (13, 21), (14, 21), (15, 21), (21, 21), (38, 21), (43, 21), (46, 21), (62, 21), (2, 22), (3, 22), (4, 22), (15, 22), (16, 22), (17, 22), (21, 22), (38, 22), (43, 22), (46, 22), (50, 22), (53, 22), (62, 22), (2, 23), (3, 23), (10, 23), (16, 23), (17, 23), (21, 23), (38, 23), (43, 23), (44, 23), (45, 23), (46, 23), (47, 23), (48, 23), (50, 23), (52, 23), (54, 23), (55, 23), (56, 23), (58, 23), (59, 23), (62, 23), (2, 24), (3, 24), (9, 24), (10, 24), (11, 24), (16, 24), (17, 24), 
(21, 24), (22, 24), (23, 24), (24, 24), (25, 24), (26, 24), (30, 24), (31, 24), (32, 24), (33, 24), (34, 24), (35, 24), (36, 24), (37, 24), (38, 24), (46, 24), (50, 24), (51, 24), (54, 24), (56, 24), (58, 24), (60, 24), (62, 24), (2, 25), (3, 25), (8, 25), (9, 25), (10, 25), (11, 25), (12, 25), (16, 25), (17, 25), (26, 25), (30, 25), (31, 25), (32, 25), (33, 25), (34, 25), (35, 25), (36, 25), (37, 25), (38, 25), (39, 25), (40, 25), (46, 25), (50, 25), (54, 25), (55, 25), (56, 25), (58, 25), (60, 25), (62, 25), (2, 26), (3, 26), (10, 26), (16, 26), (17, 26), (26, 26), (46, 26), (50, 26), (54, 26), (58, 26), (60, 26), (62, 26), (2, 27), (3, 27), (10, 27), (16, 27), (17, 27), (26, 27), (46, 27), (50, 27), (54, 27), (55, 27), 
(56, 27), (58, 27), (59, 27), (60, 27), (62, 27), (63, 27), (64, 27), (2, 28), (3, 28), (10, 28), (16, 28), (17, 28), (26, 28), (46, 28), (50, 28), (61, 28)];*/
                 /* 
                 let tuples = vec![(10, 3), (11, 3), (12, 3), (6, 4), (10, 4), (11, 4), (13, 4), (14, 4), (24, 4), (60, 4), (61, 4), (62, 4), (63, 4), (64, 4), (5, 5), (7, 5), (8, 5), (9, 5), (10, 5), (11, 5), (15, 5), (16, 5), (17, 5), (18, 5), (19, 5), (24, 5), (25, 5), (26, 
5), (58, 5), (59, 5), (60, 5), (2, 6), (5, 6), (19, 6), (20, 6), (24, 6), (27, 6), (28, 6), (49, 6), (50, 6), (56, 6), (57, 6), (64, 6), (3, 7), (5, 7), (8, 7), (9, 7), (21, 7), (25, 7), (29, 7), (30, 7), (42, 7), (43, 7), (47, 7), (48, 7), (50, 7), (54, 7), (55, 7), (63, 7), (4, 8), (5, 8), (8, 8), (9, 8), (10, 8), (21, 8), (25, 8), (26, 8), (30, 8), (40, 8), (41, 8), (43, 8), (46, 8), (47, 8), (49, 8), (53, 8), (54, 8), (62, 8), (7, 9), (8, 9), (9, 9), (10, 9), (11, 9), (23, 9), (26, 9), (31, 9), (39, 9), (43, 9), (45, 9), (46, 9), (48, 9), (49, 9), (52, 9), (53, 9), (61, 9), (6, 10), (7, 10), (8, 10), (9, 10), (10, 10), (11, 10), (12, 10), (24, 10), (25, 10), (31, 10), (38, 10), (42, 10), (44, 10), (48, 10), (60, 10), (6, 11), (7, 11), (8, 11), (9, 11), (10, 11), (11, 11), (12, 11), (13, 11), (14, 11), (31, 11), (36, 11), (37, 11), (41, 11), (44, 11), (59, 11), (8, 12), (9, 12), (10, 12), (11, 12), (12, 12), (13, 12), (30, 12), (35, 12), (40, 12), (17, 13), (21, 13), (25, 13), (30, 13), (32, 13), (33, 13), (34, 13), (42, 13), (46, 13), (49, 13), (51, 13), (52, 13), (53, 13), (55, 13), (56, 13), (57, 13), (59, 13), (60, 13), (61, 13), (18, 14), (20, 14), (22, 14), (24, 14), (26, 14), (29, 14), (31, 
14), (42, 14), (43, 14), (46, 14), (49, 14), (51, 14), (53, 14), (55, 14), (57, 14), (59, 14), (19, 15), (23, 15), (27, 15), (28, 15), (38, 15), (39, 15), (40, 15), (41, 15), (42, 15), (43, 15), (44, 15), (46, 15), (47, 15), (48, 15), (49, 15), (51, 15), (53, 15), (55, 15), (56, 15), (57, 15), (59, 15), (60, 15), (61, 15), (42, 16), (43, 16), (46, 16), (49, 16), (51, 16), (53, 16), (55, 16), (59, 16), (19, 17), (23, 17), (27, 17), (42, 17), (46, 17), (49, 17), (51, 17), (52, 17), (53, 17), (55, 17), (59, 17), (60, 17), (61, 17), (18, 18), (20, 18), (22, 18), (24, 18), (26, 18), (28, 18), (30, 18), (31, 18), (17, 19), (21, 19), (25, 19), (29, 19), (32, 19), (33, 19), (34, 19), (35, 19), (40, 19), (2, 20), (3, 20), (29, 20), (35, 20), (36, 20), (37, 20), (41, 20), (43, 20), (52, 20), (59, 20), (28, 21), (29, 21), (38, 21), (42, 21), (44, 21), (50, 21), (52, 21), (53, 21), (60, 21), (3, 22), (7, 22), (12, 22), (13, 22), (14, 22), (15, 22), (27, 22), (39, 22), (43, 
22), (45, 22), (50, 22), (54, 22), (55, 22), (61, 22), (3, 23), (6, 23), (7, 23), (12, 23), (16, 23), (26, 23), (40, 23), (43, 23), (46, 23), (51, 23), (55, 23), (62, 23), (2, 24), (6, 24), (7, 24), (12, 24), (16, 24), (17, 24), (18, 24), (19, 
24), (24, 24), (25, 24), (41, 24), (42, 24), (43, 24), (47, 24), (48, 24), (49, 24), (50, 24), (55, 24), (56, 24), (63, 24), (2, 25), (5, 25), (7, 25), (12, 25), (20, 25), (21, 25), (22, 25), (23, 25), (56, 25), (64, 25), (2, 26), (4, 26), (7, 
26), (10, 26), (11, 26), (57, 26), (58, 26), (59, 26), (3, 27), (6, 27), (8, 27), (9, 27), (59, 27), (60, 27), (2, 28), (6, 28), (7, 28), (61, 28), (62, 28), (63, 28), (64, 28)];
                */
            let tuples = vec![(2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3), (20, 3), (21, 3), (22, 3), (23, 3), (24, 3), (25, 3), (26, 3), (27, 3), (28, 3), (29, 3), (30, 3), (31, 3), (32, 3), (33, 3), (34, 3), (35, 3), (36, 3), (37, 3), (38, 3), (39, 3), (40, 3), (41, 3), (42, 3), (43, 3), (44, 3), (45, 3), (46, 3), (47, 3), (48, 3), (49, 3), (50, 3), (51, 3), (52, 3), (53, 3), (54, 3), (55, 3), (56, 3), 
(57, 3), (58, 3), (59, 3), (60, 3), (61, 3), (62, 3), (63, 3), (64, 3), (4, 4), (6, 4), (8, 4), (10, 4), (12, 4), (20, 4), (22, 4), (24, 4), (26, 4), (28, 4), (38, 4), (46, 4), (48, 4), (52, 4), (56, 4), (58, 4), (60, 4), (63, 4), (64, 4), (2, 
5), (4, 5), (6, 5), (8, 5), (10, 5), (12, 5), (14, 5), (16, 5), (17, 5), (18, 5), (20, 5), (22, 5), (24, 5), (26, 5), (28, 5), (29, 5), (31, 5), (32, 5), (33, 5), (34, 5), (35, 5), (36, 5), (38, 5), (40, 5), (41, 5), (42, 5), (43, 5), (44, 5), 
(46, 5), (47, 5), (48, 5), (50, 5), (52, 5), (54, 5), (56, 5), (58, 5), (59, 5), (60, 5), (62, 5), (63, 5), (64, 5), (2, 6), (8, 6), (14, 6), (18, 6), (20, 6), (28, 6), (36, 6), (42, 6), (50, 6), (54, 6), (63, 6), (64, 6), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (8, 7), (10, 7), (12, 7), (13, 7), (14, 7), (15, 7), (16, 7), (18, 7), (20, 7), (22, 7), (24, 7), (25, 7), (26, 7), (28, 7), (30, 7), (31, 7), (32, 7), (33, 7), (34, 7), (36, 7), (37, 7), (38, 7), (39, 7), (40, 7), (42, 7), (44, 7), (45, 7), (46, 7), (47, 7), (48, 7), (49, 7), (50, 7), (51, 7), (52, 7), (53, 7), (54, 7), (55, 7), (56, 7), (58, 7), (60, 7), (61, 7), (63, 7), (64, 7), (2, 8), (10, 8), (16, 8), (18, 8), (22, 8), (24, 8), (26, 8), (28, 8), (30, 8), (36, 8), (40, 8), (42, 8), (50, 8), (52, 8), (56, 8), (58, 8), (60, 8), (63, 8), (64, 8), (2, 9), (4, 9), (6, 9), (7, 9), (8, 9), (9, 9), (10, 9), (11, 9), (12, 9), (13, 9), (14, 9), (16, 9), (17, 9), (18, 9), (20, 9), (21, 9), (22, 9), (24, 9), (26, 9), (28, 9), (30, 9), (31, 9), (32, 9), (33, 9), (34, 9), (36, 9), (37, 9), (38, 9), (40, 9), (42, 9), (43, 9), (44, 9), (46, 9), (47, 9), (48, 9), (50, 9), (52, 9), (54, 9), (56, 9), (57, 9), (58, 9), (59, 9), (60, 9), (62, 9), (63, 9), (64, 9), (2, 10), (4, 10), (6, 10), (14, 10), (18, 10), (24, 10), (26, 10), (28, 10), (30, 10), (36, 10), (40, 10), (42, 10), (46, 10), (48, 10), (54, 10), (58, 10), (63, 10), (64, 10), (2, 11), (4, 11), (6, 11), (8, 11), (9, 11), (10, 11), (11, 
11), (12, 11), (13, 11), (14, 11), (15, 11), (16, 11), (18, 11), (20, 11), (21, 11), (22, 11), (23, 11), (24, 11), (26, 11), (28, 11), (30, 11), (32, 11), (33, 11), (34, 11), (35, 11), (36, 11), (38, 11), (40, 11), (42, 11), (43, 11), (44, 11), (48, 11), (50, 11), (52, 11), (53, 11), (54, 11), (55, 11), (56, 11), (58, 11), (60, 11), (61, 11), (63, 11), (64, 11), (2, 12), (4, 12), (6, 12), (8, 12), (14, 12), (18, 12), (26, 12), (30, 12), (32, 12), (36, 12), (38, 12), (42, 12), (46, 12), (48, 12), (50, 12), (52, 12), (54, 12), (56, 12), (60, 12), (63, 12), (64, 12), (2, 13), (3, 13), (4, 13), (6, 13), (8, 13), (10, 13), (12, 13), (14, 13), (16, 13), (18, 13), (19, 13), (20, 13), (21, 13), (23, 13), (24, 13), (25, 13), (26, 13), (27, 13), (28, 13), (29, 13), (30, 13), (32, 13), (34, 13), (36, 13), (38, 13), (39, 13), (40, 13), (41, 13), (42, 13), (44, 13), (45, 13), (46, 13), (48, 13), (50, 13), (51, 13), (52, 13), (54, 13), (56, 13), (57, 13), (58, 13), (59, 13), 
(60, 13), (61, 13), (63, 13), (64, 13), (2, 14), (6, 14), (10, 14), (12, 14), (16, 14), (19, 14), (30, 14), (32, 14), (34, 14), (38, 14), (48, 14), (60, 14), (63, 14), (64, 14), (2, 15), (4, 15), (5, 15), (6, 15), (7, 15), (8, 15), (10, 15), (11, 15), (12, 15), (14, 15), (15, 15), (16, 15), (19, 15), (20, 15), (21, 15), (22, 15), (24, 15), (25, 15), (26, 15), (28, 15), (29, 15), (30, 15), (32, 15), (34, 15), (36, 15), (37, 15), (38, 15), (39, 15), (40, 15), (42, 15), (43, 15), (44, 15), (45, 15), (46, 15), (47, 15), (48, 15), (49, 15), (50, 15), (52, 15), (54, 15), (55, 15), (56, 15), (57, 15), (58, 15), (60, 15), (61, 15), (63, 15), (64, 15), (2, 16), (4, 16), (12, 16), (16, 16), (22, 16), (24, 16), (26, 16), (34, 16), (36, 16), (38, 16), (42, 16), (50, 16), (52, 16), (54, 16), (58, 16), (60, 16), (63, 16), (64, 16), (2, 17), (4, 17), (5, 17), (6, 17), (8, 17), (9, 17), (10, 17), (11, 17), (12, 17), (14, 17), (16, 17), (18, 17), (19, 17), (20, 17), (22, 17), (24, 17), (26, 17), (27, 17), (28, 17), (29, 17), (30, 17), (31, 17), (32, 17), (33, 17), (34, 17), (36, 17), (38, 17), (40, 17), (41, 17), (42, 17), (44, 17), (46, 17), (47, 17), (48, 17), (50, 17), (51, 17), (52, 17), (53, 17), (54, 17), (56, 17), (58, 17), (59, 17), (60, 17), (61, 17), (63, 17), (64, 17), (2, 18), (8, 18), (14, 18), (18, 18), (20, 18), (22, 18), (24, 18), (32, 18), (36, 18), (40, 18), (44, 18), (46, 18), (50, 18), (56, 18), (58, 18), (63, 18), (64, 18), (2, 19), (3, 19), (4, 19), (5, 19), (6, 19), (7, 19), (8, 19), (9, 19), (10, 19), (12, 19), (13, 19), (14, 19), (15, 19), (16, 19), (18, 19), (20, 19), (22, 19), (24, 19), (25, 19), (26, 19), (28, 19), (29, 19), (30, 19), (31, 19), (32, 19), (34, 19), (35, 19), (36, 19), (38, 19), (40, 19), (42, 19), (43, 19), (44, 19), (46, 19), (47, 19), (48, 19), (50, 19), (51, 19), (52, 19), (54, 19), (55, 19), (56, 19), (58, 19), (60, 19), (61, 19), (62, 19), (63, 19), (64, 19), (2, 20), (4, 20), (12, 20), 
(14, 20), (16, 20), (18, 20), (20, 20), (28, 20), (32, 20), (36, 20), (38, 20), (40, 20), (44, 20), (46, 20), (50, 20), (54, 20), (58, 20), (60, 20), (63, 20), (64, 20), (2, 21), (6, 21), (8, 21), (9, 21), (10, 21), (11, 21), (12, 21), (14, 21), (16, 21), (18, 21), (20, 21), (21, 21), (22, 21), (23, 21), (24, 21), (25, 21), (26, 21), (27, 21), (28, 21), (30, 21), (32, 21), (33, 21), (34, 21), (36, 21), (37, 21), (38, 21), (40, 21), (41, 21), (42, 21), (44, 21), (46, 21), (47, 21), (48, 21), (50, 21), (52, 21), (53, 21), (54, 21), (56, 21), (57, 21), (58, 21), (60, 21), (62, 21), (63, 21), (64, 21), (2, 22), (4, 22), (6, 22), (8, 22), (10, 22), (18, 22), (20, 22), (30, 22), (34, 22), (38, 22), (42, 22), (44, 22), (46, 22), 
(50, 22), (52, 22), (56, 22), (58, 22), (63, 22), (64, 22), (2, 23), (4, 23), (5, 23), (6, 23), (8, 23), (10, 23), (12, 23), (14, 23), (15, 23), (16, 23), (18, 23), (20, 23), (22, 23), (23, 23), (24, 23), (26, 23), (27, 23), (28, 23), (29, 23), (30, 23), (31, 23), (32, 23), (34, 23), (35, 23), (36, 23), (38, 23), (39, 23), (40, 23), (42, 23), (43, 23), (44, 23), (46, 23), (47, 23), (48, 23), (50, 23), (52, 23), (53, 23), (54, 23), (56, 23), (58, 23), (61, 23), (63, 23), (64, 23), (2, 24), (6, 24), (8, 24), (12, 24), (14, 24), (18, 24), (20, 24), (22, 24), (26, 24), (34, 24), (40, 24), (44, 24), (46, 24), (52, 24), (58, 24), (60, 24), (61, 24), (63, 24), (64, 24), (2, 25), (3, 25), (4, 25), (6, 25), (8, 25), (10, 25), (11, 
25), (12, 25), (14, 25), (16, 25), (17, 25), (18, 25), (20, 25), (21, 25), (22, 25), (24, 25), (26, 25), (28, 25), (29, 25), (30, 25), (31, 25), (32, 25), (33, 25), (34, 25), (36, 25), (38, 25), (39, 25), (40, 25), (41, 25), (42, 25), (44, 25), (45, 25), (46, 25), (48, 25), (50, 25), (51, 25), (52, 25), (54, 25), (55, 25), (56, 25), (58, 25), (61, 25), (63, 25), (64, 25), (2, 26), (14, 26), (24, 26), (26, 26), (36, 26), (48, 26), (56, 26), (64, 26), (2, 27), (3, 27), (4, 27), (5, 27), (6, 27), (7, 27), (8, 27), (9, 27), (10, 27), (11, 27), (12, 27), (13, 27), (14, 27), (15, 27), (16, 27), (17, 27), (18, 27), (19, 27), (20, 27), (21, 27), (22, 27), (23, 27), (24, 27), (25, 27), (26, 27), (27, 27), (28, 27), (29, 27), (30, 27), (31, 27), (32, 27), (33, 27), (34, 27), (35, 27), (36, 27), (37, 27), (38, 27), (39, 27), (40, 27), (41, 27), (42, 27), (43, 27), (44, 27), (45, 27), (46, 27), (47, 27), (48, 27), (49, 27), (50, 27), (51, 27), (52, 27), (53, 27), (54, 27), 
(55, 27), (56, 27), (57, 27), (58, 27), (59, 27), (60, 27), (61, 27), (62, 27), (63, 27), (64, 27), (2, 28), (3, 28), (4, 28), (5, 28), (6, 28), (7, 28), (8, 28), (9, 28), (10, 28), (11, 28), (12, 28), (13, 28), (14, 28), (15, 28), (16, 28), (17, 28), (18, 28), (19, 28), (20, 28), (21, 28), (22, 28), (23, 28), (24, 28), (25, 28), (26, 28), (27, 28), (28, 28), (29, 28), (30, 28), (31, 28), (32, 28), (33, 28), (34, 28), (35, 28), (36, 28), (37, 28), (38, 28), (39, 28), (40, 28), (41, 28), (42, 28), (43, 28), (44, 28), (45, 28), (46, 28), (47, 28), (48, 28), (49, 28), (50, 28), (51, 28), (52, 28), (53, 28), (54, 28), (55, 28), (56, 28), (57, 28), (58, 28), (59, 28), (60, 28), (61, 28), (62, 28), (63, 28), (64, 28)];
            /*
            let tuples = vec![(2, 3), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 9), (2, 10), (2, 11), (2, 12), (2, 13), (2, 14), (2, 15), (2, 16), (2, 17), (2, 18), (2, 19), (2, 20), (2, 21), (2, 22), (2, 23), (2, 24), (2, 25), (2, 26), (2, 27), (2, 28),
                                               (63, 3), (63, 4), (63, 5), (63, 6), (63, 7), (63, 8), (63, 9), (63, 10), (63, 11), (63, 12), (63, 13), (63, 14), (63, 15), (63, 16), (63, 17), (63, 18), (63, 19), (63, 20), (63, 21), (63, 22), (63, 23), (63, 24), (63, 25), (63, 26), (63, 27), (63, 28),
                                               (2, 3), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (8, 3), (9, 3), (10, 3), (11, 3), (12, 3), (13, 3), (14, 3), (15, 3), (16, 3), (17, 3), (18, 3), (19, 3), (20, 3), (21, 3), (22, 3), (23, 3), (24, 3), (25, 3), (26, 3), (27, 3), (28, 3), (29, 3), (30, 3), (31, 3), (32, 3), (33, 3), (34, 3), (35, 3), (36, 3), (37, 3), (38, 3), (39, 3), (40, 3), (41, 3), (42, 3), (43, 3), (44, 3), (45, 3), (46, 3), (47, 3), (48, 3), (49, 3), (50, 3), (51, 3), (52, 3), (53, 3), (54, 3), (55, 3), (56, 3), (57, 3), (58, 3), (59, 3), (60, 3), (61, 3), (62, 3), (63, 3), (64, 3),
                                               (2, 27), (3, 27), (4, 27), (5, 27), (6, 27), (7, 27), (8, 27), (9, 27), (10, 27), (11, 27), (12, 27), (13, 27), (14, 27), (15, 27), (16, 27), (17, 27), (18, 27), (19, 27), (20, 27), (21, 27), (22, 27), (23, 27), (24, 27), (25, 27), (26, 27), (27, 27), (28, 27), (29, 27), (30, 27), (31, 27), (32, 27), (33, 27), (34, 27), (35, 27), (36, 27), (37, 27), (38, 27), (39, 27), (40, 27), (41, 27), (42, 27), (43, 27), (44, 27), (45, 27), (46, 27), (47, 27), (48, 27), (49, 27), (50, 27), (51, 27), (52, 27), (53, 27), (54, 27), (55, 27), (56, 27), (57, 27), (58, 27), (59, 27), (60, 27), (61, 27), (62, 27), (63, 27), (64, 27),
                                                (2,4),(2,5),(2,6),(2,7),(2,8),(2,9),(2,10),(2,11),(3,4),(3,5),(3,6),(3,7),(3,8),(3,9),(3,10),(3,11),(3,12),(3,21),(3,22),(3,23),(3,24),(3,25),(3,26),(4,4),(4,5),(4,6),(4,7),(4,8),(4,9),(4,10),(4,11),(4,12),(4,21),(4,22),(4,23),(4,24),(4,25),(4,26),(5,4),(5,5),(5,6),(5,7),(5,8),(5,9),(5,10),(5,11),(5,12),(5,21),(5,22),(5,23),(5,24),(5,25),(5,26),(6,4),(6,5),(6,6),(6,7),(6,8),(6,9),(6,10),(6,11),(6,12),(6,21),(6,22),(6,23),(6,24),(6,25),(6,26),
                                                (7,4),(7,5),(7,6),(7,7),(7,8),(7,9),(7,10),(7,11),(7,12),(7,13),(7,14),(7,15),(7,16),(7,17),(7,18),(7,21),(7,22),(7,23),(7,24),(7,25),(7,26),(8,4),(8,5),(8,6),(8,7),(8,8),(8,9),(8,10),(8,11),(8,12),(8,13),(8,14),(8,15),(8,16),(8,17),(8,18),(9,4),(9,5),(9,6),(9,7),(9,8),(9,9),(9,10),(9,11),(9,12),(9,12),(9,13),(9,14),(9,15),(9,16),(9,17),(9,18),(10,4),(10,5),(10,6),(10,7),(10,8),(10,9),(10,10),(10,11),(10,12),(10,13),(10,14),(10,15),(10,16),(10,17),(10,18),(11,21),(11,22),(11,23),(11,24),(11,25),(11,26),(12,21),(12,22),(12,23),(12,24),(12,25),(12,26),(13,21),(13,22),(13,23),(13,24),(13,25),(13,26),(14,21),(14,22),(14,23),(14,24),(14,25),(14,26),(15,7),(15,8),(15,9),(15,10),(15,11),(15,12),(15,13),(15,14),(15,15),(15,16),(15,17),(15,18),(15,19),(15,20),(15,21),(15,22),(15,23),(15,24),(15,25),(15,26),(16,7),(16,8),(16,9),(16,10),(16,11),(16,12),(16,13),(16,14),(16,15),(16,16),(16,17),(16,18),(16,19),(16,20),(16,21),(16,22),(16,23),(16,24),(16,25),(16,26),(17,7),(17,8),(17,9),(17,10),(17,11),(17,12),(17,13),(17,14),(17,15),(17,16),(17,17),(17,18),(17,19),(17,20),(17,21),(17,22),(17,23),(17,24),(17,25),(17,26),(18,21),(18,22),(18,23),(18,24),(18,25),(18,26),(19,21),(19,22),(19,23),(19,24),(19,25),(19,26),(20,21),(20,22),(20,23),(20,24),(20,25),(20,26),(21,21),(21,22),(21,23),(21,24),(21,25),(21,26),
                                                (22,4),(22,5),(22,6),(22,7),(22,8),(22,9),(22,10),(22,11),(22,12),(23,4),(23,5),(23,6),(23,7),(23,8),(23,9),(23,10),(23,11),(23,12), (24,4),(24,5),(24,6),(24,7),(24,8),(24,9),(24,10),(24,11),(24,12),(29,8),(29,9),(29,10),(29,11),(29,12),(29,13),(29,14),(29,15),(29,16),(29,17),(29,18),(29,19),(29,20),(29,21),(29,22),(29,23),(29,24),(29,25),(29,26),(30,8),(30,9),(30,10),(30,11),(30,12),(30,13),(30,14),(30,15),(30,16),(30,17),(30,18),(30,19),(30,20),(30,21),(30,22),(30,23),(30,24),(30,25),(30,26),(31,8),(31,9),(31,10),(31,11),(31,12),(31,13),(31,14),(31,15),(31,16),(31,17),(31,18),(31,19),(31,20),(31,21),(31,22),(31,23),(31,24),(31,25),(31,26),(32,17),(33,17),(34,17),(35,17),(36,17),(37,17),(38,17),(39,17),(40,17),(41,17),(42,17),(43,17),(44,17),(45,17),(46,17),(47,17),(32,16),(33,16),(34,16),(35,16),(36,16),(37,16),(38,16),(39,16),(40,16),(41,16),(42,16),(43,16),(44,16),(45,16),(46,16),(47,16),(32,15),(33,15),(34,15),(35,15),(36,15),(37,15),(38,15),(39,15),(40,15),(41,15),(42,15),(43,15),(44,15),(45,15),(46,15),(47,15),
                                                (39,4),(39,5),(39,6),(39,7),(39,8),(39,9),(39,10),(40,4),(40,5),(40,6),(40,7),(40,8),(40,9),(40,10),(41,4),(41,5),(41,6),(41,7),(41,8),(41,9),(41,10),(42,4),(42,5),(42,6),(42,7),(42,8),(42,9),(42,10),
                                                (43,9),(44,9),(45,9),(46,9),(47,9),(48,9),(49,9),(50,9),(51,9),(52,9),(53,9),(54,9),(55,9),(56,9),(57,9),(43,10),(44,10),(45,10),(46,10),(47,10),(48,10),(49,10),(50,10),(51,10),(52,10),(53,10),(54,10),(55,10),(56,10),(57,10),(52,15),(52,16),(52,17),(52,18),(52,19),(52,20),(52,21),(52,22),(53,15),(53,16),(53,17),(53,18),(53,19),(53,20),(53,21),(53,22),(53,15),(54,15),(55,15),(56,15),(57,15),(58,15),(59,15),(60,15),(61,15),(62,15),(52,16),(53,16),(54,16),(55,16),(56,16),(57,16),(58,16),(59,16),(60,16),(61,16),(62,16),
                                                (51,22),(50,22),(49,22),(48,22),(47,22),(46,22),(45,22),(44,22),(43,22),(42,22),(41,22),(40,22),(39,22),(38,22),(51,21),(50,21),(49,21),(48,21),(47,21),(46,21),(45,21),(44,21),(43,21),(42,21),(41,21),(40,21),(39,21),(38,21)
                                                ];
            */
            let stair_pos = vec![(63,26),(2,4)];
            // Draw an image.
            
             for x in 2..64 {
                  for y in 3..28 {
                     let dst = ggez::glam::Vec2::new(x as f32 *25.0-25.0, y as f32 *25.0+100.0);
                     let dst2 =ggez::glam::Vec2::new(x as f32 *25.0-27.5-25.0, y as f32 *25.0-27.5+100.0);
                     let dst3 =ggez::glam::Vec2::new(x as f32 *25.0-75.0, y as f32 *25.0+100.0);
                      if tuples.contains(&(x, y)) {
                        canvas.draw(&self.image2, graphics::DrawParam::new().dest(dst).scale(scale1));     
                    }else if stair_pos.contains(&(x,y)){
                        canvas.draw(&self.stair,graphics::DrawParam::new().dest(dst3).scale(scale3));
                    }else { 
                        canvas.draw(&self.image1, graphics::DrawParam::new().dest(dst2).scale(scale2));     
                    }
                    
            }
             
    }

                self.player.draw(&mut canvas);
                canvas.finish(ctx)?;
                ggez::timer::yield_now();
    
            }
            Ok(())
        }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {


        match input.keycode {

            Some(KeyCode::Up) => {
                self.player.pos = GridPosition::new(self.player.pos.x, (self.player.pos.y - 1).rem_euclid(75));
            }

            Some(KeyCode::Down) => {
                self.player.pos = GridPosition::new(self.player.pos.x, (self.player.pos.y + 1).rem_euclid(75));
            }

            Some(KeyCode::Left) => {
                self.player.pos = GridPosition::new((self.player.pos.x - 1).rem_euclid(139), self.player.pos.y)
            }

            Some(KeyCode::Right) => {
                self.player.pos = GridPosition::new((self.player.pos.x + 1).rem_euclid(139), self.player.pos.y)
            }

            _ => (),





        }
        Ok(())
    }
}
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
        .window_mode(conf::WindowMode::default().dimensions(1600.0, 1000.0));

    //sim/ changed ctx to be mutable, passed into Mainstate::new argument
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Endless Spire");

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state)
}

