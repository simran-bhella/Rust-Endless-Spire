// A static map example 
//Shihao and Chentao
use ggez::{
    event,
    glam::*,
    graphics::{self},
    Context, GameResult,
};
use std::{env, path};

struct MainState {
    image1: graphics::Image,
    image2: graphics::Image,
}

impl MainState {
    /// Load images
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::from_path(ctx, "/shot.png")?;
        let image2 = graphics::Image::from_path(ctx, "/tile.png")?;
     
        let s = MainState {
            image1,
            image2,
        };

        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }  

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let scale1=ggez::glam::Vec2::new (0.05,0.05);
        let scale2=ggez::glam::Vec2::new (2.0,2.0);
        //"Bruce" here, "e" is little ugly, Shihao's fault!
        let tuples = vec![(5,15),(6,15),(5,16),(6,16),(5,17),(6,17),(5,18),(6,18),(5,19),(6,19),(5,20),(6,20),(5,21),(6,21),(5,22),(6,22),(5,23),(6,23),
                                           (5,24),(6,24),(5,25),(6,25),(5,26),(6,26),(5,27),(6,27),(5,28),(6,28),(5,29),(6,29),(5,30),(6,30),(5,31),(6,31),(5,32),(6,32),
                                           (5,33),(6,33),(5,34),(6,34),(7,24),(7,25),(8,24),(8,25),(9,24),(9,25),(10,24),(10,25),(11,24),(11,25),(12,24),(12,25),
                                           (7,15),(7,16),(8,15),(8,16),(9,15),(9,16),(10,15),(10,16),(11,15),(11,16),(12,15),(12,16),(13,16),(13,17),(14,17),(14,18),(15,18),(15,19),(15,20),(15,21),(14,21),(14,22),(13,22),(13,23),(12,23),
                                           (7,33),(7,34),(8,33),(8,34),(9,33),(9,34),(10,33),(10,34),(11,33),(11,34),(12,33),(12,26),(13,26),(13,27),(14,27),(14,28),(15,28),(15,29),(15,30),(15,31),(14,31),(14,32),(13,32),(13,33),(12,33),(12,34),
                                           (16,30),(17,30),(18,30),(19,30),(20,30),(19,29),(20,29),(19,28),(20,28),(19,27),(20,27),(19,26),(20,26),(19,25),(20,25),(19,24),(20,24),(19,31),(20,31),(19,32),(20,32),(19,33),(20,33),(19,34),(20,34),(21,28),(21,27),(22,27),(22,26),(23,26),(23,25),(24,25),(24,24),(25,24),(26,24),(26,25),
                                           (27,25),(28,25),(29,25),
                                           (30,24),(31,24),(30,25),(31,25),(30,26),(31,26),(30,27),(31,27),(30,28),(31,28),(30,29),(31,29),(30,30),(31,30),(30,31),(31,31),(31,32),(32,32),(32,33),(33,33),(33,34),(34,34),(35,34),(35,33),(36,33),(36,32),(37,32),(37,31),(38,31),(37,30),(38,30),(37,29),(38,29),(37,28),(38,28),(37,27),(38,27),(37,26),(38,26),(37,25),(38,25),(37,24),(38,24),(38,32),(38,33),(39,33),(39,34),(40,34),
                                           (41,34),(42,34),(43,34),(42,33),(41,32),(42,32),(41,31),(41,30),(44,34),(45,34),(46,33),(47,33),(48,32),(47,32),(45,33),(41,29),(41,28),(41,27),(41,26),(41,25),(42,25),(42,24),(42,23),(43,23),(44,23),(45,23),(45,23),(46,23),(46,24),(47,24),(47,25),(48,25),(49,25),(50,25),(51,25),(52,25),(52,24),(53,24),(53,23),(54,23),(55,23),(56,23),(57,23),(58,23),(58,24),(59,24),(60,25),(59,25),(60,26),(60,27),(59,27),
                                           (58,27),(57,27),(56,27),(55,27),(54,27),(53,27),(52,27),(52,26),(52,28),(52,29),(52,30),(53,30),(53,31),(54,31),(55,31),(56,31),(57,31),(58,31),(59,31),(59,30),(60,30)
                                           ];

        // Draw an image.
         for x in 1..79 {
              for y in 10..40 {
                 let dst = ggez::glam::Vec2::new(x as f32 *10.0, y as f32 *10.0);
                  if tuples.contains(&(x, y)) {
                    canvas.draw(&self.image1, graphics::DrawParam::new().dest(ggez::glam::Vec2::new(x as f32 *10.0-10.0, y as f32 *10.0-10.0)).scale(scale2));     
                } else { 
                    canvas.draw(&self.image2, graphics::DrawParam::new().dest(dst).scale(scale1));     
                         }
    } 
}
        // Finished drawing, show it all on the screen!
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("./resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("drawing", "ggez").add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}

//sim/ added background
struct MainState {
    window_settings: WindowSettings,
    frames: f64,
    angle: f32,
    image1: graphics::Image,
    
}



impl MainState {

    //sim/ added ctx as a param, and background
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::from_path(ctx, "/bg2.png")?;
        let s = MainState { 
            frames: 0.0, 
            angle: 0.0, 
            image1: image1,
            window_settings: WindowSettings {
            toggle_fullscreen: true,
            is_fullscreen: true,
            //resize_projection: false,
            },
    
    };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            self.angle += 0.01;

        if self.window_settings.toggle_fullscreen {
            let fullscreen_type = if self.window_settings.is_fullscreen {
                conf::FullscreenType::Desktop
            } else {
                conf::FullscreenType::Windowed
            };
             ctx.gfx.set_fullscreen(fullscreen_type)?;
             self.window_settings.toggle_fullscreen = true;
        }
    }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //let window = WindowMode;
        
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));
   
        // Text is drawn from the top-left corner.
//        let offset = self.frames as f32;
 //       let dest_point = ggez::glam::Vec2::new(offset, offset);

        //sim/ added param dst and added another canvas draw to add background
        let dst = ggez::glam::Vec2::new(20.0,20.0);
        
        canvas.draw(
            graphics::Text::new("Endless Spire")
            // graphics::Text::new("press any button to start game")
                .set_scale(69.),
                dst,
        );
        
        canvas.draw(&self.image1, graphics::DrawParam::new().dest(dst));
        canvas.finish(ctx)?;
        //ctx.gfx.present(&self.bg.Image(ctx))?;
        
        

        self.frames = 200.;
        // if (self.frames % 100.0) == 0.0 {
        //     println!("FPS: {}", ctx.time.fps());
        // }

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
    
    let cb = ggez::ContextBuilder::new("Endless Spire", "me").add_resource_path(resource_dir);

    //sim/ changed ctx to be mutable, passed into Mainstate::new argument
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Endless Spire");

    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, state)
}

//sim/ if you leave the start screen running, the "endless spire" text eventually disapears 