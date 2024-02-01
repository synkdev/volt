use super::Context;
use glutin::prelude::GlSurface;

impl Context {
    pub fn render(&mut self) {
        self.draw();
        self.finish_render();
    }

    pub fn start_render(&mut self) {
        self.draw();
    }

    pub fn draw(&mut self) {
        let canvas = self.surface.surface.canvas();

        if self.dirty {
            canvas.clear(self.background);
            for (_, component) in self.components.iter_mut() {
                component.render(canvas, &mut self.paint);
            }
            self.dirty = false
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
