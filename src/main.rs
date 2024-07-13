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
const CAMERA_SPEED: f32 = 1.0;
const CAMERA_ROT_SPEED: f32 = 1.0;

const meshes: [&str; 2] = ["../objects/knight.bin", "../objects/teapot.bin"];

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: &'a Window,
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    frame_count: u32,
    frame_count_buffer: wgpu::Buffer,
    frame_data_buffer: wgpu::Buffer,
    sphere_buffer: wgpu::Buffer,
    triangle_buffer: wgpu::Buffer,
    mesh_buffer: wgpu::Buffer,
    bounding_box_buffer: wgpu::Buffer,
    camera_position: [f32; 3],
    camera_rotation: [f32; 3],
    camera_position_buffer: wgpu::Buffer,
    camera_rotation_buffer: wgpu::Buffer,
    keys_pressed: [bool; 12], // [W, S, D, A, Space, Shift, I, K, L, J, O, U]
}

impl<'a> State<'a> {
    // Update the camera position based on the keys pressed
    fn update_camera(&mut self) {
        // Reset frame count if any key is pressed
        if self.keys_pressed.iter().any(|&k| k) {
            self.frame_count = 0;
        }

        let mut local_movement = [0.0, 0.0, 0.0];
        let mut local_rotation = [0.0, 0.0, 0.0];

        if self.keys_pressed[0] {
            // W
            local_movement[2] -= CAMERA_SPEED;
        }
        if self.keys_pressed[1] {
            // S
            local_movement[2] += CAMERA_SPEED;
        }
        if self.keys_pressed[2] {
            // D
            local_movement[0] += CAMERA_SPEED;
        }
        if self.keys_pressed[3] {
            // A
            local_movement[0] -= CAMERA_SPEED;
        }
        if self.keys_pressed[4] {
            // Space
            local_movement[1] += CAMERA_SPEED;
        }
        if self.keys_pressed[5] {
            // Shift
            local_movement[1] -= CAMERA_SPEED;
        }

        if self.keys_pressed[6] {
            // I
            local_rotation[0] += CAMERA_ROT_SPEED;
        }
        if self.keys_pressed[7] {
            // K
            local_rotation[0] -= CAMERA_ROT_SPEED;
        }
        if self.keys_pressed[8] {
            // L
            local_rotation[1] += CAMERA_ROT_SPEED;
        }
        if self.keys_pressed[9] {
            // J
            local_rotation[1] -= CAMERA_ROT_SPEED;
        }
        if self.keys_pressed[10] {
            // O
            local_rotation[2] += CAMERA_ROT_SPEED;
        }
        if self.keys_pressed[11] {
            // U
            local_rotation[2] -= CAMERA_ROT_SPEED;
        }

        // Convert local to global
        let global_movement = self.rotate_vector(local_movement, self.camera_rotation);
        let global_rotation = self.rotate_vector(local_rotation, self.camera_rotation);

        // Update new camera position and rotation
        self.camera_position[0] += global_movement[0];
        self.camera_position[1] += global_movement[1];
        self.camera_position[2] += global_movement[2];

        self.camera_rotation[0] += global_rotation[0];
        self.camera_rotation[1] += global_rotation[1];
        self.camera_rotation[2] += global_rotation[2];
    }

