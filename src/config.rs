use std::fs;
use toml::Table;

use crate::utils;
use crate::error::ZkError;

#[derive(Debug)]
pub struct ZkConfig {
    commands: ZkCommands,
}

#[derive(Debug)]
pub struct ZkCommands {
    show: ZkCmd,
}

#[derive(Debug)]
pub enum ZkCmd {
    cmd(String),
    Invalid,
}

impl ZkConfig {
    pub fn new() -> Self {
        ZkConfig {
            commands: ZkCommands::new()
        }
    }
}

impl ZkCommands {
    pub fn new() -> Self {
        ZkCommands {
            show: ZkCmd::new()
        }
    }
}

impl ZkCmd {
    pub fn new() -> Self {
        ZkCmd::Invalid
    }
}

pub fn get_config() -> Result<ZkConfig, ZkError> {
    let mut config = utils::get_zk_dir()?;
    config.push("config.toml");
    match fs::exists(&config) {
        Ok(true) =>  {},
        Ok(false) => {
            return Err(ZkError::ConfigNotExists);
        },
        Err(_) => {
            return Err(ZkError::ConfigRead);
        }
    };

    if let Ok(s) = fs::read_to_string(&config) {
        match s.parse::<Table>() {
            Ok(tab) => {
                return Ok(parse_table(tab)?);
            },
            Err(e) => {
                return Err(ZkError::ConfigError(s.to_string()));
            }
        };

    } else {
        return Err(ZkError::ConfigRead);
    }
}

fn parse_table(tab: Table) -> Result<ZkConfig, ZkError> {
    let mut config = ZkConfig::new();
    if let Some(v) = tab["show"].as_str() {
        config.commands.show = ZkCmd::cmd(v.to_string());
    }
    println!("{:?}", config);
    Ok(config)
}
