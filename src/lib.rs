pub mod color;
pub mod div;
pub mod element;
pub mod layout;
pub mod styles;
pub mod window;

use anyhow::Result;
use div::Div;
use element::Element;
use std::{num::NonZeroUsize, sync::Arc};
use taffy::{style_helpers::TaffyMaxContent, Size, Style, TaffyTree};
use vello::{
    peniko::Color,
    util::{RenderContext, RenderSurface},
    AaConfig, Renderer, RendererOptions, Scene,
};
use window::WindowOptions;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct RenderState<'s> {
    surface: RenderSurface<'s>,
    window: Arc<Window>,
}

pub struct Volt {
    pub(crate) renderers: Vec<Option<Renderer>>,
    pub root: Div,
    pub tree: TaffyTree,
}

impl Volt {
    pub fn new() -> Self {
        let mut tree = TaffyTree::new();
        let root_div = Div::default();
        let root_div_node = tree
            .new_leaf(Style {
                size: Size::from_percent(100.0, 100.0),
                ..Default::default()
            })
            .unwrap();
        let root = tree
            .new_with_children(
                Style {
                    size: Size::from_lengths(500.0, 400.0),
                    ..Default::default()
                },
                &[root_div_node],
            )
            .unwrap();
        tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
        Volt {
            renderers: vec![],
            root: Div::default(),
            tree,
        }
    }
    pub fn render(&mut self, event_loop: EventLoop<()>, mut render_cx: RenderContext) {
        let mut render_state = None::<RenderState>;
        let mut cached_window = None;
        let mut scene = Scene::new();
        event_loop
            .run(move |event, event_loop| match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    let Some(render_state) = &mut render_state else {
                        return;
                    };
                    if render_state.window.id() != window_id {
                        return;
                    }
                    match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::Resized(size) => render_cx.resize_surface(
                            &mut render_state.surface,
                            size.width,
                            size.height,
                        ),
                        WindowEvent::RedrawRequested => {
                            let width = render_state.surface.config.width;
                            let height = render_state.surface.config.height;
                            let device_handle = &render_cx.devices[render_state.surface.dev_id];
                            render_state.window.set_title("Volt Example");
                            let surface_texture = render_state
                                .surface
                                .surface
                                .get_current_texture()
                                .expect("failed to get surface texture");
                            let render_params = vello::RenderParams {
                                base_color: Color::BLACK,
                                width,
                                height,
                                antialiasing_method: AaConfig::Msaa16,
                            };
                            scene.reset();
                            self.root.render(&mut scene);

                            vello::block_on_wgpu(
                                &device_handle.device,
                                self.renderers[render_state.surface.dev_id]
                                    .as_mut()
                                    .expect("Couldnt fetch renderer")
                                    .render_to_surface_async(
                                        &device_handle.device,
                                        &device_handle.queue,
                                        &scene,
                                        &surface_texture,
                                        &render_params,
                                    ),
                            )
                            .expect("failed to render to surface");
                            surface_texture.present();
                            device_handle.device.poll(wgpu::Maintain::Poll);
                        }
                        _ => {}
                    }
                }
                Event::Suspended => {
                    if let Some(render_state) = render_state.take() {
                        cached_window = Some(render_state.window);
                    }
                    event_loop.set_control_flow(ControlFlow::Wait);
                }
                Event::Resumed => {
                    let Option::None = render_state else { return };
                    let window = cached_window
                        .take()
                        .unwrap_or_else(|| window::new(event_loop, WindowOptions::default()));
                    let size = window.inner_size();
                    let surface_future =
                        render_cx.create_surface(window.clone(), size.width, size.height);
                    let surface =
                        pollster::block_on(surface_future).expect("Error creating surface");
                    render_state = {
                        let render_state = RenderState { window, surface };
                        self.renderers.resize_with(render_cx.devices.len(), || None);
                        let id = render_state.surface.dev_id;
                        self.renderers[id].get_or_insert_with(|| {
                            Renderer::new(
                                &render_cx.devices[id].device,
                                RendererOptions {
                                    surface_format: Some(render_state.surface.format),
                                    use_cpu: false,
                                    antialiasing_support: vello::AaSupport::all(),
                                    num_init_threads: NonZeroUsize::new(1),
                                },
                            )
                            .expect("Could create renderer")
                        });
                        Some(render_state)
                    };
                    event_loop.set_control_flow(ControlFlow::Poll);
                }
                _ => {}
            })
            .expect("Couldnt run event loop");
    }
    pub fn run() -> Result<()> {
        let mut volt = Volt::new();
        volt.render(EventLoop::new()?, RenderContext::new().unwrap());
        Ok(())
    }
}
