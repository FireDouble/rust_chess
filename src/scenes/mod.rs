use ggez::{event::MouseButton, glam::Vec2, graphics::Canvas, Context};
use crate::{app::AppEvent, assets::Assets};

pub mod main_menu;

pub trait Scene {
    fn draw(&self, assets: &Assets, canvas: &mut Canvas, ctx: &mut Context);
    fn update(&mut self, press_data: Option<(Vec2, MouseButton)>, ctx: &mut Context, app_events: &mut Vec<AppEvent>);
}