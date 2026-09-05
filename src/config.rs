use std::fs;
use std::assert_matches;
use toml::Table;

use crate::utils;
use crate::error::ZkError;

#[derive(Debug)]
pub struct ZkConfig {
    pub commands: ZkCommands,
}

#[derive(Debug)]
pub struct ZkCommands {
    pub show: ZkCmd,
    pub edit: ZkCmd,
}

#[derive(Debug,PartialEq)]
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
            show: ZkCmd::new(),
            edit: ZkCmd::new()
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
                return Err(ZkError::ConfigError);
            }
        };

    } else {
        return Err(ZkError::ConfigRead);
    }
}

fn parse_table(tab: Table) -> Result<ZkConfig, ZkError> {
    let mut config = ZkConfig::new();
    if let Some(cmd_tab) = tab.get("commands") {
        if let Some(c) = cmd_tab.get("show") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.commands.show = ZkCmd::cmd(v.to_string());
        }
        if let Some(c) = cmd_tab.get("edit") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.commands.edit = ZkCmd::cmd(v.to_string());
        }
    }
    println!("{:?}", config);
    Ok(config)
}

fn clean_value(v: &str) -> &str {
    let v1 = match v.strip_prefix('"') {
        Some(st) => st,
        None => &v
    };
    let v2 = match v1.strip_suffix('"') {
        Some(st) => st,
        None => &v
    };
    v2
}

#[test]
fn config() {
    let c1 = "
[commands]
show = 'glow'
";
    let tab1 = c1.parse::<Table>().unwrap();
    let conf1 = parse_table(tab1).unwrap();
    assert_eq!(conf1.commands.show, ZkCmd::cmd(String::from("glow")));

    let c2 = "
[commands]
show = 'glow'
edit = 'vim'
";
    let tab2 = c2.parse::<Table>().unwrap();
    let conf2 = parse_table(tab2).unwrap();
    assert_eq!(conf2.commands.show, ZkCmd::cmd(String::from("glow")));
    assert_eq!(conf2.commands.edit, ZkCmd::cmd(String::from("vim")));


}
