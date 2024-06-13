use wgpu::{
    Color, CommandEncoder, Device, Queue, RenderPass, StoreOp, Surface, SurfaceConfiguration, TextureFormat, VertexBufferLayout, VertexState
};
use wgpu::util::{DeviceExt};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    hello_world: u32,
}

async fn run(event_loop: &EventLoop<()>) {
    // Create a new instance of wgpu
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    // Get a reference to the primary adapter
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    })
    .await
    .unwrap();

    // Create a window
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(event_loop)
        .unwrap();

    // Create a surface for displaying the output
    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    // Get the capabilities of the surface
    let surface_capabilities = surface.get_capabilities(&adapter);

    // Choose a format for the swapchain
    let format = surface_capabilities.formats[0];

    // Create a surface configuration
    let surface_configuration = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: WIDTH,
        height: HEIGHT,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_capabilities.alpha_modes[0],
        desired_maximum_frame_latency: 2,
        view_formats: Vec::new(),
    };

    // Create a device and queue
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        },
        None,
    )
    .await.unwrap();

    // Create a swapchain
    let swap_chain = surface.configure(&device, &surface_configuration);

    // Create a pipeline layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                has_dynamic_offset: false,
                min_binding_size: None,
                ty: wgpu::BufferBindingType::Uniform,
            },
            count: None,
        }],
        label: None,
    });

    // Create a pipeline
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        })),
        vertex: VertexState {
            module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            }),
            entry_point: "vs_main",
            buffers: &[VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            }),
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
            unclipped_depth: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        label: None,
        multiview: None,
    });

    // Create a vertex buffer
    let vertices = [
        Vertex::new(-1.0, -1.0),
        Vertex::new(1.0, -1.0),
        Vertex::new(1.0, 1.0),
        Vertex::new(-1.0, 1.0),
    ];
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // Create a uniform buffer
    let hello_world = 42;
    let uniforms = Uniforms { hello_world };
    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Create a bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None,
                }),
            },
        ],
        label: None,
    });

    // Run the main loop
    event_loop.run(move |event, _, control_flow| {
        // Update the uniform buffer
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                // Redraw the window
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Get a new frame from the swapchain
                let frame = swap_chain
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture.");
                let view = frame
                    .output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Create a command encoder
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: None,
                    });

                // Create a render pass
                let mut render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    },
                );

                // Draw the triangle
                render_pass.set_pipeline(&pipeline);
                render_pass.set_vertex_buffer(0, &vertex_buffer);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw(0..4, 0..1);

                // Finish the render pass
                drop(render_pass);

                // Submit the command buffer
                queue.submit(std::iter::once(encoder.finish()));

                // Present the frame
                frame.present();
            }
            _ => {}
        }
    });
}

fn main() {
    // Initialize the event loop
    let event_loop = EventLoop::new();

    // Run the main function
    pollster::block_on(run(&event_loop));
}