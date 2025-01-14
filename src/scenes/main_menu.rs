use ggez::{event::MouseButton, glam::Vec2, graphics::{self, Color, DrawParam, Drawable, PxScale, Rect, Text, TextFragment}, Context};

use crate::{app::AppEvent, widgets::Button};

use super::Scene;

pub struct MainMenu {
    pub play_button: Button
}

impl MainMenu {
    pub fn new(ctx: &mut ggez::Context) -> Self{
        Self {
            play_button: Button::new(
                graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(0., 0., 100., 30.),
                    Color::GREEN
                ).unwrap(),
                Text::new(TextFragment {
                    text: "Play".into(),
                    scale: Some(PxScale::from(16.0)),
                    color: Some(Color::BLACK),
                    ..Default::default()
                }),
                [288./2. - 100./2., 152.].into(),
                ctx
            )
        }
    }
}

impl Scene for MainMenu {
    fn draw(&self, _assets: &crate::assets::Assets, canvas: &mut ggez::graphics::Canvas, ctx: &mut ggez::Context) {
        self.play_button.draw(canvas, ctx);

        let mut text = Text::new("Chess");

        text.set_scale(32.);
        let x =  288./2. - text.dimensions(ctx).unwrap().w / 2.;

        canvas.draw(&text, DrawParam::new().color(Color::BLACK).dest([x, 0.]));
    }

    fn update(&mut self, press_data: Option<(Vec2, MouseButton)>, _ctx: &mut Context, app_events: &mut Vec<AppEvent>) {
        if let Some(data) = press_data {
            let is_clicked = self.play_button.is_clicked(Some(data.0));
            if is_clicked {
                app_events.push(AppEvent::OpenNewGame);
            }
        }
    }
}