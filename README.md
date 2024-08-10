# Diretto

Diretto is a lightweight yet powerful Rust library designed to facilitate
interaction with the Linux kernel's Direct Rendering Manager (DRM) interface.

## Overview

Diretto offers robust abstractions over both the legacy DRM API and the modern
atomic modesetting API. It is purpose-built for Linux environments, bypassing
the need for the standard C library (libc) and directly interfacing with the
kernel through the Rust-based `rustix` crate.

One of the unique features of Diretto is its approach to device management.
Unlike other libraries, Diretto assumes full ownership of the DRM device,. This
design not only simplifies the API but also enhances safety by providing a
reliable mechanism to verify that the device being opened supports the necessary
DRM ioctls.

## Usage

The core of the Diretto library is the `Device` struct, which serves as the
primary entry point for interacting with DRM devices. Although helper APIs are
still under development, you can already use Diretto with a more manual
approach. The example below demonstrates how to open a file descriptor using
`rustix` and create a new `Device` instance:

```rust
use rustix::fs::{self, OFlags, Mode};
use diretto::Device;

let fd = fs::open(
     "/dev/dri/card0",
     OFlags::RDWR | OFlags::NONBLOCK,
     Mode::empty(),
 )?;
let device = unsafe { Device::new_unchecked(fd) };
```

For a more advanced usage example, including rendering to the screen with
`wgpu`, refer to the [example code](examples/wgpu.rs).

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
