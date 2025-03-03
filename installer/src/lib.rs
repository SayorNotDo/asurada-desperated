#[macro_use]
extern crate serde_derive;

use crate::config::Config;
use anyhow::{Error, Result};
use std::{env, fs, path::Path, process};

mod config;

fn get_target() ->String {
    env::var("TARGET").unwrap_or(option_env!("TARGET").unwrap_or("aarch64-unknown-none".to_string(), |x| x.to_string()),)
}

fn install_packages(config: &Config, dest: &str, cookbook: Option<&str>) {
    let target = &get_target();
}

// 引导程序获取
fn fetch_bootloader(
    config: &Config,
    cookbook: Option<&str>,
    live: bool,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let bootloader_dir = format!("/tmp/asurada_installer_bootloader_{}", process::id());

    // 初始化引导程序目录
    if Path::new(&bootloader_dir).exists() {
        fs::remove_dir_all(&bootloader_dir)?;
    }
    fs::create_dir_all(&bootloader_dir)?;

    let mut bootloader_config = Config::default();
    bootloader_config.general = config.general.clone();

    Ok((vec![], vec![]))
}

fn install_dir(config: Config, output_dir: impl AsRef<Path>, cookbook: Option<&str>) -> Result<()> {
    macro_rules! prompt {
        ($dst:expr, $def:expr, $($arg:tt)*) => {};
    }

    Ok(())
}

fn install_inner(config: Config, output: &Path, cookbook: Option<&str>, live: bool) -> Result<()> {
    println!("Install {config:#?} to {}", output.display());

    if output.is_dir() {
        install_dir(config, output, cookbook)
    } else {
        Err(Error::msg(format!(
            "Output directory already exists: {}",
            output.display()
        )))
    }
}

pub fn install(
    config: Config,
    output: impl AsRef<Path>,
    cookbook: Option<&str>,
    live: bool,
) -> Result<()> {
    install_inner(config, output.as_ref(), cookbook, live)
}
