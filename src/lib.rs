use std::{
    ffi::{c_int, CString},
    mem,
};

use linux_raw_sys::drm::{drm_mode_card_res, drm_version, DRM_IOCTL_BASE};
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
        let mut connectors: Vec<u32> = create_and_reserve_buf(resources.count_connectors as usize);
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
    pub connectors: Vec<u32>,
    pub encoders: Vec<u32>,
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
}
