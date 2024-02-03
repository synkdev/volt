use crate::font_style;
use crate::helpers::compare_fields;
use crate::ui::Color::Hex;
use crate::ui::Component;

#[derive(Copy, Debug)]
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
    pub is_dirty: bool,
    pub is_hovered: bool,
    pub is_visible: bool,
}

pub struct ButtonBuilder {
    pub button: Box<Button>,
}

impl Clone for Button {
    fn clone(&self) -> Self {
        Button {
            text: self.text,
            size: self.size.clone(),
            fill: self.fill.clone(),
            on_hover_leave: self.on_hover_leave.clone(),
            on_hover_enter: self.on_hover_enter.clone(),
            on_click: self.on_click.clone(),
            on_click_release: self.on_click_release.clone(),
            position: self.position.clone(),
            color: self.color.clone(),
            radius: self.radius.clone(),
            is_dirty: self.is_dirty.clone(),
            font_size: self.font_size.clone(),
            font_style: self.font_style.clone(),
            is_hovered: self.is_hovered.clone(),
            is_visible: self.is_visible.clone(),
            font_family: self.font_family,
            font_weight: self.font_weight.clone(),
            border_width: self.border_width.clone(),
            border_color: self.border_color.clone(),
        }
    }
}

impl Component for Button {
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

    fn set_fill(&mut self, color: skia::Color) {
        self.fill = color;
    }

    fn equals(&self, other: &dyn Component) -> bool {
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
        self.is_dirty
    }

    fn is_hovered(&self) -> bool {
        self.is_hovered
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_dirty(&mut self, value: bool) {
        self.is_dirty = value
    }

    fn was_drawn(&mut self) {
        self.is_dirty = false
    }

    fn set_hovered(&mut self, value: bool) {
        self.is_hovered = value
    }
}

impl Button {
    pub fn new() -> Self {
        Button {
            text: "Button",
            size: (200.0, 50.0),
            position: (0.0, 0.0),
            fill: Hex("#313244").into().unwrap(),
            radius: 10.0,
            font_size: 16.0,
            color: Hex("#cdd6f4").into().unwrap(),
            border_color: Hex("#f38ba8").into().unwrap(),
            border_width: 2.0,
            font_style: font_style::Slant::Upright,
            font_weight: font_style::Weight::NORMAL,
            font_family: "JetBrains Mono",
            on_click: |_| {},
            on_click_release: |_| {},
            on_hover_enter: |_| {},
            on_hover_leave: |_| {},
            is_dirty: true,
            is_visible: true,
            is_hovered: false,
        }
    }
}

impl ButtonBuilder {
    pub fn new() -> Self {
        ButtonBuilder {
            button: Box::new(Button::new()),
        }
    }

    pub fn text(mut self, text: &'static str) -> Self {
        self.button.text = text;
        self.button.is_dirty = true;
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.button.position = (x, y);
        self.button.is_dirty = true;
        self
    }

    pub fn on_click(mut self, callback: fn(&mut Button)) -> Self {
        self.button.on_click = callback;
        self
    }

    pub fn on_hover_enter(mut self, callback: fn(&mut Button)) -> Self {
        self.button.on_hover_enter = callback;
        self
    }

    pub fn on_hover_leave(mut self, callback: fn(&mut Button)) -> Self {
        self.button.on_hover_leave = callback;
        self
    }

    pub fn into(self) -> Box<Button> {
        self.button
    }
}
