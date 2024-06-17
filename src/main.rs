use renderer_backend::pipeline_builder::PipelineBuilder;
mod renderer_backend;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages,
};
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoopBuilder,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

const SCREEN_SIZE: (u32, u32) = (1200, 600);
const TIME_BETWEEN_FRAMES: u64 = 17;

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    previous_frame_texture: wgpu::Texture,
    previous_frame_view: wgpu::TextureView,
    previous_frame_bind_group: wgpu::BindGroup,
    previous_frame_bind_group_layout: wgpu::BindGroupLayout,
    intermediate_texture: wgpu::Texture,
    intermediate_texture_view: wgpu::TextureView,
}

impl<'a> State<'a> {
    async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();

        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance.create_surface(window).unwrap();

        let adapter_descriptor = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Create bind group layout and bind group for sphere data
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Sphere Bind Group Layout"),
        });

        // Create a temporary bind group
        let temp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Temporary Bind Group"),
            layout: &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[],
                label: Some("Temporary Bind Group Layout"),
            }),
            entries: &[],
        });

        // Texture to hold the previous frame's data
        let previous_frame_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Previous Frame Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config.format, // Use the same format as your surface
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        // Create a view for the previous frame texture
        let previous_frame_view =
            previous_frame_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a bind group layout for the previous frame texture
        let previous_frame_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 1, // Choose a different binding index than your sphere data
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    // Add a sampler binding if needed
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Previous Frame Bind Group Layout"),
            });

        // Create the bind group for the previous frame texture
        let previous_frame_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &previous_frame_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&previous_frame_view),
                },
                // Add a sampler if needed
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(
                        &device.create_sampler(&wgpu::SamplerDescriptor::default()),
                    ),
                },
            ],
            label: Some("Previous Frame Bind Group"),
        });

        // Pass bind group layout to pipeline builder
        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        pipeline_builder.set_bind_group_layout(bind_group_layout);
        let render_pipeline =
            pipeline_builder.build_pipeline(&device, &previous_frame_bind_group_layout);

        // Create intermediate texture with COPY_SRC usage
        let intermediate_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Intermediate Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        // Create a view for the intermediate texture
        let intermediate_texture_view =
            intermediate_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            bind_group: temp_bind_group,
            previous_frame_texture,
            previous_frame_view,
            previous_frame_bind_group,
            previous_frame_bind_group_layout,
            intermediate_texture,
            intermediate_texture_view,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };
        let mut command_encoder = self
            .device
            .create_command_encoder(&command_encoder_descriptor);
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &self.intermediate_texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.75,
                    g: 0.5,
                    b: 0.25,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]); // Bind sphere data
            render_pass.set_bind_group(1, &self.previous_frame_bind_group, &[]); // Bind previous frame texture
            render_pass.draw(0..3, 0..1); // Draw the first triangle
            render_pass.draw(3..6, 0..1); // Draw the second triangle
        }

        // Copy to previous frame texture
        command_encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &self.intermediate_texture, // Source texture
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &self.previous_frame_texture, // Destination texture
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
        );

        // Copy to the surface texture
        command_encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &self.intermediate_texture, // Source texture
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &drawable.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    Timer,
}

async fn run() {
    env_logger::init();

    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build(&event_loop)
        .unwrap();
    let event_loop_proxy = event_loop.create_proxy();

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(TIME_BETWEEN_FRAMES));
        event_loop_proxy.send_event(CustomEvent::Timer).ok();
    });

    let mut state = State::new(&window).await;

    // Create sphere data - Format: [x, y, z, radius, r, g, b, er, eg, eb, emission_strength]
    let mut sphere_data: Vec<Vec<f32>> = Vec::new();
    sphere_data.push(vec![
        40.0, 0.0, 0.0, 10.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ]);
    sphere_data.push(vec![
        -40.0, 0.0, 0.0, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ]);
    sphere_data.push(vec![
        0.0, 40.0, 0.0, 10.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    ]);
    sphere_data.push(vec![
        0.0, -40.0, 0.0, 10.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ]);
    sphere_data.push(vec![
        0.0, 0.0, 40.0, 10.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    ]);
    sphere_data.push(vec![
        0.0, 0.0, -40.0, 10.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    ]);
    sphere_data.push(vec![0.0, 0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 8.0]); // Light source

    let sphere_data_u8: Vec<u8> = sphere_data
        .iter()
        .flat_map(|s| s.iter().map(|f| f.to_ne_bytes().to_vec()).flatten())
        .collect();

    // Create storage buffer for sphere data
    let sphere_buffer = state.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Sphere Buffer Data"),
        contents: bytemuck::cast_slice(&sphere_data_u8),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    // Write sphere data to buffer
    state
        .queue
        .write_buffer(&sphere_buffer, 0, bytemuck::cast_slice(&sphere_data_u8));

    // Create bind group layout and bind group for sphere data
    let bind_group_layout =
        state
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Sphere Bind Group Layout"),
            });
    state.bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Sphere Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: sphere_buffer.as_entire_binding(),
        }],
    });

    // Pass bind group layout to pipeline builder
    let mut pipeline_builder = PipelineBuilder::new();
    pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
    pipeline_builder.set_pixel_format(state.config.format);
    pipeline_builder.set_bind_group_layout(bind_group_layout);
    state.render_pipeline =
        pipeline_builder.build_pipeline(&state.device, &state.previous_frame_bind_group_layout);

    event_loop
        .run(move |event, elwt| match event {
            Event::UserEvent(..) => {
                state.window.request_redraw();
            }

            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == state.window.id() => match event {
                WindowEvent::Resized(physical_size) => state.resize(*physical_size),

                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            repeat: false,
                            ..
                        },
                    ..
                } => {
                    println!("Goodbye see you!");
                    elwt.exit();
                }

                WindowEvent::RedrawRequested => match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    Err(e) => eprintln!("{:?}", e),
                },

                _ => (),
            },

            _ => {}
        })
        .expect("Error!");
}

fn main() {
    pollster::block_on(run());
}
