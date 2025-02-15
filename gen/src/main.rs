use anyhow::Result;

mod drm;

fn main() -> Result<()> {
    drm::generate_drm_bindings()?;

    Ok(())
}
