use std::{
    ffi::{CStr, CString, c_int},
    fmt::Display,
    io, mem,
    num::NonZeroU32,
    os::fd::{AsFd, BorrowedFd, OwnedFd},
    ptr::null_mut,
    slice,
};

pub mod ioctls;
#[allow(nonstandard_style)]
pub mod sys;

use sys::{
    drm_mode_card_res, drm_mode_create_dumb, drm_mode_crtc, drm_mode_fb_cmd,
    drm_mode_get_connector, drm_mode_get_encoder, drm_mode_get_plane_res, drm_mode_map_dumb,
    drm_mode_modeinfo, drm_version,
};

use rustix::mm::{MapFlags, ProtFlags, mmap, munmap};

#[derive(Debug)]
#[repr(transparent)]
pub struct Device {
    fd: OwnedFd,
}

impl AsFd for Device {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

// FIXME: take into account hotplug
impl Device {
    pub const unsafe fn new_unchecked(fd: OwnedFd) -> Self {
        Self { fd }
    }

    pub fn version(&self) -> io::Result<Version> {
        #[inline]
        unsafe fn ret_vector_to_string(mut vec: Vec<u8>, len: usize) -> String {
            unsafe {
                vec.set_len(len);

                if let Some(last) = vec.last() {
                    if *last != b'\0' {
                        vec.push(b'\0')
                    }
                }

                CString::from_vec_with_nul_unchecked(vec)
                    .to_string_lossy()
                    .into_owned()
            }
        }

        let mut version: drm_version = unsafe { mem::zeroed() };

        unsafe { ioctls::version(self, &mut version)? }

        let mut name_buf: Vec<u8> = create_and_reserve_buf(version.name_len as usize + 1);
        let mut date_buf: Vec<u8> = create_and_reserve_buf(version.date_len as usize + 1);
        let mut desc_buf: Vec<u8> = create_and_reserve_buf(version.desc_len as usize + 1);

        version.name = name_buf.as_mut_ptr().cast();
        version.date = date_buf.as_mut_ptr().cast();
        version.desc = desc_buf.as_mut_ptr().cast();

        unsafe { ioctls::version(self, &mut version)? }

        unsafe {
            let name = ret_vector_to_string(name_buf, version.name_len as usize);
            let date = ret_vector_to_string(date_buf, version.date_len as usize);
            let desc = ret_vector_to_string(desc_buf, version.desc_len as usize);

            Ok(Version {
                major: version.version_major,
                minor: version.version_minor,
                patchlevel: version.version_patchlevel,
                name,
                date,
                desc,
            })
        }
    }

    pub fn set_master(&self) -> io::Result<()> {
        unsafe { ioctls::set_master(self)? }

        Ok(())
    }

    pub fn drop_master(&self) -> io::Result<()> {
        unsafe { ioctls::drop_master(self)? }

        Ok(())
    }

    pub fn get_resources(&self) -> io::Result<Resources> {
        let mut resources: drm_mode_card_res = unsafe { mem::zeroed() };

        unsafe { ioctls::mode_getresources(self, &mut resources)? }

        let mut fbs: Vec<u32> = create_and_reserve_buf(resources.count_fbs as usize);
        let mut crtcs: Vec<CrtcId> = create_and_reserve_buf(resources.count_crtcs as usize);
        let mut connectors: Vec<ConnectorId> =
            create_and_reserve_buf(resources.count_connectors as usize);
        let mut encoders: Vec<EncoderId> =
            create_and_reserve_buf(resources.count_encoders as usize);

        resources.fb_id_ptr = fbs.as_mut_ptr() as _;
        resources.crtc_id_ptr = crtcs.as_mut_ptr() as _;
        resources.connector_id_ptr = connectors.as_mut_ptr() as _;
        resources.encoder_id_ptr = encoders.as_mut_ptr() as _;

        unsafe { ioctls::mode_getresources(self, &mut resources)? }

        unsafe {
            fbs.set_len(resources.count_fbs as usize);
            crtcs.set_len(resources.count_crtcs as usize);
            connectors.set_len(resources.count_connectors as usize);
            encoders.set_len(resources.count_encoders as usize);
        }

        Ok(Resources {
            fbs,
            crtcs,
            connectors,
            encoders,
            min_width: resources.min_width,
            max_width: resources.max_width,
            min_height: resources.min_height,
            max_height: resources.max_height,
        })
    }