    fn rotate_vector(&mut self, vector: [f32; 3], rotation: [f32; 3]) -> [f32; 3] {
        let x = vector[0];
        let y = vector[1];
        let z = vector[2];

        let a = rotation[0].to_radians();
        let b = rotation[1].to_radians();
        let c = rotation[2].to_radians();

        let cos_a = a.cos();
        let sin_a = a.sin();
        let cos_b = b.cos();
        let sin_b = b.sin();
        let cos_c = c.cos();
        let sin_c = c.sin();

        let x_rot = x * cos_c * cos_b
            + y * (cos_c * sin_b * sin_a - sin_c * cos_a)
            + z * (cos_c * sin_b * cos_a + sin_c * sin_a);
        let y_rot = x * sin_c * cos_b
            + y * (sin_c * sin_b * sin_a + cos_c * cos_a)
            + z * (sin_c * sin_b * cos_a - cos_c * sin_a);
        let z_rot = -x * sin_b + y * cos_b * sin_a + z * cos_b * cos_a;

        [x_rot, y_rot, z_rot]
    }

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

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("Sphere Bind Group Layout"),
        });

        // Pass bind group layout to pipeline builder
        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        pipeline_builder.set_bind_group_layout(bind_group_layout);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        // Create a temporary bind group
        let temp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Temporary Bind Group"),
            layout: &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[],
                label: Some("Temporary Bind Group Layout"),
            }),
            entries: &[],
        });

        // Create sphere data - Format: [x, y, z, radius, r, g, b, er, eg, eb, emission_strength, smoothness]
        let mut sphere_data: Vec<Vec<f32>> = Vec::new();
        // sphere_data.push(vec![
        //     -40.0, 0.0, 0.0, 10.0, 0.25, 0.25, 1.0, 0.0, 0.0, 0.0, 0.0, 0.9,
        // ]);
        // sphere_data.push(vec![
        //     40.0, 0.0, 0.0, 10.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        // ]);
        // sphere_data.push(vec![
        //     0.0, -20.0, 0.0, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        // ]);
        // sphere_data.push(vec![
        //     0.0, -5030.0, 0.0, 5000.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        // ]);
        sphere_data.push(vec![
            20.0, 40.0, -20.0, 10.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 10.0, 0.0,
        ]); // Light source

        let sphere_data_u8: Vec<u8> = sphere_data
            .iter()
            .flat_map(|s| s.iter().map(|f| f.to_ne_bytes().to_vec()).flatten())
            .collect();

        // Buffer for sphere data
        let sphere_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Sphere Buffer Data"),
            contents: bytemuck::cast_slice(&sphere_data_u8),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Write sphere data to buffer
        queue.write_buffer(&sphere_buffer, 0, bytemuck::cast_slice(&sphere_data_u8));

        // Load triangle data
        let triangle_data = include_bytes!("../objects/knight.bin");
        let triangle_data = triangle_data.to_vec();
        let triangle_data = triangle_data.chunks(4).collect::<Vec<_>>();
        let triangle_data = triangle_data.iter().map(|d| f32::from_ne_bytes([d[0], d[1], d[2], d[3]])).collect::<Vec<_>>();

        let triangle_data_u8 = triangle_data.iter().map(|f| f.to_ne_bytes().to_vec()).flatten().collect::<Vec<_>>();

        // Buffer for triangle data
        let triangle_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Triangle Buffer Data"),
            contents: bytemuck::cast_slice(&triangle_data_u8),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Write triangle data to buffer
        queue.write_buffer(&triangle_buffer, 0, bytemuck::cast_slice(&triangle_data_u8));

        // Create a bounding box for the triangles
        let mut bounding_box: Vec<Vec<f32>> = vec![vec![f32::MAX; 3], vec![f32::MIN; 3]];
        for i in 0..triangle_data.len() / 9 {
            for j in 0..3 {
                for k in 0..3 {
                    bounding_box[0][k] = bounding_box[0][k].min(triangle_data[i * 9 + j * 3 + k]);
                    bounding_box[1][k] = bounding_box[1][k].max(triangle_data[i * 9 + j * 3 + k]);
                }
            }
        }

        let bounding_box: Vec<f32> = vec![bounding_box[0][0], bounding_box[0][1], bounding_box[0][2], bounding_box[1][0], bounding_box[1][1], bounding_box[1][2]];
        
        // // Convert bounding box to u8
        let bounding_box_u8: Vec<u8> = bounding_box.iter().map(|f| f.to_ne_bytes().to_vec()).flatten().collect();

        // Buffer for the bounding box
        let bounding_box_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Bounding Box Buffer Data"),
            contents: bytemuck::cast_slice(&bounding_box_u8),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Buffer for the frame count
        let frame_count_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Frame Count Buffer"),
            contents: bytemuck::cast_slice(&[0]),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Frame data that starts off completely black
        let frame_data =
            vec![vec![vec![0.0; 3]; size.width as usize]; ((size.height as f32) * 1.5) as usize];
        let frame_data_flat: Vec<f32> = frame_data.iter().flatten().flatten().copied().collect();
        let frame_data_u8: Vec<u8> = frame_data_flat
            .iter()
            .map(|f| f.to_ne_bytes().to_vec())
            .flatten()
            .collect();

        // Buffer for the frame data
        let frame_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Frame Data Buffer"),
            contents: bytemuck::cast_slice(&frame_data_u8),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Camera data
        let camera_position = [200.0, 0.0, 200.0];
        let camera_rotation = [0.0, 90.0, 0.0];

        // Buffer for the camera position
        let camera_position_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Position Buffer"),
            contents: bytemuck::cast_slice(&camera_position),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Buffer for the camera rotation
        let camera_rotation_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Rotation Buffer"),
            contents: bytemuck::cast_slice(&camera_rotation),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        });

        // Write data to camera buffers
        queue.write_buffer(
            &camera_position_buffer,
            0,
            bytemuck::cast_slice(&camera_position),
        );
        queue.write_buffer(
            &camera_rotation_buffer,
            0,
            bytemuck::cast_slice(&camera_rotation),
        );

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            bind_group: temp_bind_group,
            frame_count: 0,
            frame_count_buffer,
            frame_data_buffer,
            sphere_buffer,
            triangle_buffer,
            bounding_box_buffer,
            camera_position,
            camera_rotation,
            camera_position_buffer,
            camera_rotation_buffer,
            keys_pressed: [false; 12],
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
        // Update the frame count buffer before rendering
        self.queue.write_buffer(
            &self.frame_count_buffer,
            0,
            bytemuck::cast_slice(&[self.frame_count]),
        );

        // Update camera position and rotation buffers before rendering
        self.queue.write_buffer(
            &self.camera_position_buffer,
            0,
            bytemuck::cast_slice(&self.camera_position),
        );
        self.queue.write_buffer(
            &self.camera_rotation_buffer,
            0,
            bytemuck::cast_slice(&self.camera_rotation),
        );

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
            view: &image_view,
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
            render_pass.set_bind_group(0, &self.bind_group, &[]); // Access using self
            render_pass.draw(0..3, 0..1); // Draw the first triangle
            render_pass.draw(3..6, 0..1); // Draw the second triangle
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        self.frame_count += 1;

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

    // Create bind group layout
    let bind_group_layout =
        state
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 6,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("Sphere Bind Group Layout"),
            });
    state.bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Sphere Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: state.sphere_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: state.frame_count_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: state.frame_data_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: state.camera_position_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: state.camera_rotation_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: state.triangle_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 6,
                resource: state.bounding_box_buffer.as_entire_binding(),
            },
        ],
    });

    // Pass bind group layout to pipeline builder
    let mut pipeline_builder = PipelineBuilder::new();
    pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
    pipeline_builder.set_pixel_format(state.config.format);
    pipeline_builder.set_bind_group_layout(bind_group_layout);
    state.render_pipeline = pipeline_builder.build_pipeline(&state.device);

    event_loop
        .run(move |event, elwt| match event {
            Event::UserEvent(..) => {
                state.update_camera();
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
                    println!("Closing window");
                    elwt.exit();
                }

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key,
                            state: element_state,
                            ..
                        },
                    ..
                } => {
                    let pressed = *element_state == ElementState::Pressed;
                    match physical_key {
                        PhysicalKey::Code(KeyCode::KeyW) => state.keys_pressed[0] = pressed,
                        PhysicalKey::Code(KeyCode::KeyS) => state.keys_pressed[1] = pressed,
                        PhysicalKey::Code(KeyCode::KeyD) => state.keys_pressed[2] = pressed,
                        PhysicalKey::Code(KeyCode::KeyA) => state.keys_pressed[3] = pressed,
                        PhysicalKey::Code(KeyCode::Space) => state.keys_pressed[4] = pressed,
                        PhysicalKey::Code(KeyCode::ShiftLeft) => state.keys_pressed[5] = pressed,
                        PhysicalKey::Code(KeyCode::KeyI) => state.keys_pressed[6] = pressed,
                        PhysicalKey::Code(KeyCode::KeyK) => state.keys_pressed[7] = pressed,
                        PhysicalKey::Code(KeyCode::KeyL) => state.keys_pressed[8] = pressed,
                        PhysicalKey::Code(KeyCode::KeyJ) => state.keys_pressed[9] = pressed,
                        PhysicalKey::Code(KeyCode::KeyO) => state.keys_pressed[10] = pressed,
                        PhysicalKey::Code(KeyCode::KeyU) => state.keys_pressed[11] = pressed,

                        _ => {}
                    }
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
