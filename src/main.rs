mod args;
mod boxes;
mod notes;
mod utils;
mod error;
mod config;

use crate::error::ZkError;
use crate::config::ZkConfig;

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

    let bname = matches.get_one::<String>("box");

    match matches.subcommand() {
        Some(("box", sub_m)) => {
            match boxes::handle_subcommand(sub_m) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            };
        },
        Some(("add", sub_m)) => {
            match notes::add_note(sub_m, bname) {
                Ok(_) => {
                    println!("Note added successfully.");
                },
                Err(e) => {
                    error::warning(&e.to_string());
                },
            }
        },
        Some(("use", sub_m)) => {
            match boxes::use_box(sub_m) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("edit", sub_m)) => {
            match notes::edit_note(sub_m, bname) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("rm", sub_m)) => {
            match notes::rm_note(sub_m, bname) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("show", sub_m)) => {
            match notes::show_note(sub_m, &config, bname) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("ls", sub_m)) => {
            match notes::list_notes(Some(sub_m), bname) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
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
            match notes::import_file(&sub_m) {
                Ok(_) => {
                    println!("file successfully imported to current box");
                },
                Err(e) => {
                    error::warning(&e.to_string());
                }
            };
        },
        Some(("search", sub_m)) => {
            if let Some(needle) = sub_m.get_one::<String>("needle") {
                match notes::search_notes(needle, bname) {
                    Ok(_) => {},
                    Err(e) => {
                        error::warning(&e.to_string());
                    }
                }
            }
        },
        _ => {
            match notes::list_notes(None, bname) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        }
    };

}
