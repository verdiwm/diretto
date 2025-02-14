use crate::sys::DRM_IOCTL_BASE;
use rustix::ioctl::{ioctl, NoArg, NoneOpcode, ReadOpcode, ReadWriteOpcode, Updater, WriteOpcode};
use std::os::fd::AsFd;
pub unsafe fn version<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_version,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 0u8, crate::sys::drm_version>,
                crate::sys::drm_version,
            >::new(data),
        )
    }
}
pub unsafe fn get_unique<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_unique,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 1u8, crate::sys::drm_unique>,
                crate::sys::drm_unique,
            >::new(data),
        )
    }
}
pub unsafe fn get_magic<F: AsFd>(fd: F, data: &mut crate::sys::drm_auth) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < ReadOpcode < DRM_IOCTL_BASE , 2u8 , crate :: sys :: drm_auth > , crate :: sys :: drm_auth > :: new (data))
    }
}
pub unsafe fn irq_busid<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_irq_busid,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 3u8, crate::sys::drm_irq_busid>,
                crate::sys::drm_irq_busid,
            >::new(data),
        )
    }
}
pub unsafe fn get_map<F: AsFd>(fd: F, data: &mut crate::sys::drm_map) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < ReadWriteOpcode < DRM_IOCTL_BASE , 4u8 , crate :: sys :: drm_map > , crate :: sys :: drm_map > :: new (data))
    }
}
pub unsafe fn get_client<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_client,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 5u8, crate::sys::drm_client>,
                crate::sys::drm_client,
            >::new(data),
        )
    }
}
pub unsafe fn get_stats<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_stats,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < ReadOpcode < DRM_IOCTL_BASE , 6u8 , crate :: sys :: drm_stats > , crate :: sys :: drm_stats > :: new (data))
    }
}
pub unsafe fn set_version<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_set_version,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 7u8, crate::sys::drm_set_version>,
                crate::sys::drm_set_version,
            >::new(data),
        )
    }
}
pub unsafe fn modeset_ctl<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_modeset_ctl,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 8u8, crate::sys::drm_modeset_ctl>,
                crate::sys::drm_modeset_ctl,
            >::new(data),
        )
    }
}
pub unsafe fn gem_close<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_gem_close,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 9u8, crate::sys::drm_gem_close>,
                crate::sys::drm_gem_close,
            >::new(data),
        )
    }
}
pub unsafe fn gem_flink<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_gem_flink,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 10u8, crate::sys::drm_gem_flink>,
                crate::sys::drm_gem_flink,
            >::new(data),
        )
    }
}
pub unsafe fn gem_open<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_gem_open,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 11u8, crate::sys::drm_gem_open>,
                crate::sys::drm_gem_open,
            >::new(data),
        )
    }
}
pub unsafe fn get_cap<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_get_cap,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 12u8, crate::sys::drm_get_cap>,
                crate::sys::drm_get_cap,
            >::new(data),
        )
    }
}
pub unsafe fn set_client_cap<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_set_client_cap,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 13u8, crate::sys::drm_set_client_cap>,
                crate::sys::drm_set_client_cap,
            >::new(data),
        )
    }
}
pub unsafe fn set_unique<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_unique,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 16u8, crate::sys::drm_unique>,
                crate::sys::drm_unique,
            >::new(data),
        )
    }
}
pub unsafe fn auth_magic<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_auth,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 17u8 , crate :: sys :: drm_auth > , crate :: sys :: drm_auth > :: new (data))
    }
}
pub unsafe fn block<F: AsFd>(fd: F, data: &mut crate::sys::drm_block) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 18u8, crate::sys::drm_block>,
                crate::sys::drm_block,
            >::new(data),
        )
    }
}
pub unsafe fn unblock<F: AsFd>(fd: F, data: &mut crate::sys::drm_block) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 19u8, crate::sys::drm_block>,
                crate::sys::drm_block,
            >::new(data),
        )
    }
}
pub unsafe fn control<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_control,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 20u8, crate::sys::drm_control>,
                crate::sys::drm_control,
            >::new(data),
        )
    }
}
pub unsafe fn add_map<F: AsFd>(fd: F, data: &mut crate::sys::drm_map) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 21u8, crate::sys::drm_map>,
                crate::sys::drm_map,
            >::new(data),
        )
    }
}
pub unsafe fn add_bufs<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_buf_desc,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 22u8, crate::sys::drm_buf_desc>,
                crate::sys::drm_buf_desc,
            >::new(data),
        )
    }
}
pub unsafe fn mark_bufs<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_buf_desc,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 23u8, crate::sys::drm_buf_desc>,
                crate::sys::drm_buf_desc,
            >::new(data),
        )
    }
}
pub unsafe fn info_bufs<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_buf_info,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 24u8, crate::sys::drm_buf_info>,
                crate::sys::drm_buf_info,
            >::new(data),
        )
    }
}
pub unsafe fn map_bufs<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_buf_map,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 25u8, crate::sys::drm_buf_map>,
                crate::sys::drm_buf_map,
            >::new(data),
        )
    }
}
pub unsafe fn free_bufs<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_buf_free,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 26u8, crate::sys::drm_buf_free>,
                crate::sys::drm_buf_free,
            >::new(data),
        )
    }
}
pub unsafe fn rm_map<F: AsFd>(fd: F, data: &mut crate::sys::drm_map) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 27u8 , crate :: sys :: drm_map > , crate :: sys :: drm_map > :: new (data))
    }
}
pub unsafe fn set_sarea_ctx<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_ctx_priv_map,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 28u8, crate::sys::drm_ctx_priv_map>,
                crate::sys::drm_ctx_priv_map,
            >::new(data),
        )
    }
}
pub unsafe fn get_sarea_ctx<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_ctx_priv_map,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 29u8, crate::sys::drm_ctx_priv_map>,
                crate::sys::drm_ctx_priv_map,
            >::new(data),
        )
    }
}
pub unsafe fn set_master<F: AsFd>(fd: F) -> rustix::io::Result<()> {
    unsafe { ioctl(fd, NoArg::<NoneOpcode<DRM_IOCTL_BASE, 30u8, ()>>::new()) }
}
pub unsafe fn drop_master<F: AsFd>(fd: F) -> rustix::io::Result<()> {
    unsafe { ioctl(fd, NoArg::<NoneOpcode<DRM_IOCTL_BASE, 31u8, ()>>::new()) }
}
pub unsafe fn add_ctx<F: AsFd>(fd: F, data: &mut crate::sys::drm_ctx) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 32u8, crate::sys::drm_ctx>,
                crate::sys::drm_ctx,
            >::new(data),
        )
    }
}
pub unsafe fn rm_ctx<F: AsFd>(fd: F, data: &mut crate::sys::drm_ctx) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 33u8, crate::sys::drm_ctx>,
                crate::sys::drm_ctx,
            >::new(data),
        )
    }
}
pub unsafe fn mod_ctx<F: AsFd>(fd: F, data: &mut crate::sys::drm_ctx) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 34u8 , crate :: sys :: drm_ctx > , crate :: sys :: drm_ctx > :: new (data))
    }
}
pub unsafe fn get_ctx<F: AsFd>(fd: F, data: &mut crate::sys::drm_ctx) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 35u8, crate::sys::drm_ctx>,
                crate::sys::drm_ctx,
            >::new(data),
        )
    }
}
pub unsafe fn switch_ctx<F: AsFd>(fd: F, data: &mut crate::sys::drm_ctx) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 36u8 , crate :: sys :: drm_ctx > , crate :: sys :: drm_ctx > :: new (data))
    }
}
pub unsafe fn new_ctx<F: AsFd>(fd: F, data: &mut crate::sys::drm_ctx) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 37u8 , crate :: sys :: drm_ctx > , crate :: sys :: drm_ctx > :: new (data))
    }
}
pub unsafe fn res_ctx<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_ctx_res,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 38u8, crate::sys::drm_ctx_res>,
                crate::sys::drm_ctx_res,
            >::new(data),
        )
    }
}
pub unsafe fn add_draw<F: AsFd>(fd: F, data: &mut crate::sys::drm_draw) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 39u8, crate::sys::drm_draw>,
                crate::sys::drm_draw,
            >::new(data),
        )
    }
}
pub unsafe fn rm_draw<F: AsFd>(fd: F, data: &mut crate::sys::drm_draw) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 40u8, crate::sys::drm_draw>,
                crate::sys::drm_draw,
            >::new(data),
        )
    }
}
pub unsafe fn dma<F: AsFd>(fd: F, data: &mut crate::sys::drm_dma) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 41u8, crate::sys::drm_dma>,
                crate::sys::drm_dma,
            >::new(data),
        )
    }
}
pub unsafe fn lock<F: AsFd>(fd: F, data: &mut crate::sys::drm_lock) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 42u8 , crate :: sys :: drm_lock > , crate :: sys :: drm_lock > :: new (data))
    }
}
pub unsafe fn unlock<F: AsFd>(fd: F, data: &mut crate::sys::drm_lock) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 43u8 , crate :: sys :: drm_lock > , crate :: sys :: drm_lock > :: new (data))
    }
}
pub unsafe fn finish<F: AsFd>(fd: F, data: &mut crate::sys::drm_lock) -> rustix::io::Result<()> {
    unsafe {
        ioctl (fd , Updater :: < WriteOpcode < DRM_IOCTL_BASE , 44u8 , crate :: sys :: drm_lock > , crate :: sys :: drm_lock > :: new (data))
    }
}
pub unsafe fn prime_handle_to_fd<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_prime_handle,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 45u8, crate::sys::drm_prime_handle>,
                crate::sys::drm_prime_handle,
            >::new(data),
        )
    }
}
pub unsafe fn prime_fd_to_handle<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_prime_handle,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 46u8, crate::sys::drm_prime_handle>,
                crate::sys::drm_prime_handle,
            >::new(data),
        )
    }
}
pub unsafe fn agp_acquire<F: AsFd>(fd: F) -> rustix::io::Result<()> {
    unsafe { ioctl(fd, NoArg::<NoneOpcode<DRM_IOCTL_BASE, 48u8, ()>>::new()) }
}
pub unsafe fn agp_release<F: AsFd>(fd: F) -> rustix::io::Result<()> {
    unsafe { ioctl(fd, NoArg::<NoneOpcode<DRM_IOCTL_BASE, 49u8, ()>>::new()) }
}
pub unsafe fn agp_enable<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_agp_mode,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 50u8, crate::sys::drm_agp_mode>,
                crate::sys::drm_agp_mode,
            >::new(data),
        )
    }
}
pub unsafe fn agp_info<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_agp_info,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadOpcode<DRM_IOCTL_BASE, 51u8, crate::sys::drm_agp_info>,
                crate::sys::drm_agp_info,
            >::new(data),
        )
    }
}
pub unsafe fn agp_alloc<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_agp_buffer,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 52u8, crate::sys::drm_agp_buffer>,
                crate::sys::drm_agp_buffer,
            >::new(data),
        )
    }
}
pub unsafe fn agp_free<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_agp_buffer,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 53u8, crate::sys::drm_agp_buffer>,
                crate::sys::drm_agp_buffer,
            >::new(data),
        )
    }
}
pub unsafe fn agp_bind<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_agp_binding,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 54u8, crate::sys::drm_agp_binding>,
                crate::sys::drm_agp_binding,
            >::new(data),
        )
    }
}
pub unsafe fn agp_unbind<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_agp_binding,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 55u8, crate::sys::drm_agp_binding>,
                crate::sys::drm_agp_binding,
            >::new(data),
        )
    }
}
pub unsafe fn sg_alloc<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_scatter_gather,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 56u8, crate::sys::drm_scatter_gather>,
                crate::sys::drm_scatter_gather,
            >::new(data),
        )
    }
}
pub unsafe fn sg_free<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_scatter_gather,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 57u8, crate::sys::drm_scatter_gather>,
                crate::sys::drm_scatter_gather,
            >::new(data),
        )
    }
}
pub unsafe fn wait_vblank<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_wait_vblank,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 58u8, crate::sys::drm_wait_vblank>,
                crate::sys::drm_wait_vblank,
            >::new(data),
        )
    }
}
pub unsafe fn crtc_get_sequence<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_crtc_get_sequence,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 59u8, crate::sys::drm_crtc_get_sequence>,
                crate::sys::drm_crtc_get_sequence,
            >::new(data),
        )
    }
}
pub unsafe fn crtc_queue_sequence<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_crtc_queue_sequence,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 60u8, crate::sys::drm_crtc_queue_sequence>,
                crate::sys::drm_crtc_queue_sequence,
            >::new(data),
        )
    }
}
pub unsafe fn update_draw<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_update_draw,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                WriteOpcode<DRM_IOCTL_BASE, 63u8, crate::sys::drm_update_draw>,
                crate::sys::drm_update_draw,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getresources<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_card_res,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 160u8, crate::sys::drm_mode_card_res>,
                crate::sys::drm_mode_card_res,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getcrtc<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_crtc,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 161u8, crate::sys::drm_mode_crtc>,
                crate::sys::drm_mode_crtc,
            >::new(data),
        )
    }
}
pub unsafe fn mode_setcrtc<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_crtc,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 162u8, crate::sys::drm_mode_crtc>,
                crate::sys::drm_mode_crtc,
            >::new(data),
        )
    }
}
pub unsafe fn mode_cursor<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_cursor,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 163u8, crate::sys::drm_mode_cursor>,
                crate::sys::drm_mode_cursor,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getgamma<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_crtc_lut,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 164u8, crate::sys::drm_mode_crtc_lut>,
                crate::sys::drm_mode_crtc_lut,
            >::new(data),
        )
    }
}
pub unsafe fn mode_setgamma<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_crtc_lut,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 165u8, crate::sys::drm_mode_crtc_lut>,
                crate::sys::drm_mode_crtc_lut,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getencoder<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_encoder,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 166u8, crate::sys::drm_mode_get_encoder>,
                crate::sys::drm_mode_get_encoder,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getconnector<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_connector,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 167u8, crate::sys::drm_mode_get_connector>,
                crate::sys::drm_mode_get_connector,
            >::new(data),
        )
    }
}
pub unsafe fn mode_attachmode<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_mode_cmd,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 168u8, crate::sys::drm_mode_mode_cmd>,
                crate::sys::drm_mode_mode_cmd,
            >::new(data),
        )
    }
}
pub unsafe fn mode_detachmode<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_mode_cmd,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 169u8, crate::sys::drm_mode_mode_cmd>,
                crate::sys::drm_mode_mode_cmd,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getproperty<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_property,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 170u8, crate::sys::drm_mode_get_property>,
                crate::sys::drm_mode_get_property,
            >::new(data),
        )
    }
}
pub unsafe fn mode_setproperty<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_connector_set_property,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 171u8, crate::sys::drm_mode_connector_set_property>,
                crate::sys::drm_mode_connector_set_property,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getpropblob<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_blob,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 172u8, crate::sys::drm_mode_get_blob>,
                crate::sys::drm_mode_get_blob,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getfb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_fb_cmd,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 173u8, crate::sys::drm_mode_fb_cmd>,
                crate::sys::drm_mode_fb_cmd,
            >::new(data),
        )
    }
}
pub unsafe fn mode_addfb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_fb_cmd,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 174u8, crate::sys::drm_mode_fb_cmd>,
                crate::sys::drm_mode_fb_cmd,
            >::new(data),
        )
    }
}
pub unsafe fn mode_rmfb<F: AsFd>(fd: F, data: &mut u32) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<ReadWriteOpcode<DRM_IOCTL_BASE, 175u8, u32>, u32>::new(data),
        )
    }
}
pub unsafe fn mode_page_flip<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_crtc_page_flip,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 176u8, crate::sys::drm_mode_crtc_page_flip>,
                crate::sys::drm_mode_crtc_page_flip,
            >::new(data),
        )
    }
}
pub unsafe fn mode_dirtyfb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_fb_dirty_cmd,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 177u8, crate::sys::drm_mode_fb_dirty_cmd>,
                crate::sys::drm_mode_fb_dirty_cmd,
            >::new(data),
        )
    }
}
pub unsafe fn mode_create_dumb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_create_dumb,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 178u8, crate::sys::drm_mode_create_dumb>,
                crate::sys::drm_mode_create_dumb,
            >::new(data),
        )
    }
}
pub unsafe fn mode_map_dumb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_map_dumb,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 179u8, crate::sys::drm_mode_map_dumb>,
                crate::sys::drm_mode_map_dumb,
            >::new(data),
        )
    }
}
pub unsafe fn mode_destroy_dumb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_destroy_dumb,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 180u8, crate::sys::drm_mode_destroy_dumb>,
                crate::sys::drm_mode_destroy_dumb,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getplaneresources<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_plane_res,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 181u8, crate::sys::drm_mode_get_plane_res>,
                crate::sys::drm_mode_get_plane_res,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getplane<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_plane,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 182u8, crate::sys::drm_mode_get_plane>,
                crate::sys::drm_mode_get_plane,
            >::new(data),
        )
    }
}
pub unsafe fn mode_setplane<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_set_plane,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 183u8, crate::sys::drm_mode_set_plane>,
                crate::sys::drm_mode_set_plane,
            >::new(data),
        )
    }
}
pub unsafe fn mode_addfb2<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_fb_cmd2,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 184u8, crate::sys::drm_mode_fb_cmd2>,
                crate::sys::drm_mode_fb_cmd2,
            >::new(data),
        )
    }
}
pub unsafe fn mode_obj_getproperties<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_obj_get_properties,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 185u8, crate::sys::drm_mode_obj_get_properties>,
                crate::sys::drm_mode_obj_get_properties,
            >::new(data),
        )
    }
}
pub unsafe fn mode_obj_setproperty<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_obj_set_property,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 186u8, crate::sys::drm_mode_obj_set_property>,
                crate::sys::drm_mode_obj_set_property,
            >::new(data),
        )
    }
}
pub unsafe fn mode_cursor2<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_cursor2,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 187u8, crate::sys::drm_mode_cursor2>,
                crate::sys::drm_mode_cursor2,
            >::new(data),
        )
    }
}
pub unsafe fn mode_atomic<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_atomic,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 188u8, crate::sys::drm_mode_atomic>,
                crate::sys::drm_mode_atomic,
            >::new(data),
        )
    }
}
pub unsafe fn mode_createpropblob<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_create_blob,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 189u8, crate::sys::drm_mode_create_blob>,
                crate::sys::drm_mode_create_blob,
            >::new(data),
        )
    }
}
pub unsafe fn mode_destroypropblob<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_destroy_blob,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 190u8, crate::sys::drm_mode_destroy_blob>,
                crate::sys::drm_mode_destroy_blob,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_create<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_create,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 191u8, crate::sys::drm_syncobj_create>,
                crate::sys::drm_syncobj_create,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_destroy<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_destroy,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 192u8, crate::sys::drm_syncobj_destroy>,
                crate::sys::drm_syncobj_destroy,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_handle_to_fd<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_handle,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 193u8, crate::sys::drm_syncobj_handle>,
                crate::sys::drm_syncobj_handle,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_fd_to_handle<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_handle,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 194u8, crate::sys::drm_syncobj_handle>,
                crate::sys::drm_syncobj_handle,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_wait<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_wait,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 195u8, crate::sys::drm_syncobj_wait>,
                crate::sys::drm_syncobj_wait,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_reset<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_array,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 196u8, crate::sys::drm_syncobj_array>,
                crate::sys::drm_syncobj_array,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_signal<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_array,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 197u8, crate::sys::drm_syncobj_array>,
                crate::sys::drm_syncobj_array,
            >::new(data),
        )
    }
}
pub unsafe fn mode_create_lease<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_create_lease,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 198u8, crate::sys::drm_mode_create_lease>,
                crate::sys::drm_mode_create_lease,
            >::new(data),
        )
    }
}
pub unsafe fn mode_list_lessees<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_list_lessees,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 199u8, crate::sys::drm_mode_list_lessees>,
                crate::sys::drm_mode_list_lessees,
            >::new(data),
        )
    }
}
pub unsafe fn mode_get_lease<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_get_lease,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 200u8, crate::sys::drm_mode_get_lease>,
                crate::sys::drm_mode_get_lease,
            >::new(data),
        )
    }
}
pub unsafe fn mode_revoke_lease<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_revoke_lease,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 201u8, crate::sys::drm_mode_revoke_lease>,
                crate::sys::drm_mode_revoke_lease,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_timeline_wait<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_timeline_wait,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 202u8, crate::sys::drm_syncobj_timeline_wait>,
                crate::sys::drm_syncobj_timeline_wait,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_query<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_timeline_array,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 203u8, crate::sys::drm_syncobj_timeline_array>,
                crate::sys::drm_syncobj_timeline_array,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_transfer<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_transfer,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 204u8, crate::sys::drm_syncobj_transfer>,
                crate::sys::drm_syncobj_transfer,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_timeline_signal<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_timeline_array,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 205u8, crate::sys::drm_syncobj_timeline_array>,
                crate::sys::drm_syncobj_timeline_array,
            >::new(data),
        )
    }
}
pub unsafe fn mode_getfb2<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_fb_cmd2,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 206u8, crate::sys::drm_mode_fb_cmd2>,
                crate::sys::drm_mode_fb_cmd2,
            >::new(data),
        )
    }
}
pub unsafe fn syncobj_eventfd<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_syncobj_eventfd,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 207u8, crate::sys::drm_syncobj_eventfd>,
                crate::sys::drm_syncobj_eventfd,
            >::new(data),
        )
    }
}
pub unsafe fn mode_closefb<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_mode_closefb,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 208u8, crate::sys::drm_mode_closefb>,
                crate::sys::drm_mode_closefb,
            >::new(data),
        )
    }
}
pub unsafe fn set_client_name<F: AsFd>(
    fd: F,
    data: &mut crate::sys::drm_set_client_name,
) -> rustix::io::Result<()> {
    unsafe {
        ioctl(
            fd,
            Updater::<
                ReadWriteOpcode<DRM_IOCTL_BASE, 209u8, crate::sys::drm_set_client_name>,
                crate::sys::drm_set_client_name,
            >::new(data),
        )
    }
}
