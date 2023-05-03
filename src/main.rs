use ggez::{event, graphics, Context, GameResult};
use ggez::conf;
use std::{env, path};

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
    //resize_projection: bool,
}

//sim/ added background
struct MainState {
    window_settings: WindowSettings,
    frames: f64,
    angle: f32,
    bg: graphics::Image,
}


impl MainState {

    //sim/ added ctx as a param, and background
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let bg = graphics::Image::from_path(ctx, r"\bg2.png")?;
        let s = MainState { frames: 0.0, angle: 0.0, bg,
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
            // ctx.gfx.set_fullscreen(fullscreen_type)?;
            // self.window_settings.toggle_fullscreen = true;
        }
    }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //let window = WindowMode;
        
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 0.5]));
   
        // Text is drawn from the top-left corner.
        let offset = self.frames as f32;
        let dest_point = ggez::glam::Vec2::new(offset, offset);

        //sim/ added param dst and added another canvas draw to add background
        //let dst = ggez::glam::Vec2::new(0.4,0.0);
        
        canvas.draw(
            graphics::Text::new("Endless Spire")
            // graphics::Text::new("press any button to start game")
                .set_scale(69.),
            dest_point,
        );
        canvas.draw(&self.bg, graphics::DrawParam::new().dest(dest_point));
        canvas.finish(ctx)?;
        ctx.gfx.present(&self.bg.image(ctx))?;
        
        

        self.frames = 0.5;
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

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

//sim/ if you leave the start screen running, the "endless spire" text eventually disapears 