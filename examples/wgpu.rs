use std::{
    os::fd::{AsFd, AsRawFd},
    time::{Duration, Instant},
};

use anyhow::{Context, Result, bail};
use diretto::{
    ClientCapability, Connector, Device, ModeType, Resources, sys::DRM_MODE_OBJECT_PLANE,
};
use raw_window_handle::{DisplayHandle, DrmDisplayHandle, DrmWindowHandle, WindowHandle};
use rustix::fs::{self, Mode, OFlags};
use tracing::{debug, trace};
use wgpu::SurfaceTargetUnsafe;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let device = find_drm_device()?;
    let resources = device.get_resources()?;
    let connector = find_drm_connector(&device, &resources)?;

    let mode = {
        let mut mode = None;

        let mut area = 0;

        for current_mode in connector.modes {
            if current_mode.ty().contains(ModeType::PREFERRED) {
                mode = Some(current_mode);
                break;
            }

            let current_area = current_mode.display_width() * current_mode.display_height();
            if current_area > area {
                mode = Some(current_mode);
                area = current_area;
            }
        }

        mode.expect("Couldn't find a mode")
    };

    debug!(
        "Selected mode {}x{}@{}",
        mode.display_width(),
        mode.display_height(),
        mode.vertical_refresh_rate()
    );

    device.set_client_capability(ClientCapability::Atomic, true)?;

    let plane_resources = device.get_plane_resources()?;

    let mut plane = None;

    for id in plane_resources {
        debug!("Found plane {id}");
        let (props, values) = unsafe { device.get_properties(id, DRM_MODE_OBJECT_PLANE)? };

        trace!("Properties for plane {id}:");
        for (index, prop) in props.into_iter().enumerate() {
            let (name, possible_values) = unsafe { device.get_property(prop)? };
            let current_value = values[index];

            trace!(
                "  Property '{}' = {} (possible values: {:?})",
                name.to_string_lossy(),
                current_value,
                possible_values
            );

            if name.as_c_str() == c"type" {
                match current_value {
                    1 => {
                        trace!("    This is a primary plane");
                        plane = Some(id)
                    }
                    2 => trace!("    This is an overlay plane"),
                    3 => trace!("    This is a cursor plane"),
                    _ => trace!("    Unknown plane type"),
                }
            }
        }
    }

    let plane = plane.expect("Failed to find an appropriate plane");

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

    let instance = wgpu::Instance::default();

    let surface_target = SurfaceTargetUnsafe::RawHandle {
        raw_display_handle: display_handle.as_raw(),
        raw_window_handle: window_handle.as_raw(),
    };

    let surface = unsafe { instance.create_surface_unsafe(surface_target)? };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
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
            .expect("failed to acquire next swapchain texture");

        let texture_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        // Create the renderpass which will clear the screen.
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
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

        // If you wanted to call any drawing commands, they would go here.

        // End the renderpass.
        drop(renderpass);

        // Submit the command in the queue to execute
        queue.submit([encoder.finish()]);

        frame.present();
    }

    Ok(())
}

fn find_drm_device() -> Result<Device> {
    // TODO: implement an actual strategy
    let fd = fs::open(
        "/dev/dri/card1",
        OFlags::RDWR | OFlags::NONBLOCK,
        Mode::empty(),
    )?;
    let device = unsafe { Device::new_unchecked(fd) };

    debug!("Opened device /dev/dri/card1");

    Ok(device)
}

fn find_drm_connector(device: &Device, resources: &Resources) -> Result<Connector> {
    for connector_id in &resources.connectors {
        let connector = device.get_connector(*connector_id, false)?;
        if connector.connection.is_connected() {
            return Ok(connector);
        }
    }

    bail!("No connected display found")
}
