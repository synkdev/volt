use crate::ui::{Color::Hex, Element};

#[derive(Clone)]
pub struct Div {
    pub children: Vec<Box<dyn Element>>,
    pub z_index: usize,
    pub position: (f32, f32),
    pub fill: skia::Color,
    pub radius: f32,
    pub border_width: f32,
    pub border_color: skia::Color,
    pub size: (f32, f32),
    pub on_click: fn(&mut Self),
    pub on_click_release: fn(&mut Self),
    pub on_hover_enter: fn(&mut Self),
    pub on_hover_leave: fn(&mut Self),
    pub dirty: bool,
    pub hovered: bool,
    pub clicked: bool,
}

impl Element for Div {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint) {
        let rect =
            skia::Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1);

        paint.set_anti_alias(true);

        // Draw button box
        paint.set_color(self.fill);
        paint.set_style(skia::PaintStyle::Fill);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw border
        paint.set_color(self.border_color);
        paint.set_style(skia::PaintStyle::Stroke);
        paint.set_stroke_width(self.border_width);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw text
    }

    fn equals(&self, other: &dyn Element) -> bool {
        if let Some(other_component) = other.downcast_ref::<Div>() {
            // return compare_fields!(
            //     self, other_component; fill
            // );
            return self.fill == other_component.fill;
        } else {
            println!("no compare");
            return false;
        }
    }

    fn on_click(&mut self) {
        (self.on_click)(self)
    }

    fn on_click_release(&mut self) {
        (self.on_click_release)(self)
    }

    fn on_hover_enter(&mut self) {
        let old_state = self.clone();

        {
            (self.on_hover_enter)(self);
        }

        println!("value of equals on enter: {}", self.equals(&old_state));
        println!(
            "old fill: {:?};   new fill: {:?}",
            old_state.fill, self.fill
        );

        if self.equals(&old_state) {
            self.set_dirty(false);
        } else {
            self.set_dirty(true);
        }
    }

    fn on_hover_leave(&mut self) {
        let old_state = self.clone();

        (self.on_hover_leave)(self);

        println!("value of equals on leave: {}", self.equals(&old_state));
        println!(
            "old fill: {:?};   new fill: {:?}",
            old_state.fill, self.fill
        );
        if !self.equals(&old_state) {
            self.set_dirty(false);
        } else {
            self.set_dirty(true);
        }
    }

    fn get_bounds(&self) -> skia::Rect {
        skia::Rect::from_xywh(self.position.0, self.position.1, self.size.0, self.size.1)
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn is_hovered(&self) -> bool {
        self.hovered
    }

    fn is_clicked(&self) -> bool {
        self.clicked
    }

    fn set_dirty(&mut self, value: bool) {
        self.dirty = value
    }

    fn set_hovered(&mut self, value: bool) {
        self.hovered = value
    }

    fn set_clicked(&mut self, value: bool) {
        self.clicked = value
    }
}
