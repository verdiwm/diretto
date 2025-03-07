use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write as _,
};

use anyhow::Result;
use heck::ToSnakeCase;
use quote::{format_ident, quote};
use regex::Regex;
use tempfile::TempDir;
use xshell::{Shell, cmd};

const LINUX_VERSION: &str = "v6.13";

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

    cmd!(sh, "git fetch origin tag {LINUX_VERSION}").run()?;
    cmd!(sh, "git checkout {LINUX_VERSION}").run()?;

    let temp_dir = TempDir::with_prefix("diretto-linux-bindings-")?;
    let work_dir = temp_dir.path();

    cmd!(sh, "make headers_install INSTALL_HDR_PATH={work_dir}").run()?;

    let bindings = bindgen::builder()
        .clang_arg(format!("-I{}", work_dir.join("include").display()))
        .header("src/drm.h")
        .use_core()
        .generate()?;

    bindings.write_to_file("../src/sys.rs")?;

    let header = fs::read_to_string(work_dir.join("include/drm/drm.h"))?;

    let re = Regex::new(r"#define\s+DRM_IOCTL_([A-Z0-9_]+)\s+DRM_([A-Z0-9_]+)\s*\((.*?)\)")?;

    let mut ioctls = Vec::new();

    for capture in re.captures_iter(&header) {
        let name = capture.get(1).unwrap().as_str().trim();
        let action = capture.get(2).unwrap().as_str().trim();
        let content = capture.get(3).unwrap().as_str().trim();

        let action = match action {
            "IO" => Action::IO,
            "IOW" => Action::IOW,
            "IOR" => Action::IOR,
            "IOWR" => Action::IOWR,
            _ => panic!("Unknown action: \"{action}\""),
        };

        if let Some((code, model)) = content.split_once(',') {
            println!("Found {name} with action {action}, code {code} returning {model}");

            let model =
                find_type(model.trim()).unwrap_or_else(|| panic!("Invalid model: \"{model}\""));

            ioctls.push(IoCtl {
                name: name.to_string(),
                action,
                code: code.to_string(),
                ret: Some(model),
            });
        } else {
            println!("Found {name} with action {action} and code {content}");

            ioctls.push(IoCtl {
                name: name.to_string(),
                action,
                code: content.to_string(),
                ret: None,
            });
        }
    }

    println!("Found a total of {} ioctls", ioctls.len());

    let mut functions = Vec::new();

    for ioctl in ioctls {
        let IoCtl {
            name,
            action,
            code,
            ret,
        } = ioctl;

        let name = format_ident!("{}", name.to_snake_case());
        let code = u8::from_str_radix(code.trim_start_matches("0x"), 16)?;

        let opcode = match action {
            Action::IO => quote! {opcode::none},
            Action::IOW => quote! {opcode::write},
            Action::IOR => quote! {opcode::read},
            Action::IOWR => quote! {opcode::read_write},
        };

        if let Some(ret) = ret {
            let ret = match ret {
                Model::Struct(m) | Model::Union(m) => {
                    let ident = format_ident!("{m}");
                    quote! { super::sys::#ident}
                }
                Model::UnsignedInt => quote! { u32},
            };

            functions.push(quote! {
                pub unsafe fn #name<F: AsFd>(fd: F, data: &mut #ret) -> rustix::io::Result<()> {
                    unsafe {
                        ioctl(fd, Updater::<{#opcode::<#ret>(DRM_IOCTL_BASE, #code) }, #ret>::new(data))
                    }
                }
            });
        } else {
            if !matches!(action, Action::IO) {
                panic!("Invalid ioctl")
            }

            functions.push(quote! {
                pub unsafe fn #name<F: AsFd>(fd: F) -> rustix::io::Result<()> {
                    unsafe {
                        ioctl(fd, NoArg::<{opcode::none(DRM_IOCTL_BASE, #code)}>::new())
                    }
                }
            });
        };
    }

    let module = quote! {
        #![allow(clippy::missing_safety_doc)]
        use super::sys::DRM_IOCTL_BASE;
        use rustix::ioctl::{NoArg, Updater, ioctl, opcode};
        use std::os::fd::AsFd;

        #(#functions)*
    };

    let mut module_path = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("../src/ioctls.rs")?;

    write!(&mut module_path, "{module}",)?;

    Ok(())
}

struct IoCtl {
    name: String,
    action: Action,
    code: String,
    ret: Option<Model>,
}

#[allow(clippy::upper_case_acronyms)]
enum Action {
    IO,
    IOW,
    IOR,
    IOWR,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO => write!(f, "IO"),
            Self::IOW => write!(f, "IOW"),
            Self::IOR => write!(f, "IOR"),
            Self::IOWR => write!(f, "IOWR"),
        }
    }
}

enum Model {
    Struct(String),
    Union(String),
    UnsignedInt,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Struct(m) | Self::Union(m) => m.fmt(f),
            Self::UnsignedInt => write!(f, "unsigned int"),
        }
    }
}

fn find_type(model: &str) -> Option<Model> {
    if let Some(model) = model.strip_prefix("struct") {
        return Some(Model::Struct(model.trim().to_string()));
    }

    if let Some(model) = model.strip_prefix("union") {
        return Some(Model::Union(model.trim().to_string()));
    }

    if model.strip_prefix("unsigned int").is_some() {
        return Some(Model::UnsignedInt);
    }

    None
}
