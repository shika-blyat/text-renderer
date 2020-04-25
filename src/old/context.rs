use wgpu::*;
use winit::window::Window;

pub struct Frame<'a> {
    pub frame: SwapChainOutput,
    pub encoder: CommandEncoder,
    pub ctx: &'a GfxContext,
}

impl<'a> Frame<'a> {
    pub fn finish(self) {
        self.ctx.queue.submit(&[self.encoder.finish()]);
    }
}
pub struct GfxContext {
    pub window: Window,
    pub swapchain: SwapChain,
    pub sc_desc: SwapChainDescriptor,
    pub surface: Surface,
    pub device: Device,
    pub queue: Queue,
    pub adapter: Adapter,
    pub size: (u32, u32),
    pub render_format: TextureFormat,
}
impl GfxContext {
    pub fn new(window: Window) -> Self {
        let surface = wgpu::Surface::create(&window);

        // Initialize GPU
        let adapter = futures::executor::block_on(wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::all(),
        ))
        .expect("Request adapter");
        let (device, queue) = futures::executor::block_on(async {
            adapter
                .request_device(&wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits { max_bind_groups: 1 },
                })
                .await
        });

        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let size = window.inner_size();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: render_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swapchain = device.create_swap_chain(&surface, &sc_desc);
        Self {
            surface,
            adapter,
            window,
            render_format,
            size: (size.width, size.height),
            device,
            queue,
            swapchain,
            event_loop,
            sc_desc,
        }
    }
    pub fn next_frame(&mut self) -> Frame {
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Redraw"),
            });

        let frame = self.swapchain.get_next_texture().expect("Get next frame");
        Frame {
            ctx: self,
            frame,
            encoder,
        }
    }
}
