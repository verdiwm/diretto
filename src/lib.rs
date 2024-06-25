use std::{
    ffi::{c_int, CString},
    fmt::Display,
    mem,
    num::NonZeroU32,
};

use linux_raw_sys::drm::{
    drm_mode_card_res, drm_mode_get_connector, drm_mode_modeinfo, drm_version, DRM_IOCTL_BASE,
};
use raw_window_handle::{DisplayHandle, DrmDisplayHandle};
use rustix::{
    fd::{AsRawFd, OwnedFd},
    io,
    ioctl::{ioctl, ReadWriteOpcode, Updater},
};

#[derive(Debug)]
pub struct Device {
    fd: OwnedFd,
}

// FIXME: take into account hotplug
impl Device {
    pub unsafe fn new(fd: OwnedFd) -> Self {
        Self { fd }
    }

    pub fn display_handle(&self) -> DisplayHandle {
        unsafe { DisplayHandle::borrow_raw(DrmDisplayHandle::new(self.fd.as_raw_fd()).into()) }
    }

    #[inline]
    fn ioctl_rw<const NUM: u8, T>(&self, data: &mut T) -> io::Result<()> {
        unsafe {
            ioctl(
                &self.fd,
                Updater::<ReadWriteOpcode<DRM_IOCTL_BASE, NUM, T>, T>::new(data),
            )
        }
    }

    pub fn version(&self) -> io::Result<Version> {
        #[inline]
        unsafe fn ret_vector_to_cstring(mut vec: Vec<u8>, len: usize) -> CString {
            vec.set_len(len);

            if let Some(last) = vec.last() {
                if *last != b'\0' {
                    vec.push(b'\0')
                }
            }

            CString::from_vec_with_nul_unchecked(vec)
        }

        let mut version: Box<drm_version> = Box::new(unsafe { mem::zeroed() });

        self.ioctl_rw::<0x00, drm_version>(&mut version)?;

        let mut name_buf: Vec<u8> = create_and_reserve_buf(version.name_len as usize + 1);
        let mut date_buf: Vec<u8> = create_and_reserve_buf(version.date_len as usize + 1);
        let mut desc_buf: Vec<u8> = create_and_reserve_buf(version.desc_len as usize + 1);

        version.name = name_buf.as_mut_ptr().cast();
        version.date = date_buf.as_mut_ptr().cast();
        version.desc = desc_buf.as_mut_ptr().cast();

        self.ioctl_rw::<0x00, drm_version>(&mut version)?;

        unsafe {
            let name = ret_vector_to_cstring(name_buf, version.name_len as usize);
            let date = ret_vector_to_cstring(date_buf, version.date_len as usize);
            let desc = ret_vector_to_cstring(desc_buf, version.desc_len as usize);

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

    pub fn get_resources(&self) -> io::Result<Resources> {
        let mut resources: drm_mode_card_res = unsafe { mem::zeroed() };

        self.ioctl_rw::<0xA0, drm_mode_card_res>(&mut resources)?;

        let mut fbs: Vec<u32> = create_and_reserve_buf(resources.count_fbs as usize);
        let mut crtcs: Vec<u32> = create_and_reserve_buf(resources.count_crtcs as usize);
        let mut connectors: Vec<ConnectorId> =
            create_and_reserve_buf(resources.count_connectors as usize);
        let mut encoders: Vec<u32> = create_and_reserve_buf(resources.count_encoders as usize);

        resources.fb_id_ptr = fbs.as_mut_ptr() as _;
        resources.crtc_id_ptr = crtcs.as_mut_ptr() as _;
        resources.connector_id_ptr = connectors.as_mut_ptr() as _;
        resources.encoder_id_ptr = encoders.as_mut_ptr() as _;

        self.ioctl_rw::<0xA0, drm_mode_card_res>(&mut resources)?;

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

        self.ioctl_rw::<0xA7, drm_mode_get_connector>(&mut connector)?;

        let mut encoders: Vec<u32> = create_and_reserve_buf(connector.count_encoders as usize);
        let mut modes: Vec<drm_mode_modeinfo> =
            create_and_reserve_buf(connector.count_encoders as usize); // FIXME: handle special case where modes is empty
        let mut props: Vec<u32> = create_and_reserve_buf(connector.count_props as usize);
        let mut prop_values: Vec<u64> = create_and_reserve_buf(connector.count_props as usize);

        connector.encoders_ptr = encoders.as_mut_ptr() as _;
        connector.modes_ptr = modes.as_mut_ptr() as _;
        connector.props_ptr = props.as_mut_ptr() as _;
        connector.prop_values_ptr = prop_values.as_mut_ptr() as _;

        self.ioctl_rw::<0xA7, drm_mode_get_connector>(&mut connector)?;

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
            mm_width,
            mm_height,
            subpixel,
            pad,
            ..
        } = connector;

        Ok(Connector {
            encoder_id,
            connector_id,
            connector_type,
            connector_type_id,
            connection,
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
    pub name: CString,
    pub date: CString,
    pub desc: CString,
}

#[derive(Debug)]
pub struct Resources {
    pub fbs: Vec<u32>,
    pub crtcs: Vec<u32>,
    pub connectors: Vec<ConnectorId>,
    pub encoders: Vec<u32>,
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
    pub connection: u32,
    pub mm_width: u32,
    pub mm_height: u32,
    pub subpixel: u32,
    pub pad: u32,
    pub encoders: Vec<u32>,
    pub modes: Vec<drm_mode_modeinfo>,
    pub props: Vec<u32>,
    pub prop_values: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ConnectorId(NonZeroU32);

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
