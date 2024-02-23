use anyhow::Result;
use std::num::NonZeroUsize;
use std::sync::Arc;
use vello::kurbo::{RoundedRect, Stroke};
use vello::peniko::Color;
use vello::util::RenderSurface;
use vello::RendererOptions;
use vello::{kurbo::Affine, util::RenderContext, AaConfig, Renderer, Scene};
use winit::{event::*, event_loop::ControlFlow};
use winit::{event_loop::EventLoop, window::Window};

pub struct RenderState<'s> {
    surface: RenderSurface<'s>,
    window: Arc<Window>,
}

pub struct Volt;

impl Volt {
    pub fn create_window(event_loop: &winit::event_loop::EventLoopWindowTarget<()>) -> Arc<Window> {
        use winit::{dpi::LogicalSize, window::WindowBuilder};
        Arc::new(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::new(1044, 800))
                .with_resizable(true)
                .with_title("Volt test")
                .build(event_loop)
                .unwrap(),
        )
    }
    pub fn render(event_loop: EventLoop<()>, mut render_cx: RenderContext) {
        let mut renderers: Vec<Option<Renderer>> = vec![];
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
                            let shape = RoundedRect::new(100.0, 100.0, 400.0, 400.0, 20.0);
                            let stroke = Stroke::new(2.0);
                            let stroke_color = Color::rgb(255.0, 255.0, 255.0);
                            scene.stroke(&stroke, Affine::IDENTITY, stroke_color, None, &shape);
                            vello::block_on_wgpu(
                                &device_handle.device,
                                renderers[render_state.surface.dev_id]
                                    .as_mut()
                                    .unwrap()
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
                        .unwrap_or_else(|| Self::create_window(event_loop));
                    let size = window.inner_size();
                    let surface_future =
                        render_cx.create_surface(window.clone(), size.width, size.height);
                    let surface =
                        pollster::block_on(surface_future).expect("Error creating surface");
                    render_state = {
                        let render_state = RenderState { window, surface };
                        renderers.resize_with(render_cx.devices.len(), || None);
                        let id = render_state.surface.dev_id;
                        renderers[id].get_or_insert_with(|| {
                            eprintln!("Creating renderer {id}");
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
        let event_loop = EventLoop::new()?;
        let render_cx = RenderContext::new().unwrap();
        Self::render(event_loop, render_cx);
        Ok(())
    }
}
