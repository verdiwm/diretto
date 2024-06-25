use std::{
    ffi::{c_int, CString},
    ptr::null_mut,
};

use linux_raw_sys::drm::{drm_version, DRM_IOCTL_BASE};
use raw_window_handle::{DisplayHandle, DrmDisplayHandle};
use rustix::{
    fd::{AsFd, AsRawFd, OwnedFd},
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

    pub fn version(&self) -> io::Result<Version> {
        #[inline]
        fn call_version_ioctl<F: AsFd>(fd: F, version: &mut drm_version) -> io::Result<()> {
            unsafe {
                ioctl(
                    fd,
                    Updater::<ReadWriteOpcode<DRM_IOCTL_BASE, 0x00, drm_version>, drm_version>::new(
                        version,
                    ),
                )
            }
        }

        #[inline]
        fn create_and_reserve_buf(len: usize) -> Vec<u8> {
            let mut buf = Vec::with_capacity(len);
            buf.reserve_exact(len);

            buf
        }

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

        let mut version = Box::new(drm_version {
            version_major: 0,
            version_minor: 0,
            version_patchlevel: 0,
            name_len: 0,
            name: null_mut(),
            date_len: 0,
            date: null_mut(),
            desc_len: 0,
            desc: null_mut(),
        });

        call_version_ioctl(&self.fd, &mut version)?;

        let mut name_buf = create_and_reserve_buf(version.name_len as usize + 1);
        let mut date_buf = create_and_reserve_buf(version.date_len as usize + 1);
        let mut desc_buf = create_and_reserve_buf(version.desc_len as usize + 1);

        version.name = name_buf.as_mut_ptr().cast();
        version.date = date_buf.as_mut_ptr().cast();
        version.desc = desc_buf.as_mut_ptr().cast();

        call_version_ioctl(&self.fd, &mut version)?;

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
