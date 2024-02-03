use crate::font_style;
use crate::ui::{Color, Element, Widget};

#[derive(Copy, Debug, Clone)]
pub struct Button {
    pub text: &'static str,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: skia::Color,
    pub fill: skia::Color,
    pub radius: f32,
    pub border_width: f32,
    pub border_color: skia::Color,
    pub font_size: f32,
    pub font_family: &'static str,
    pub font_weight: skia::font_style::Weight,
    pub font_style: skia::font_style::Slant,
    pub on_click: fn(&mut Self),
    pub on_click_release: fn(&mut Self),
    pub on_hover_enter: fn(&mut Self),
    pub on_hover_leave: fn(&mut Self),
    pub dirty: bool,
    pub hovered: bool,
    pub clicked: bool,
}

impl Element for Button {
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
        paint.set_color(self.color);
        paint.set_style(skia::PaintStyle::Fill);
        let font_style = skia::font_style::FontStyle::new(
            self.font_weight,
            skia::font_style::Width::NORMAL,
            self.font_style,
        );
        let font_family = skia::FontMgr::new()
            .match_family_style(self.font_family, font_style)
            .unwrap();
        let font = skia::Font::from_typeface(font_family, self.font_size);
        let text = skia::TextBlob::from_str(self.text, &font).unwrap();
        let text_bounds = font.measure_text(self.text, Some(&paint));
        let text_x = rect.center_x() - (text_bounds.1.width() / 2.0);
        let text_offset =
            ((font.metrics().1.descent - font.metrics().1.ascent) / 2.0) - font.metrics().1.descent;
        let text_y = rect.center_y() + text_offset;
        canvas.draw_text_blob(text, (text_x, text_y), &paint);
    }

    fn equals(&self, other: &dyn Element) -> bool {
        if let Some(other_component) = other.downcast_ref::<Button>() {
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
