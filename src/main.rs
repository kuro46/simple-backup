#[macro_use]
extern crate log;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    if let Err(e) = std::env::var("RUST_LOG") {
        match e {
            std::env::VarError::NotPresent => std::env::set_var("RUST_LOG", "info"),
            _ => println!("Cannot set logging level to 'into'"),
        }
    }
    pretty_env_logger::init();
    debug!("Logger initialized!");

    let config_path = if let Some(config_path) = base_dirs().find_config_file("config.toml") {
        info!("Path of config file: {:?}", &config_path);
        config_path
    } else {
        info!("Config file is not exists! Creating...");
        create_config_file();
        info!("Created!");
        return;
    };

    debug!("Config file is exists! Openning file...");
    let mut config_file = File::open(&config_path).expect("Unable to open config file");

    let config: Config = {
        debug!("Loading and parsing configuration...");
        let mut file_content = String::new();
        config_file
            .read_to_string(&mut file_content)
            .expect("Unable to read config file");
        toml::from_str(&file_content).expect("Unable to parse config file")
    };

    debug!("Configuration parsed: {:?}", config);

    let is_filled_target_dir = config.target_dir.is_empty();
    let is_filled_dest_dir = config.dest_dir.is_empty();
    if is_filled_target_dir || is_filled_target_dir {
        if is_filled_target_dir {
            error!("'target_dir' is not set! Please fill this variable!");
        }
        if is_filled_dest_dir {
            error!("'dest_dir' is not set! Please fill this variable!");
        }
        return;
    }

    if !Path::new(&config.target_dir).exists() {
        error!("'target_dir' is not exists! Please review configuration!");
        return;
    }
    if !Path::new(&config.dest_dir).exists() {
        error!("'dest_dir' is not exists! Please review configuration!");
        return;
    }

    start_backup(&config);
}

fn create_config_file() {
    let config_path = base_dirs()
        .place_config_file("config.toml")
        .expect("Unable to get path for config file");
    let mut config_file = File::create(&config_path).expect("Unable to create config file");
    let default_config = include_str!("../resources/config.toml");
    write!(&mut config_file, "{}", default_config).expect("Unable to write default config");
}

fn base_dirs() -> xdg::BaseDirectories {
    xdg::BaseDirectories::with_prefix("simple-backup").unwrap()
}

fn start_backup(config: &Config) {
    info!("Backup started!");
    let date_time = Local::now().format("%Y%m%d-%H%M%S");
    let exit_code = Command::new(&config.executable)
        .arg("-avh")
        .arg("--delete")
        .arg("--backup")
        .arg(format!(
            "--backup-dir={}/backup-{}",
            &config.dest_dir, &date_time
        ))
        .arg(&config.target_dir)
        .arg(format!("{}/backup-latest", &config.dest_dir))
        .spawn()
        .expect("Failed to spawn process")
        .wait()
        .expect("Error occurred while waiting for exit");
    info!(
        "Process exited with code {}",
        exit_code
            .code()
            .map(|code| code.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    /// Executable of rsync
    executable: String,
    /// Target directory
    target_dir: String,
    /// Destination directory
    dest_dir: String,
}
