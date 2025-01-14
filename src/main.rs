use std::collections::HashMap;
use ggez::{
    event,
    graphics::Image,
    Context, GameResult,
};

mod assets;
mod game;
mod app;
mod widgets;
mod scenes;



pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .add_resource_path("./resources")
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(288., 288.)
        )
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Chess")
        );
    let (mut ctx, event_loop) = cb.build()?;
    let app = app::App::init(&mut ctx);
    event::run(ctx, event_loop, app)
}