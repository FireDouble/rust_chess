use ggez::{glam::Vec2, graphics::Canvas, Context};

use super::game_event::GameEvent;

pub mod endgame_modal;
pub mod pawn_promotion_modal;

pub trait Modal {
    fn draw(&self, canvas: &mut Canvas, ctx: &mut Context);
    fn check_for_message(&self, press_position: Option<Vec2>) -> Option<GameEvent>;
}