use downcast_rs::{impl_downcast, Downcast};

pub trait Layers: Downcast {
    fn render(&self, canvas: &skia::canvas::Canvas, paint: &mut skia::Paint);
}

impl_downcast!(Layers);
