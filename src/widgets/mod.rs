use ggez::{glam::Vec2, graphics::{Canvas, Drawable, Mesh, Rect, Text}, Context};


pub struct Button {
    pub background: Mesh,
    pub background_dimensions: Rect,
    pub text: Text,
    pub position: Vec2,
}

impl Button {
    pub fn new(background: Mesh, text: Text, position: Vec2, ctx: &mut Context) -> Self {
        let background_dimensions = background.dimensions(ctx).unwrap();
        
        Self {
            background,
            background_dimensions,
            text,
            position
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) {
        canvas.draw(&self.background, self.position);

        let text_dimensions = self.text.dimensions(ctx).unwrap();

        canvas.draw(
            &self.text,
            [self.position.x + self.background_dimensions.w/2. - text_dimensions.w/2., self.position.y + self.background_dimensions.h/2. - text_dimensions.h/2.]
        );
    }

    pub fn is_clicked(&self, press_position: Option<Vec2>) -> bool {
        if let Some(position) = press_position {
            if position.x > self.position.x && position.x < self.position.x + self.background_dimensions.w
            && position.y > self.position.y && position.y < self.position.y + self.background_dimensions.h {
                return true;
            }
        }

        false
    }
}