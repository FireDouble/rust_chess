use ggez::{glam::Vec2, graphics::{self, Color, Drawable, PxScale, Rect, Text, TextFragment}, Context};

use crate::{game::{game_event::GameEvent, piece::PieceType, position::Position}, widgets::Button};

use super::Modal;

pub struct PawnPromotionModal{
    queen_button: Button,
    rook_button: Button,
    knight_button: Button,
    bishop_button: Button, 
    position: Position,
}

impl PawnPromotionModal {
    pub fn new(ctx: &mut Context, position: Position) -> Self {
        let queen_button = Button::new(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 60., 60.),
                Color::GREEN
            ).unwrap(),
            Text::new(TextFragment {
                text: "Queen".into(),
                scale: Some(PxScale::from(16.0)),
                color: Some(Color::BLACK),
                ..Default::default()
            }),
            [0., 20.].into(),
            ctx
        );

        let rook_button = Button::new(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 60., 60.),
                Color::GREEN
            ).unwrap(),
            Text::new(TextFragment {
                text: "Rook".into(),
                scale: Some(PxScale::from(16.0)),
                color: Some(Color::BLACK),
                ..Default::default()
            }),
            [60., 20.].into(),
            ctx
        );

        let knight_button = Button::new(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 60., 60.),
                Color::GREEN
            ).unwrap(),
            Text::new(TextFragment {
                text: "Knight".into(),
                scale: Some(PxScale::from(16.0)),
                color: Some(Color::BLACK),
                ..Default::default()
            }),
            [0., 70.].into(),
            ctx
        );

        let bishop_button = Button::new(
            graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 60., 60.),
                Color::GREEN
            ).unwrap(),
            Text::new(TextFragment {
                text: "Bishop".into(),
                scale: Some(PxScale::from(16.0)),
                color: Some(Color::BLACK),
                ..Default::default()
            }),
            [60., 70.].into(),
            ctx
        );

        Self {
            queen_button,
            rook_button,
            knight_button,
            bishop_button,
            position
        }
    }
}

impl Modal for PawnPromotionModal {
    fn draw(&self, canvas: &mut graphics::Canvas, ctx: &mut Context) {
        canvas.draw(
            &graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 120., 70.),
                Color::WHITE
        ).unwrap(), [0., 0.]);

        let mut text = Text::new(TextFragment {
            text: "Choose Piece".into(),
            scale: Some(PxScale::from(16.0)),
            color: Some(Color::BLACK),
            ..Default::default()
        });

        text.set_bounds(Vec2::new(120., 20.));

        let text_dimensions = text.dimensions(ctx).unwrap();



        canvas.draw(
            &text,
            [60. - text_dimensions.w/2., 0.]
        );

        self.queen_button.draw(canvas, ctx);
        self.rook_button.draw(canvas, ctx);
        self.knight_button.draw(canvas, ctx);
        self.bishop_button.draw(canvas, ctx);
    }

    fn check_for_message(&self, press_position: Option<ggez::glam::Vec2>) -> Option<GameEvent> {
        if self.queen_button.is_clicked(press_position) {
            return Some(GameEvent::ChoosePiece(PieceType::Queen, self.position));
        }
        if self.rook_button.is_clicked(press_position) {
            return Some(GameEvent::ChoosePiece(PieceType::Rook, self.position));
        }
        if self.knight_button.is_clicked(press_position) {
            return Some(GameEvent::ChoosePiece(PieceType::Knight, self.position));
        }
        if self.bishop_button.is_clicked(press_position) {
            return Some(GameEvent::ChoosePiece(PieceType::Bishop, self.position));
        }
        
        None
    }
}