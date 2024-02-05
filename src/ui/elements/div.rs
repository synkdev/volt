use crate::ui::handle_events::MouseEventType;
use skia::Color;
use skia::Rect;

use crate::ui::handle_events::get_active_element;
use crate::ui::Element;

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
    pub full_redraw: bool,
    pub active_element: Option<Box<dyn Element>>,
}

impl Element for Div {
    fn render(&mut self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint) {
        let border_offset = self.border_width / 2.0;
        let rect = Rect::from_xywh(
            self.position.0 + border_offset,
            self.position.1 + border_offset,
            self.size.0,
            self.size.1,
        );

        let clipped_rect = Rect::from_xywh(
            rect.left(),
            rect.top() + border_offset,
            rect.width() - border_offset,
            rect.height() - border_offset,
        );
        // Draw div box
        paint.set_color(self.fill);
        paint.set_style(skia::PaintStyle::Fill);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        // Draw border
        paint.set_color(self.border_color);
        paint.set_style(skia::PaintStyle::Stroke);
        paint.set_stroke_width(self.border_width);

        canvas.draw_round_rect(rect, self.radius, self.radius, &paint);

        canvas.clip_rect(clipped_rect, None, Some(true));

        // Draw children
        self.order_children();
        for child in self.children.iter_mut() {
            if child.is_dirty() {
                child.render(canvas, paint);
            }
        }

        canvas.restore();
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
        let (new_active_element, event_type) =
            get_active_element(&mut self.children, self.active_element.as_mut(), position);

        match (self.active_element.as_mut(), new_active_element, event_type) {
            (Some(active_element), Some(new_element), MouseEventType::Entered) => {
                active_element.on_hover_leave();
                self.active_element = Some(new_element);
                new_element.on_hover_enter();
            }

            (Some(active_element), None, MouseEventType::Exited) => {
                active_element.on_hover_leave();
                self.active_element = None;
            }

            (None, Some(new_element), MouseEventType::Entered) => {
                self.active_element = Some(new_element);
                new_element.on_hover_enter();
            }
            _ => {}
        }
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

impl Div {
    pub fn new() -> Self {
        Div {
            children: Vec::new(),
            on_hover_enter: |_| {},
            on_hover_leave: |_| {},
            on_click: |_| {},
            on_click_release: |_| {},
            clicked: false,
            fill: Color::TRANSPARENT,
            size: (400.0, 400.0),
            dirty: false,
            radius: 0.0,
            z_index: 0,
            hovered: false,
            position: (0.0, 0.0),
            border_width: 5.0,
            border_color: Color::GRAY,
            full_redraw: true,
            active_element: None,
        }
    }

    pub fn into(self) -> Box<Div> {
        Box::new(self)
    }

    pub fn add(&mut self, element: Box<dyn Element>) {
        self.children.push(element);
    }

    pub fn order_children(&mut self) {
        self.children
            .sort_by(|a, b| a.get_z_index().cmp(&b.get_z_index()));
    }
}
