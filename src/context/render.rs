use crate::ui::{Component, Element};

use super::Context;
use glutin::prelude::GlSurface;

impl Context {
    pub fn render(&mut self) {
        let canvas = self.surface.surface.canvas();
        if self.root.dirty {
            self.root.render(canvas, &mut self.paint);
            self.finish_render();
        }
    }
    //
    // pub fn find_dirty_components(&mut self) {
    //     for (id, component) in self.components.iter_mut() {
    //         if component.is_dirty() {
    //             println!("dirty: {id}");
    //             self.dirty_components.push(id.to_owned());
    //         }
    //     }
    // }
    //
    // pub fn redraw_full(&mut self) {
    //     let canvas = self.surface.surface.canvas();
    //
    //     canvas.clear(self.background);
    //     for (_, component) in self.components.iter_mut() {
    //         component.render(canvas, &mut self.paint);
    //     }
    //     self.dirty = false;
    //     self.dirty_components.clear();
    // }
    //
    // pub fn redraw_partial(&mut self) {
    //     let canvas = self.surface.surface.canvas();
    //
    //     for dirty_component in &self.dirty_components {
    //         println!("rendering: {dirty_component}");
    //         let component: &mut Box<dyn Component> =
    //             self.components.get_mut(dirty_component).unwrap();
    //         component.render(canvas, &mut self.paint);
    //         component.was_drawn();
    //     }
    //     self.dirty_components.clear();
    // }
    //
    pub fn finish_render(&mut self) {
        self.gr_context
            .gl_surface
            .swap_buffers(&self.gr_context.gl_context)
            .unwrap();
        self.surface.gr_context.flush_and_submit();
    }
}
