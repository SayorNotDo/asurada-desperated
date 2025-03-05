#[macro_use]
extern crate serde_derive;

use crate::config::Config;
use termion::input::TermRead;
use rand::{rngs::OsRng, RngCore};

use anyhow::{bail, Error, Result};
use std::{env, fs, io, path::Path, process};
use std::fmt::format;
use std::io::Read;
use rand::TryRngCore;

mod config;
mod disk_wrapper;

fn get_target() -> String {
    env::var("TARGET").unwrap_or(option_env!("TARGET").map_or("aarch64.rs-unknown-none".to_string(), |x| x.to_string()))
}

fn hash_password(password: String) -> Result<String> {
    if !password.is_empty() {
        let salt = format!("{:X}", OsRng.try_next_u64()?);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;
        Ok(hash)
    } else {
        Ok("".into())
    }
}

fn prompt_password(prompt: &str, confirm_prompt: &str) -> Result<String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    print!("{} ", prompt);
    let password = stdin.read_passwd(&mut stdout)?;

    print!("\n{}", confirm_prompt);
    let confirm_password = stdin.read_passwd(&mut stdout)?;

    // Note: Actually comparing two Option<String> values
    if confirm_password != password {
        bail!("password do not match");
    }
    Ok(password.unwrap_or("".to_string()))



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
    // 初始化引导程序目录
    let bootloader_dir = format!("/tmp/asurada_installer_bootloader_{}", process::id());
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
