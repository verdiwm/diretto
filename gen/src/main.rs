use std::fs;

use anyhow::Result;
use tempfile::TempDir;
use xshell::{cmd, Shell};

const VERSION: &str = "v6.13";

fn main() -> Result<()> {
    let sh = Shell::new()?;

    if !fs::exists("linux/.git")? {
        cmd!(
            sh,
            "git clone --filter=tree:0 --no-checkout https://github.com/torvalds/linux.git"
        )
        .run()?;
    }

    sh.change_dir("linux");

    cmd!(sh, "git fetch origin tag {VERSION}").run()?;
    cmd!(sh, "git checkout {VERSION}").run()?;

    let temp_dir = TempDir::with_prefix("diretto-linux-bindings-")?;
    let work_dir = temp_dir.path();

    cmd!(sh, "make headers_install INSTALL_HDR_PATH={work_dir}").run()?;

    let bindings = bindgen::builder()
        .clang_arg(format!("-I{}", work_dir.join("include").display()))
        .header("src/bindings.h")
        .use_core()
        .generate()?;

    bindings.write_to_file("../src/sys.rs")?;

    Ok(())
}
