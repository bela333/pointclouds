use las::Read;
use nalgebra::vector;
use object::{BasicVertex, Object};

use pass::{points_pass::PointsPass, Pass};
use texture_store::{TextureHandle, TextureStore};
use wgpu::{PresentMode, TextureDescriptor};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

mod material;
mod object;
mod pass;
mod texture_store;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = wgpu::Instance::default();

    let surface = instance.create_surface(&window).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty() | wgpu::Features::POLYGON_MODE_POINT,
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities.formats[0];

    // Reserve textures
    let mut texture_store = TextureStore::new();
    let depth_buffer = texture_store.reserve(
        &device,
        &TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
    );

    // Setup objects
    let mut vertices = Vec::new();
    let mut reader = las::Reader::from_path("pointcloud.las").unwrap();
    for point in reader.points() {
        let point = point.unwrap();
        if let Some(color) = point.color {
            vertices.push(BasicVertex {
                position: vector![point.x as f32, point.z as f32-1.0, point.y as f32],
                color: vector![
                    color.red as f32 / 65536.,
                    color.green as f32 / 65536.,
                    color.blue as f32 / 65536.
                ],
            });
        } else {
            vertices.push(BasicVertex {
                position: vector![point.x as f32, point.y as f32, point.z as f32],
                color: vector![0.0, 0.0, 0.0],
            });
        }
    }

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(64),
            },
            count: None,
        }],
    });

    let object1 = object::BasicObject::new(&device, surface_format, &bind_group_layout, vertices);

    let objects: Vec<Box<dyn Object>> = vec![Box::new(object1)];

    // Create passes
    let pointpass = PointsPass::new(
        &device,
        &bind_group_layout,
        objects,
        TextureHandle::get_surface(),
        depth_buffer,
    );
    let passes: Vec<Box<dyn Pass>> = vec![Box::new(pointpass)];

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    config.present_mode = PresentMode::Mailbox;
    surface.configure(&device, &config);

    let window = &window;

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let start_time = std::time::Instant::now();

    event_loop
        .run(move |event, target| {
            // Have the closure take ownership of the resources.
            // `event_loop.run` never returns, therefore we must do this to ensure
            // the resources are properly cleaned up.
            let _ = (&instance, &adapter);

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = &event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        size = PhysicalSize::new(config.width, config.height);
                        surface.configure(&device, &config);
                        // On macos the window needs to be redrawn manually after resizing
                        window.request_redraw();

                        // Update the depth buffer
                        texture_store
                            .recreate(
                                &device,
                                &TextureDescriptor {
                                    label: None,
                                    size: wgpu::Extent3d {
                                        width: size.width,
                                        height: size.height,
                                        depth_or_array_layers: 1,
                                    },
                                    mip_level_count: 1,
                                    sample_count: 1,
                                    dimension: wgpu::TextureDimension::D2,
                                    format: wgpu::TextureFormat::Depth32Float,
                                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                                        | wgpu::TextureUsages::TEXTURE_BINDING,
                                    view_formats: &[],
                                },
                                depth_buffer,
                            )
                            .unwrap();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }

            if let Event::AboutToWait = &event {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let resolver = texture_store.get_resolver(&view);
                    let elapsed = start_time.elapsed();
                    for pass in &passes {
                        pass.render(
                            size.width as f32 / size.height as f32,
                            &queue,
                            &mut encoder,
                            &resolver,
                            elapsed
                        );
                    }
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
        })
        .unwrap();
}

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
