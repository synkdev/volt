use crate::ui::Element;

use super::Context;
use glutin::prelude::GlSurface;

impl Context {
    pub fn render(&mut self) {
        let canvas = self.surface.surface.canvas();
        if self.root.full_redraw {
            println!("root needs full redraw");
            canvas.clear(self.background);
            self.root.full_redraw = false;
            self.root.render(canvas, &mut self.paint);
        }
        self.root.find_dirty_children();
        println!("{:?}", self.root.dirty_children.first());
        if self.root.is_dirty() {
            println!("root is dirty");
            self.root.render(canvas, &mut self.paint);
            self.finish_render();
        } else if self.root.dirty_children.len() > 0 {
            println!("root not dirty, rendering children");
            self.root.render_children(canvas, &mut self.paint);
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
