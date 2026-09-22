use std::fs;
use std::assert_matches;
use toml::Table;

use crate::utils;
use crate::error::ZkError;

#[derive(Debug,Default)]
pub struct ZkConfig {
    pub show: ZkShowCommands,
    pub general: ZkGenCommands,
}

#[derive(Debug,Default)]
pub struct ZkGenCommands {
    pub highlight: ZkCmd
}
#[derive(Debug,Default)]
pub struct ZkShowCommands {
    pub md: ZkCmd,
    pub pdf: ZkCmd,
}

#[derive(Debug,PartialEq,Default)]
pub enum ZkCmd {
    cmd(String),
    #[default]
    Invalid,
}

impl ZkConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

pub fn get_config() -> Result<ZkConfig, ZkError> {
    let mut config = utils::get_szettel_dir()?;
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
    if let Some(cmd_tab) = tab.get("show") {
        if let Some(c) = cmd_tab.get("md") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.show.md = ZkCmd::cmd(v.to_string());
        }
        if let Some(c) = cmd_tab.get("pdf") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.show.pdf = ZkCmd::cmd(v.to_string());
        }
    }
    if let Some(cmd_tab) = tab.get("general") {
        if let Some(c) = cmd_tab.get("highlight") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.general.highlight = ZkCmd::cmd(v.to_string());
        }
    }

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
[show]
md = 'glow'
";
    let tab1 = c1.parse::<Table>().unwrap();
    let conf1 = parse_table(tab1).unwrap();
    assert_eq!(conf1.show.md, ZkCmd::cmd(String::from("glow")));

    let c2 = "
[show]
md = 'glow'
pdf = 'sioyek'
";
    let tab2 = c2.parse::<Table>().unwrap();
    let conf2 = parse_table(tab2).unwrap();
    assert_eq!(conf2.show.md, ZkCmd::cmd(String::from("glow")));
    assert_eq!(conf2.show.pdf, ZkCmd::cmd(String::from("sioyek")));


}