    pub fn get_connector(&self, connector_id: ConnectorId, probe: bool) -> io::Result<Connector> {
        let stack_mode: drm_mode_modeinfo = unsafe { mem::zeroed() };
        let mut connector: drm_mode_get_connector = unsafe { mem::zeroed() };

        connector.connector_id = connector_id.into();

        if !probe {
            connector.count_modes = 1;
            connector.modes_ptr = &stack_mode as *const _ as _
        }

        unsafe { ioctls::mode_getconnector(self, &mut connector)? }

        let mut encoders: Vec<u32> = create_and_reserve_buf(connector.count_encoders as usize);
        let mut modes: Vec<Mode> = create_and_reserve_buf(connector.count_modes as usize); // FIXME: handle special case where modes is empty
        let mut props: Vec<u32> = create_and_reserve_buf(connector.count_props as usize);
        let mut prop_values: Vec<u64> = create_and_reserve_buf(connector.count_props as usize);

        connector.encoders_ptr = encoders.as_mut_ptr() as _;
        connector.modes_ptr = modes.as_mut_ptr() as _;
        connector.props_ptr = props.as_mut_ptr() as _;
        connector.prop_values_ptr = prop_values.as_mut_ptr() as _;

        unsafe { ioctls::mode_getconnector(self, &mut connector)? }

        unsafe {
            encoders.set_len(connector.count_encoders as usize);
            modes.set_len(connector.count_modes as usize);
            props.set_len(connector.count_props as usize);
            prop_values.set_len(connector.count_props as usize);
        }

        let drm_mode_get_connector {
            encoder_id,
            connector_type,
            connector_type_id,
            connection,
            connector_id,
            mm_width,
            mm_height,
            subpixel,
            pad,
            ..
        } = connector;

        Ok(Connector {
            encoder_id,
            connector_id: unsafe { ConnectorId::new_unchecked(connector_id) },
            connector_type,
            connector_type_id,
            connection: connection.into(),
            mm_width,
            mm_height,
            subpixel,
            pad,
            encoders,
            modes,
            props,
            prop_values,
        })
    }

    pub fn get_encoder(&self, encoder_id: EncoderId) -> io::Result<drm_mode_get_encoder> {
        let mut encoder: drm_mode_get_encoder = unsafe { mem::zeroed() };

        encoder.encoder_id = encoder_id.into();

        unsafe { ioctls::mode_getencoder(self, &mut encoder)? }

        Ok(encoder)
    }

    pub fn create_dumb_buffer(&self, height: u32, width: u32, bpp: u32) -> io::Result<DumbBuffer> {
        let mut create: drm_mode_create_dumb = unsafe { mem::zeroed() };

        create.height = height;
        create.width = width;
        create.bpp = bpp;

        unsafe { ioctls::mode_create_dumb(self, &mut create)? }

        Ok(DumbBuffer {
            fb_id: create.handle,
            height,
            width,
            stride: create.pitch,
            handle: create.handle,
            size: create.size,
        })
    }

    // FIXME: this prob needs some generics instead of DumbBuffer
    pub fn add_framebuffer(
        &self,
        framebuffer: &DumbBuffer,
        bpp: u32,
        depth: u32,
    ) -> io::Result<()> {
        let mut fb_cmd: drm_mode_fb_cmd = unsafe { mem::zeroed() };

        fb_cmd.fb_id = framebuffer.fb_id;
        fb_cmd.width = framebuffer.width;
        fb_cmd.height = framebuffer.height;
        fb_cmd.pitch = framebuffer.stride;
        fb_cmd.bpp = bpp;
        fb_cmd.depth = depth;
        fb_cmd.handle = framebuffer.handle;

        unsafe { ioctls::mode_addfb(self, &mut fb_cmd)? }

        Ok(())
    }

    pub fn map_dumb_buffer(&self, framebuffer: &DumbBuffer) -> io::Result<DumbBufferMapping> {
        let mut map: drm_mode_map_dumb = unsafe { mem::zeroed() };

        map.handle = framebuffer.fb_id;

        unsafe { ioctls::mode_map_dumb(self, &mut map)? }

        let map = unsafe {
            mmap(
                null_mut(),
                framebuffer.size as usize,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED,
                self,
                map.offset,
            )?
        };

        let map = unsafe { std::slice::from_raw_parts_mut(map.cast(), framebuffer.size as usize) };
        map.fill(0);

        Ok(DumbBufferMapping { inner: map })
    }

    pub fn get_crtc(&self, crtc_id: CrtcId) -> io::Result<drm_mode_crtc> {
        let mut crtc: drm_mode_crtc = unsafe { mem::zeroed() };

        crtc.crtc_id = crtc_id.into();

        unsafe { ioctls::mode_getcrtc(self, &mut crtc)? }

        Ok(crtc)
    }

