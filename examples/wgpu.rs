use std::{
    borrow::Cow,
    io,
    os::fd::{AsFd, AsRawFd},
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use diretto::drm::{Connector, Device};
use raw_window_handle::{DisplayHandle, DrmDisplayHandle, DrmWindowHandle, WindowHandle};
use rustix::fs::{self, Mode, OFlags};
use tracing::{debug, info};
use wgpu::SurfaceTargetUnsafe;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let fd = fs::open(
        "/dev/dri/card0",
        OFlags::RDWR | OFlags::NONBLOCK,
        Mode::empty(),
    )?;
    let device = unsafe { Device::new_unchecked(fd) };

    info!("Opened device /dev/dri/card0");

    let version = device.version()?;

    debug!(
        "Driver: {} ({}) version {}.{}.{} ({})",
        version.name.to_string_lossy(),
        version.desc.to_string_lossy(),
        version.major,
        version.minor,
        version.patchlevel,
        version.date.to_string_lossy()
    );

    let res = device.get_resources()?;

    let connectors = res
        .connectors
        .iter()
        .map(|id| device.get_connector(*id, true))
        .collect::<io::Result<Vec<Connector>>>()?;

    for connector in &connectors {
        debug!("Found connector {}", connector.connector_id);

        for mode in &connector.modes {
            debug!(
                "Found mode {}@{} for connector {}",
                mode.name().to_string_lossy(),
                mode.vertical_refresh_rate(),
                connector.connector_id
            )
        }
    }

    let connector = connectors
        .into_iter()
        .find(|connector| connector.connection == 1) // 1 means connected
        .unwrap();

    let mode = connector.modes.first().expect("Connector has no modes");

    let planes = device.get_plane_resources()?;

    // FIXME: use a proper strategy to determine the best plane
    let plane = planes[0];

    let display_handle = unsafe {
        DisplayHandle::borrow_raw({
            let handle = DrmDisplayHandle::new(device.as_fd().as_raw_fd());
            handle.into()
        })
    };

    let window_handle = unsafe {
        WindowHandle::borrow_raw({
            let handle =
                DrmWindowHandle::new_with_connector_id(plane, connector.connector_id.into());
            handle.into()
        })
    };

    debug!("Got here?");

    let instance = wgpu::Instance::default();

    debug!("not here?");

    let surface_target = SurfaceTargetUnsafe::RawHandle {
        raw_display_handle: display_handle.as_raw(),
        raw_window_handle: window_handle.as_raw(),
    };

    let surface = unsafe { instance.create_surface_unsafe(surface_target)? };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .context("Failed to find an appropriate adapter")?;

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .context("Failed to create device")?;

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let config = surface
        .get_default_config(
            &adapter,
            mode.display_width().into(),
            mode.display_height().into(),
        )
        .expect("Surface not supported by adapter");

    surface.configure(&device, &config);

    let start = Instant::now();

    while Instant::now().duration_since(start) < Duration::from_secs(5) {
        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }

    Ok(())
}
