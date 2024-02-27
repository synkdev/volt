pub mod color;
pub mod context;
pub mod div;
pub mod element;
pub mod layout;
pub mod styles;
pub mod node;
pub mod window;

use std::num::NonZeroUsize;

use anyhow::Result;
use context::Context;
use div::Div;
use element::Element;
use taffy::{NodeId, TaffyTree};
use vello::{
	peniko::Color,
	util::{RenderContext, RenderSurface},
	AaConfig, AaSupport, Renderer, RendererOptions, Scene,
};
use window::WindowOptions;
use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
};

pub struct Volt<'s> {
	/// Vello renderer stuff
	pub(crate) renderer: Renderer,
	pub(crate) surface: RenderSurface<'s>,
	pub(crate) event_loop: EventLoop<()>,
	pub(crate) render_cx: RenderContext,
	pub(crate) scene: Scene,
	pub cx: Context,
}

impl<'s> Volt<'s> {
	pub async fn new() -> Self {
		let tree = TaffyTree::new();
		// let root_div = Div::default();
		// let root_div_node = tree
		// 	.new_leaf(Style {
		// 		size: Size::from_percent(100.0, 100.0),
		// 		..Default::default()
		// 	})
		// 	.unwrap();
		// let root = tree
		// 	.new_with_children(
		// 		Style {
		// 			size: Size::from_lengths(500.0, 400.0),
		// 			..Default::default()
		// 		},
		// 		&[root_div_node],
		// 	)
		// 	.unwrap();
		// tree.compute_layout(root, Size::MAX_CONTENT).unwrap();
		let mut render_cx = RenderContext::new().expect("Couldn't create a Vello RenderContext");
		let event_loop = EventLoop::new().expect("Couldn't create event loop");
		let window = window::new(&event_loop, WindowOptions::default());
		let size = window.inner_size();
		let surface = render_cx
			.create_surface(window.clone(), size.width, size.height)
			.await
			.expect("Error creating surface");
		let renderer = Renderer::new(
			&render_cx.devices[surface.dev_id].device,
			RendererOptions {
				surface_format: Some(surface.config.format),
				use_cpu: false,
				antialiasing_support: AaSupport::all(),
				num_init_threads: NonZeroUsize::new(1),
			},
		)
		.expect("Couldn't create Vello Renderer");
		let scene = Scene::new();
		let cx = Context::new();

		Volt {
			renderer,
			render_cx,
			scene,
			surface,
			event_loop,
			cx,
		}
	}

	pub fn render(mut self) {
		self.event_loop
			.run(move |event, event_loop| match event {
				Event::WindowEvent { ref event, .. } => match event {
					WindowEvent::CloseRequested => event_loop.exit(),
					WindowEvent::Resized(size) => {
						self.render_cx
							.resize_surface(&mut self.surface, size.width, size.height)
					}
					WindowEvent::RedrawRequested => {
						let width = self.surface.config.width;
						let height = self.surface.config.height;
						let device_handle = &self.render_cx.devices[self.surface.dev_id];
						let surface_texture = self
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
						self.scene.reset();
						// self.root.render(&mut scene);
						Div::default().render(&mut self.cx, &mut self.scene);

						vello::block_on_wgpu(
							&device_handle.device,
							self.renderer.render_to_surface_async(
								&device_handle.device,
								&device_handle.queue,
								&self.scene,
								&surface_texture,
								&render_params,
							),
						)
						.expect("failed to render to surface");
						surface_texture.present();
						device_handle.device.poll(wgpu::Maintain::Poll);
					}
					_ => {}
				},
				Event::Suspended => {
					event_loop.set_control_flow(ControlFlow::Wait);
				}
				Event::Resumed => {
					event_loop.set_control_flow(ControlFlow::Poll);
				}
				_ => {}
			})
			.expect("Couldnt run event loop");
	}

	pub async fn run() -> Result<()> {
		Volt::new().await.render();
		Ok(())
	}
}
