use ggez::{event, graphics, Context, GameResult};
//use ggez::conf;
use std::{env, path};

//struct WindowSettings {
//    toggle_fullscreen: bool,
//    is_fullscreen: bool,
    //resize_projection: bool,
//}

//sim/ added background
struct MainState {
//    window_settings: WindowSettings,
    frames: f64,
//    angle: f32,
    image1: graphics::Image,
    
}



impl MainState {

    //sim/ added ctx as a param, and background
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::from_path(ctx, "/background.png")?;
        let s = MainState { 
            frames: 0.0, 
     //       angle: 0.0, 
            image1: image1,
      //      window_settings: WindowSettings {
      //      toggle_fullscreen: true,
     //       is_fullscreen: true,
            //resize_projection: false,
     //       },
    
    };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        /*const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            self.angle += 0.01;

        if self.window_settings.toggle_fullscreen {
            let fullscreen_type = if self.window_settings.is_fullscreen {
                conf::FullscreenType::Windowed
            } else {
                conf::FullscreenType::Windowed
            };
             ctx.gfx.set_fullscreen(fullscreen_type)?;
             self.window_settings.toggle_fullscreen = true;
        }
    }
    */
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.9]));

        let dst = ggez::glam::Vec2::new(0.0,-120.0);
        
        let tit = ggez::glam::Vec2::new(170.0,225.0);
        let text = ggez::glam::Vec2::new(220.0,375.0);
        
        canvas.draw(&self.image1, graphics::DrawParam::new().dest(dst));
        canvas.draw(
            graphics::Text::new("Endless Spire")
                .set_scale(75.),
                graphics::DrawParam::default().dest(tit),
        );
        canvas.draw(
            graphics::Text::new("Press Any Key to Continue")
                .set_scale(35.),
                graphics::DrawParam::default().dest(text),
        );
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