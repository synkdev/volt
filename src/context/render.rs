use crate::ui::Element;

use super::Context;
use glutin::prelude::GlSurface;

impl Context {
    pub fn render(&mut self) {
        self.root.find_dirty_children();
        if self.root.is_dirty() {
            println!("root needs full redraw");
            self.redraw_full();
            self.root.set_dirty(false);
            self.finish_render();
        } else if self.root.dirty_children.len() > 0 {
            let canvas = self.surface.surface.canvas();
            println!("root not dirty, rendering children");
            self.root.render_children(canvas, &mut self.paint);
            self.finish_render();
        }
    }

    pub fn redraw_full(&mut self) {
        let canvas = self.surface.surface.canvas();

        canvas.clear(self.background);
        self.root.render(canvas, &mut self.paint);
    }

    pub fn finish_render(&mut self) {
        self.gr_context
            .gl_surface
            .swap_buffers(&self.gr_context.gl_context)
            .unwrap();
        self.surface.gr_context.flush_and_submit();
    }
}
