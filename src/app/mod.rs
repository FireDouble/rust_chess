use ggez::{event::{self, MouseButton}, glam::Vec2, graphics, Context, GameResult};

use crate::{assets::Assets, game::Game, scenes::{Scene, main_menu::MainMenu}};

pub enum AppEvent {
    OpenMainMenu,
    OpenNewGame,
}

pub struct App {
    pub assets: Assets,
    pub current_scene: Box<dyn Scene>,
    pub press_data: Option<(Vec2, MouseButton)>,
    pub events: Vec<AppEvent>,
    // pub press_button: Option<MouseButton>,
    // pub highlighted_spaces: Vec<[usize; 2]>
}

impl App {
    pub fn init(ctx: &mut Context) -> Self {
        Self {
            assets: Assets::load_all(ctx),
            current_scene: Box::new(MainMenu::new(ctx)),
            press_data: None,
            events: Vec::new(),
            // press_button: None,
            // highlighted_spaces: Vec::new()
        }
    }
}

impl event::EventHandler<ggez::GameError> for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for event in self.events.drain(..) {
            match event {
                AppEvent::OpenMainMenu => self.current_scene = Box::new(MainMenu::new(ctx)),
                AppEvent::OpenNewGame => self.current_scene = Box::new(Game::new()),
            }   
        }

        self.current_scene.update(self.press_data, ctx, &mut self.events);

        self.press_data = None;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([1., 1., 1., 1.]));

        self.current_scene.draw(&self.assets, &mut canvas, ctx);

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: f32, y: f32,) -> GameResult {
        self.press_data = Some((Vec2::from([x, y]), button));
        // self.press_button = Some(button);

        
        Ok(())
    }
}