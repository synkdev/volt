use crate::font_style;
use crate::ui::{Color::Hex, Element};

#[derive(Copy, Debug, Clone)]
pub struct Button {
    pub text: &'static str,
    pub position: (f32, f32),
    pub z_index: usize,
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

    fn on_click(&mut self) {
        (self.on_click)(self)
    }

    fn on_click_release(&mut self) {
        (self.on_click_release)(self)
    }

    fn on_hover_enter(&mut self) {
        (self.on_hover_enter)(self);
    }

    fn on_hover_leave(&mut self) {
        (self.on_hover_leave)(self);
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

    fn get_z_index(&self) -> usize {
        self.z_index
    }
    fn set_z_index(&mut self, index: usize) {
        self.z_index = index;
    }
    fn mouse_moved(&mut self, position: (f32, f32)) {
        println!("moved");
    }

    fn mouse_input(
        &mut self,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
        position: (f32, f32),
    ) {
        println!("clicked?");
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
            dirty: true,
            clicked: true,
            hovered: false,
            z_index: 1,
        }
    }
}
