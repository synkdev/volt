use anyhow::Result;
use instant::{Duration, Instant};
use std::num::NonZeroUsize;
use std::{collections::HashSet, sync::Arc};
use vello::peniko::Color;
use vello::util::RenderSurface;
use vello::{
    kurbo::{Affine, Vec2},
    util::RenderContext,
    AaConfig, Renderer, Scene,
};
use vello::{BumpAllocators, RendererOptions};

use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::Window,
};

struct RenderState<'s> {
    surface: RenderSurface<'s>,
    window: Arc<Window>,
}
