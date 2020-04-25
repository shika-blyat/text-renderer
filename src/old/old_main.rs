mod apply_keycode;
mod context;
mod widgets;

use apply_keycode::apply_keycode;
use wgpu_glyph::{GlyphBrushBuilder, Scale, Section};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use context::GfxContext;
use widgets::text_box::TextBox;

const FONT: &[u8] = include_bytes!("../ttf/JetBrainsMono-Regular.ttf");

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500.0, 500.0))
        .build(&event_loop)
        .expect("Failed to create window");
    let mut ctx = GfxContext::new(window);
    let mut glyph_brush = GlyphBrushBuilder::using_font_bytes(FONT)
        .expect("Load fonts")
        .build(&ctx.device, ctx.render_format);
    let mut text_box = TextBox::new("Abc");
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(new_size) => {
                ctx.size = (new_size.width, new_size.height);

                ctx.swapchain = ctx.device.create_swap_chain(
                    &ctx.surface,
                    &wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                        format: ctx.render_format,
                        width: new_size.width,
                        height: new_size.height,
                        present_mode: wgpu::PresentMode::Mailbox,
                    },
                );
            }
            WindowEvent::ReceivedCharacter(c) => text_box.push(c),
            WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } if input.state == ElementState::Pressed => {
                match input.virtual_keycode {
                    Some(keycode) => apply_keycode(&mut text_box, keycode),
                    None => (),
                }
            }
            _ => (),
        },
        Event::MainEventsCleared => {
            ctx.window.request_redraw();
        }
        Event::RedrawRequested { .. } => {
            let mut frame = ctx.next_frame();
            text_box.draw(&mut frame, &mut glyph_brush);
            glyph_brush
                .draw_queued(
                    &frame.ctx.device,
                    &mut frame.encoder,
                    &frame.frame.view,
                    frame.ctx.size.0,
                    frame.ctx.size.1,
                )
                .expect("Draw queued");
            frame.finish();
        }
        _ => {
            *control_flow = winit::event_loop::ControlFlow::Poll;
        }
    })
}
