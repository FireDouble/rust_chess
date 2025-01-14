use ggez::{glam::Vec2, graphics::{self, Canvas, Color, Drawable, PxScale, Rect, Text, TextFragment}, Context};

use crate::{game::piece::Player, widgets::Button};

use super::{Modal, GameEvent};

pub struct EndgameModal {
    end_type: EndType,
    replay_button: Button,
    exit_button: Button
}

pub enum EndType {
    Checkmate(Player),
    Stalemate,
    InsufficientMaterial,
    Repetition,
    FiftyMoveRule
}


impl EndgameModal {
    pub fn new(end_type: EndType, ctx: &mut Context) -> Self {
        let replay_button = Button::new(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 100., 30.),
                Color::GREEN
            ).unwrap(),
            Text::new(TextFragment {
                text: "Play Again".into(),
                scale: Some(PxScale::from(16.0)),
                color: Some(Color::BLACK),
                ..Default::default()
            }),
            [39., 152.].into(),
            ctx
        );

        let exit_button = Button::new(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 100., 30.),
                Color::GREEN
            ).unwrap(),
            Text::new(TextFragment {
                text: "Exit".into(),
                scale: Some(PxScale::from(16.0)),
                color: Some(Color::BLACK),
                ..Default::default()
            }),
            [149., 152.].into(),
            ctx
        );

        Self {
            end_type,
            replay_button,
            exit_button
        }
    }
}

impl Modal for EndgameModal {
    fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) {
        canvas.draw(
            &graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 256., 96.),
                Color::WHITE
        ).unwrap(), [288./2. - 256./2., 288./2. - 96./2.]);

        let win_text = match self.end_type {
            EndType::Checkmate(player) => if player == Player::White { "White Won by checkmate" } else { "Black Won by checkmate" },
            EndType::Stalemate => "Draw by Stalemate",
            EndType::InsufficientMaterial => "Draw by Insufficient Material",
            EndType::Repetition => "Draw by Repetition",
            EndType::FiftyMoveRule => "Draw by 50 Move Rule",
        };

        let mut text = Text::new(TextFragment {
            text: win_text.into(),
            scale: Some(PxScale::from(16.0)),
            color: Some(Color::BLACK),
            ..Default::default()
        });

        text.set_bounds(Vec2::new(256., 96.));

        let text_dimensions = text.dimensions(ctx).unwrap();



        canvas.draw(
            &text,
            [288./2. - text_dimensions.w/2., 288./2. - 96./2. + 10.]
        );


        self.replay_button.draw(canvas, ctx);
        self.exit_button.draw(canvas, ctx);
    }

    fn check_for_message(&self, press_position: Option<Vec2>) -> Option<GameEvent> {
        if self.replay_button.is_clicked(press_position) {
            return Some(GameEvent::Replay);
        }
        if self.exit_button.is_clicked(press_position) {
            return Some(GameEvent::Exit);
        }
        
        None
    }
}