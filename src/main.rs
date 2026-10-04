use std::path::PathBuf;
mod args;
mod boxes;
mod notes;
mod utils;
mod error;
mod config;
mod vcs;

use regex::Regex;

use crate::error::ZkError;
use crate::config::ZkConfig;

use crate::boxes::ZkBox;

fn main() {

    let matches = args::parse_args();

    let config = match config::get_config() {
        Ok(c) => c,
        Err(ZkError::ConfigNotExists) => {
            ZkConfig::new()
        },
        Err(e) => {
            error::warning(&e.to_string());
            ZkConfig::new()
        }
    };

    let color = config.get_highlight();
    let base_dir = config.get_szk_home();
    let editor = config.get_editor();

    let style = anstyle::Style::new().fg_color(Some(anstyle::Ansi256Color::from(*color).into()));

    let bx = match matches.get_one::<String>("box") {
        Some(bname) => {
            let mut home = base_dir.clone();
            home.push(bname);
            Some(ZkBox::from(home))
        },
        None => {
            match boxes::get_current_box(&base_dir) {
                Ok(zkbox) => Some(zkbox),
                Err(e) => {
                    error::warning(&e.to_string());
                    None
                }
            }
        }
    };

    match matches.subcommand() {
        Some(("box", sub_m)) => {
            match sub_m.subcommand() {
                Some(("ls", ssub_m)) => {
                    let all = ssub_m.get_flag("all");
                    /*let b = match boxes::get_boxes(base_dir) {
                        Ok(box_list) => box_list,
                        Err(e) => {
                            error::warning(&e.to_string());
                            return;
                        }
                    };*/
                    match boxes::list_boxes(base_dir, all, color) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                },
                Some(("add", ssub_m)) => {
                    if let Some(name) = ssub_m.get_one::<String>("name") {
                        match boxes::add_box(base_dir, &name) {
                            Ok(_) => {},
                            Err(e) => {
                                error::warning(&e.to_string());
                            }
                        };
                    }
                },
                Some(("rm", ssub_m)) => {
                    if let Some(name) = ssub_m.get_one::<String>("name") {
                        match boxes::remove_tracking(&name) {
                            Ok(_) => {
                                error::info("successfully removed box");
                            },
                            Err(e) => {
                                error::warning(&e.to_string());
                            }
                        };
                    }
                },
                Some(("track", ssub_m)) => {
                    if let Some(name) = ssub_m.get_one::<String>("name") {
                        match boxes::track_box(&name) {
                            Ok(_) => {},
                            Err(e) => {
                                error::warning(&e.to_string());
                            }
                        };
                    }
                },
                _ => {
                    /*let b = match boxes::get_boxes(base_dir) {
                        Ok(box_list) => box_list,
                        Err(e) => {
                            error::warning(&e.to_string());
                            return;
                        }
                    };*/

                    match boxes::list_boxes(base_dir, false, color) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                }
            };
        },
        Some(("add", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(name) = sub_m.get_one::<String>("name") {
                    match bx.add_card(name, editor) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                }
            }
        },
        Some(("use", sub_m)) => {
            if let Some(name) = sub_m.get_one::<String>("name") {
                match boxes::use_box(&name, color) {
                    Ok(_) => {},
                    Err(e) => {
                        error::warning(&e.to_string());
                    }
                }
            }
        },
        Some(("edit", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(name) = sub_m.get_one::<String>("name") {
                    match bx.edit_card(name, editor) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                }
            }
        },
        Some(("rm", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(name) = sub_m.get_one::<String>("name") {
                    match bx.rm_card(name) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                }
            }
        },
        Some(("show", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(name) = sub_m.get_one::<String>("name") {
                    match bx.show_card(name, config.get_show_cmds()) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                }
            }
        },
        Some(("ls", sub_m)) => {
            if let Some(bx) = bx {
                let pattern = match sub_m.get_one::<String>("pattern") {
                    Some(pat) => {
                        match Regex::new(pat) {
                            Ok(p) => Some(p),
                            Err(e) => {
                                error::warning(&e.to_string());
                                None
                            }
                        }
                    },
                    None => {
                        match sub_m.get_one::<String>("number") {
                            Some(n) => {
                                let mut pat = String::from("^");
                                pat.push_str(n.as_str());
                                match Regex::new(&pat) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        error::warning(&e.to_string());
                                        None
                                    }
                                }
                            },
                            None => None
                        }
                    }
                };
                let cards = match bx.get_cards(pattern.as_ref()) {
                    Ok(c) => c, 
                    Err(e) => {
                        error::warning(&e.to_string());
                        return;
                    }
                };
                match notes::list_cards(cards, Some(config.get_prefix()), Some(style)) {
                    Ok(_) => {},
                    Err(e) => {
                        error::warning(&e.to_string());
                    }
                };
            }
        },
        Some(("config", _sub_m)) => {
            let check = match config::get_config() {
                Ok(c) => {
                    error::info("config check passed");
                },
                Err(ZkError::ConfigNotExists) => {
                    error::info("no config file found");
                },
                Err(ZkError::ConfigRead) => {
                    error::warning("config file exists but cannot be read");
                },
                Err(ZkError::ConfigError) => {
                    error::warning("there are errors in the config file");
                },
                Err(e) => {
                    error::warning(&e.to_string());
                }
            };
        },
        Some(("import", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(p) = sub_m.get_one::<String>("path") {
                    let path = PathBuf::from(p);
                    match bx.import_file(&path) {
                        Ok(_) => {},
                        Err(e) => {
                            error::warning(&e.to_string());
                        }
                    };
                }
            }
        },
        Some(("search", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(needle) = sub_m.get_one::<String>("needle") {
                    if let Ok(r) = Regex::new(needle) {
                        match bx.search_cards(&r) {
                            Ok(v) => {
                                println!("{:?}", v);
                            },
                            Err(e) => {
                                error::warning(&e.to_string());
                            }
                        };
                        
                    } else {
                        error::warning("failed to parse pattern");
                    }
                }
            }
        },
        Some(("mv", sub_m)) => {
            if let Some(bx) = bx {
                if let Some(old) = sub_m.get_one::<String>("old") {
                    if let Some(new) = sub_m.get_one::<String>("new") {
                        match bx.move_card(&old, &new) {
                            Ok(_) => {
                                error::info("card renamed successfully.");
                            },
                            Err(e) => {
                                error::warning(&e.to_string());
                            }
                        };
                    }
                }
            }
        },
        _ => {
            if let Some(bx) = bx {
                let cards = match bx.get_cards(None) {
                    Ok(c) => c, 
                    Err(e) => {
                        error::warning(&e.to_string());
                        return;
                    }
                };
                match notes::list_cards(cards, Some(config.get_prefix()), Some(style)) {
                    Ok(_) => {},
                    Err(e) => {
                        error::warning(&e.to_string());
                    }
                };
            }
        }
    };

}
