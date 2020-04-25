use super::widget::Widget;
use crate::context::Frame;

use wgpu_glyph::{GlyphBrush, Scale, Section};

pub struct TextBox {
    content: String,
}

impl TextBox {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into(),
        }
    }
    pub fn push(&mut self, c: char) {
        self.content.push(c);
    }
    pub fn pop(&mut self) {
        self.content.pop();
    }
    pub fn draw(&self, frame: &mut Frame, brush: &mut GlyphBrush<'_, ()>) {
        {
            let _ = frame
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.4,
                            g: 0.4,
                            b: 0.4,
                            a: 1.0,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
        }

        brush.queue(Section {
            text: self.content.as_str(),
            screen_position: (30.0, 30.0),
            color: [0.0, 0.0, 0.0, 1.0],
            scale: Scale { x: 40.0, y: 40.0 },
            bounds: (frame.ctx.size.0 as f32, frame.ctx.size.0 as f32),
            ..Section::default()
        });
    }
}
impl Widget for TextBox {}
