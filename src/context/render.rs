use crate::ui::Element;

use super::Context;
use glutin::prelude::GlSurface;

impl Context {
    pub fn render(&mut self) {
        let canvas = self.surface.surface.canvas();
        if self.root.full_redraw {
            canvas.clear(self.background);
            self.root.full_redraw = false;
        }
        if self.root.dirty {
            self.root.render(canvas, &mut self.paint);
            self.root.set_dirty(false);
            self.finish_render();
        }
    }

    pub fn finish_render(&mut self) {
        self.gr_context
            .gl_surface
            .swap_buffers(&self.gr_context.gl_context)
            .unwrap();
        self.surface.gr_context.flush_and_submit();
    }
}
