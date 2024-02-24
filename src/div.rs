use vello::kurbo::{Affine, Point, RoundedRect, Size, Stroke};

use crate::{color::Color, element::Element, styles::BorderOffset};

pub struct Div {
    pub children: Vec<Box<dyn Element>>,
    pub size: Size,
    pub position: Point,
    pub background: Color,
    pub border_width: f64,
    pub border_color: Color,
    pub border_offset: BorderOffset,
    pub radius: f64,
}

impl Element for Div {
    fn render(&mut self, scene: &mut vello::Scene) {
        let shape = RoundedRect::new(
            self.position.x,
            self.position.y,
            self.position.x + self.size.width,
            self.position.y + self.size.height,
            self.radius,
        );
        let border_stroke = match self.border_offset {
            BorderOffset::Outset => RoundedRect::new(
                self.position.x + self.border_width,
                self.position.y + self.border_width,
                self.position.x + self.size.width + self.border_width,
                self.position.y + self.size.height + self.border_width,
                self.radius,
            ),
            BorderOffset::Inset => RoundedRect::new(
                self.position.x - self.border_width,
                self.position.y - self.border_width,
                self.position.x + self.size.width - self.border_width,
                self.position.y + self.size.height - self.border_width,
                self.radius,
            ),
            BorderOffset::Center => RoundedRect::new(
                self.position.x + (self.border_width / 2.0),
                self.position.y + (self.border_width / 2.0),
                self.position.x + self.size.width + (self.border_width / 2.0),
                self.position.y + self.size.height + (self.border_width / 2.0),
                self.radius,
            ),
        };
        let stroke = Stroke::new(self.border_width);
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            self.background.clone().into(),
            None,
            &shape,
        );
        scene.stroke(
            &stroke,
            Affine::IDENTITY,
            self.border_color.clone().into(),
            None,
            &border_stroke,
        );
        for child in self.children.iter_mut() {
            child.render(scene);
        }
    }
}

impl Default for Div {
    fn default() -> Self {
        Div {
            children: vec![],
            size: Size::new(200.0, 200.0),
            position: Point::new(20.0, 20.0),
            background: Color::Hex("#313243"),
            border_width: 2.0,
            border_color: Color::Hex("#f38ba8"),
            border_offset: BorderOffset::Center,
            radius: 20.0,
        }
    }
}
