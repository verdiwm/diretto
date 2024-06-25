use raw_window_handle::{DisplayHandle, DrmDisplayHandle};
use rustix::fd::{AsRawFd, OwnedFd};

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
}
