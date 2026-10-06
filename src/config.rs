use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use toml::Table;

use crate::utils;
use crate::error::ZkError;

#[derive(Debug,Default)]
pub struct ZkConfig {
    general: ZkGenCommands,
    show: ZkShowCommands,
    prefix: ZkPrefixes,
}

#[derive(Debug,Default)]
pub struct ZkGenCommands {
    pub editor: ZkCmd,
    pub highlight: u8,
    pub dir: PathBuf,
}
#[derive(Debug,Default)]
pub struct ZkShowCommands {
    pub md: ZkCmd,
    pub pdf: ZkCmd,
}

#[derive(Debug,Default)]
pub struct ZkPrefixes {
    pub md: String,
    pub pdf: String
}

#[derive(Debug,PartialEq,Default)]
pub enum ZkCmd {
    Cmd(String),
    #[default]
    Invalid,
}

impl ZkConfig {
    pub fn new() -> Self {
        let mut z: ZkConfig = Default::default();
        z.general.highlight = 4;
        z.general.dir = match env::home_dir() {
            Some(mut p) => {
                p.push(".szettel");
                p
            },
            None => PathBuf::new()
        };
        z
    }

    pub fn get_szk_home(&self) -> &PathBuf {
        &self.general.dir
    }
    pub fn get_show_cmds(&self) -> &ZkShowCommands {
        &self.show
    }
    pub fn get_editor(&self) -> &ZkCmd {
        &self.general.editor
    }
    pub fn get_highlight(&self) -> &u8 {
        &self.general.highlight
    }
    pub fn get_prefix(&self) -> &ZkPrefixes {
        &self.prefix
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
            Err(_e) => {
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
            config.show.md = ZkCmd::Cmd(v.to_string());
        }
        if let Some(c) = cmd_tab.get("pdf") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.show.pdf = ZkCmd::Cmd(v.to_string());
        }
    }
    if let Some(cmd_tab) = tab.get("general") {
        if let Some(c) = cmd_tab.get("highlight") {
            let v = c.to_string();
            let v = clean_value(&v);
            let u = match u8::from_str(v) {
                Ok(value) => value,
                Err(_) => {
                    return Err(ZkError::ConfigHighlight);
                }
            };
            config.general.highlight = u;
        }
        if let Some(c) = cmd_tab.get("editor") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.general.editor = ZkCmd::Cmd(v.to_string());
        }
        if let Some(c) = cmd_tab.get("dir") {
            let v = c.to_string();
            let v = clean_value(&v);
            if v.starts_with("/") {
                match fs::exists(v) {
                    Ok(true) => {
                        config.general.dir = PathBuf::from(v);
                    }, 
                    Ok(false) => {
                        return Err(ZkError::ConfigHomeDir);
                    },
                    Err(_e) => {
                        return Err(ZkError::Other(String::from("Could not read directory specified in config file.")));
                    }
                };
            } else {
                let mut home = match env::home_dir() {
                    Some(p) => p,
                    None => {
                        PathBuf::new()
                    }
                };
                home.push(v);
                match fs::exists(&home) {
                    Ok(true) => {
                        config.general.dir = PathBuf::from(home);
                    }, 
                    Ok(false) => {
                        return Err(ZkError::ConfigHomeDir);
                    },
                    Err(_e) => {
                        return Err(ZkError::Other(String::from("Could not read directory specified in config file.")));
                    }
                };
            }
        }
    }
    if let Some(cmd_tab) = tab.get("prefix") {
        if let Some(c) = cmd_tab.get("md") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.prefix.md = v.to_string();
        }
        if let Some(c) = cmd_tab.get("pdf") {
            let v = c.to_string();
            let v = clean_value(&v);
            config.prefix.pdf = v.to_string();
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
[general]
editor = 'vim'
highlight = 129

[show]
md = 'glow'
";
    let tab1 = c1.parse::<Table>().unwrap();
    let conf1 = parse_table(tab1).unwrap();
    let mut home = env::home_dir().unwrap();
    home.push(".szettel");
    assert_eq!(conf1.show.md, ZkCmd::Cmd(String::from("glow")));
    assert_eq!(conf1.general.editor, ZkCmd::Cmd(String::from("vim")));
    assert_eq!(conf1.general.highlight, 129u8);
    assert_eq!(conf1.show.pdf, ZkCmd::Invalid);
    assert_eq!(conf1.general.dir, home);

    let c2 = "
[general]
editor = 'nano'
dir = '/root'

[show]
md = 'glow'
pdf = 'sioyek'
";
    let tab2 = c2.parse::<Table>().unwrap();
    let conf2 = parse_table(tab2).unwrap();
    assert_eq!(conf2.show.md, ZkCmd::Cmd(String::from("glow")));
    assert_eq!(conf2.show.pdf, ZkCmd::Cmd(String::from("sioyek")));
    assert_eq!(conf2.general.editor, ZkCmd::Cmd(String::from("nano")));
    assert_eq!(conf2.general.highlight, 4u8);
    assert_eq!(conf2.general.dir, PathBuf::from("/root"));

    let c2 = "
[general]
editor = 'nano'
dir = 'Documents'

[show]
md = 'glow'
pdf = 'sioyek'

[prefix]
md = '  '
pdf = 'd '
";
    let tab2 = c2.parse::<Table>().unwrap();
    let conf2 = parse_table(tab2).unwrap();
    let mut home = env::home_dir().unwrap();
    home.push("Documents");
    assert_eq!(conf2.show.md, ZkCmd::Cmd(String::from("glow")));
    assert_eq!(conf2.show.pdf, ZkCmd::Cmd(String::from("sioyek")));
    assert_eq!(conf2.general.editor, ZkCmd::Cmd(String::from("nano")));
    assert_eq!(conf2.general.highlight, 4u8);
    assert_eq!(conf2.general.dir, home);
    assert_eq!(conf2.prefix.md, String::from("  "));
    assert_eq!(conf2.prefix.pdf, String::from("d "));
}