    pub fn set_crtc(
        &self,
        crtc_id: CrtcId,
        fb_id: u32,
        x: u32,
        y: u32,
        connectors: &[ConnectorId],
        mode: Mode,
    ) -> io::Result<()> {
        let mut crtc: drm_mode_crtc = unsafe { mem::zeroed() };

        crtc.x = x;
        crtc.y = y;
        crtc.crtc_id = crtc_id.into();
        crtc.fb_id = fb_id;
        crtc.set_connectors_ptr = connectors.as_ptr() as _;
        crtc.count_connectors = connectors.len() as _;
        crtc.mode = mode.0;
        crtc.mode_valid = 1;

        unsafe { ioctls::mode_setcrtc(self, &mut crtc)? }

        Ok(())
    }

    pub fn get_plane_resources(&self) -> io::Result<Vec<u32>> {
        let mut res: drm_mode_get_plane_res = unsafe { mem::zeroed() };

        unsafe { ioctls::mode_getplaneresources(self, &mut res)? }

        let mut vec: Vec<u32> = create_and_reserve_buf(res.count_planes as usize);

        res.plane_id_ptr = vec.as_mut_ptr() as _;

        unsafe { ioctls::mode_getplaneresources(self, &mut res)? }

        unsafe { vec.set_len(res.count_planes as usize) }

        Ok(vec)
    }

    // FIXME: use sealed traits and wrapper types to make this safe
    pub unsafe fn get_properties(
        &self,
        object_id: u32,
        object_type: u32,
    ) -> io::Result<(Vec<u32>, Vec<u64>)> {
        let mut res: sys::drm_mode_obj_get_properties = unsafe { mem::zeroed() };

        res.obj_id = object_id;
        res.obj_type = object_type;

        unsafe { ioctls::mode_obj_getproperties(self, &mut res)? }

        let mut props: Vec<u32> = create_and_reserve_buf(res.count_props as usize);
        let mut values: Vec<u64> = create_and_reserve_buf(res.count_props as usize);

        res.props_ptr = props.as_mut_ptr() as _;
        res.prop_values_ptr = values.as_mut_ptr() as _;

        unsafe { ioctls::mode_obj_getproperties(self, &mut res)? }

        unsafe {
            props.set_len(res.count_props as usize);
            values.set_len(res.count_props as usize);
        }

        Ok((props, values))
    }

    pub unsafe fn get_property(&self, prop_id: u32) -> io::Result<(CString, Vec<u64>)> {
        let mut res: sys::drm_mode_get_property = unsafe { mem::zeroed() };

        res.prop_id = prop_id;

        unsafe { ioctls::mode_getproperty(self, &mut res)? }

        let mut values: Vec<u64> = create_and_reserve_buf(res.count_values as usize);
        // let mut blobs: Vec<u64> = create_and_reserve_buf(res.count_values as usize);

        res.values_ptr = values.as_mut_ptr() as _;
        // res.enum_blob_ptr = blobs.as_mut_ptr() as _;
        res.count_enum_blobs = 0; // FIXME: this needs special handling so just ignore it for now

        unsafe { ioctls::mode_getproperty(self, &mut res)? }

        unsafe {
            values.set_len(res.count_values as usize);
            // blobs.set_len(res.count_enum_blobs as usize);
        }

        let name = unsafe { CStr::from_ptr(res.name.as_ptr().cast()) };

        Ok((name.to_owned(), values))
    }

    pub fn get_plane(&self) -> io::Result<()> {
        let mut res: sys::drm_mode_get_plane = unsafe { mem::zeroed() };
        unsafe { ioctls::mode_getplane(self, &mut res)? };

        Ok(())
    }

    pub fn set_client_capability(&self, cap: ClientCapability, value: bool) -> io::Result<()> {
        let mut cap = sys::drm_set_client_cap {
            capability: cap as u64,
            value: value as u64,
        };

        unsafe { ioctls::set_client_cap(self, &mut cap)? };

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u32)]
pub enum ClientCapability {
    Atomic = sys::DRM_CLIENT_CAP_ATOMIC,
}

pub struct DumbBufferMapping<'a> {
    inner: &'a mut [u8],
}

impl Drop for DumbBufferMapping<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = munmap(self.inner.as_mut_ptr() as *mut _, self.inner.len());
        }
    }
}

impl AsRef<[u8]> for DumbBufferMapping<'_> {
    fn as_ref(&self) -> &[u8] {
        self.inner
    }
}

impl AsMut<[u8]> for DumbBufferMapping<'_> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner
    }
}

#[inline]
fn create_and_reserve_buf<T>(len: usize) -> Vec<T> {
    if len == 0 {
        return Vec::new();
    }

    let mut buf = Vec::with_capacity(len);
    buf.reserve_exact(len);

    buf
}

#[derive(Debug)]
pub struct Version {
    pub major: c_int,
    pub minor: c_int,
    pub patchlevel: c_int,
    pub name: String,
    pub date: String,
    pub desc: String,
}

#[derive(Debug)]
pub struct Resources {
    pub fbs: Vec<u32>,
    pub crtcs: Vec<CrtcId>,
    pub connectors: Vec<ConnectorId>,
    pub encoders: Vec<EncoderId>,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}

#[derive(Debug)]
pub struct Connector {
    pub encoder_id: u32,
    pub connector_id: ConnectorId,
    pub connector_type: u32,
    pub connector_type_id: u32,
    pub connection: ConnectorStatus,
    pub mm_width: u32,
    pub mm_height: u32,
    pub subpixel: u32,
    pub pad: u32,
    pub encoders: Vec<u32>,
    pub modes: Vec<Mode>,
    pub props: Vec<u32>,
    pub prop_values: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u32)]
pub enum ConnectorStatus {
    Connected = 1,
    Disconnected = 2,
    Unknown = 3,
}

impl From<u32> for ConnectorStatus {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Connected,
            2 => Self::Disconnected,
            _ => Self::Unknown,
        }
    }
}

impl ConnectorStatus {
    pub const fn is_connected(&self) -> bool {
        matches!(self, Self::Connected)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ConnectorId(NonZeroU32);

impl ConnectorId {
    pub const unsafe fn new_unchecked(id: u32) -> Self {
        unsafe { Self(NonZeroU32::new_unchecked(id)) }
    }
}

impl Display for ConnectorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<ConnectorId> for u32 {
    fn from(value: ConnectorId) -> Self {
        value.0.get()
    }
}

impl From<ConnectorId> for NonZeroU32 {
    fn from(value: ConnectorId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CrtcId(NonZeroU32);

impl CrtcId {
    pub const unsafe fn new_unchecked(id: u32) -> Self {
        unsafe { Self(NonZeroU32::new_unchecked(id)) }
    }
}

impl Display for CrtcId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<CrtcId> for u32 {
    fn from(value: CrtcId) -> Self {
        value.0.get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EncoderId(NonZeroU32);

impl Display for EncoderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<EncoderId> for u32 {
    fn from(value: EncoderId) -> Self {
        value.0.get()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Mode(drm_mode_modeinfo);

bitflags::bitflags! {
    /// Display mode type
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ModeType: u32 {
        const BUILTIN = sys::DRM_MODE_TYPE_BUILTIN;
        const CLOCK_C = sys::DRM_MODE_TYPE_CLOCK_C;
        const CRTC_C = sys::DRM_MODE_TYPE_CRTC_C;
        const PREFERRED = sys::DRM_MODE_TYPE_PREFERRED;
        const DEFAULT = sys::DRM_MODE_TYPE_DEFAULT;
        const USERDEF = sys::DRM_MODE_TYPE_USERDEF;
        const DRIVER = sys::DRM_MODE_TYPE_DRIVER;
        const ALL = sys::DRM_MODE_TYPE_ALL;
    }
}

impl Mode {
    pub const fn name(&self) -> &CStr {
        unsafe {
            CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(
                self.0.name.as_ptr().cast(),
                self.0.name.len(),
            ))
        }
    }

    pub const fn display_width(&self) -> u16 {
        self.0.hdisplay
    }

    pub const fn display_height(&self) -> u16 {
        self.0.vdisplay
    }

    pub const fn vertical_refresh_rate(&self) -> u32 {
        self.0.vrefresh
    }

    pub fn wsi_refresh_rate(&self) -> u32 {
        ((self.0.clock as f64 * 1000.0
            / (self.0.htotal as f64 * self.0.vtotal as f64 * self.0.vscan.max(1) as f64))
            * 1000.0
            + 0.5) as u32
    }

    pub fn ty(&self) -> ModeType {
        ModeType::from_bits_truncate(self.0.type_)
    }
}

#[derive(Debug)]
pub struct DumbBuffer {
    pub fb_id: u32,
    pub height: u32,
    pub width: u32,
    pub stride: u32,
    pub handle: u32,
    pub size: u64,
}
