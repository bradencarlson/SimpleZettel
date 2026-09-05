mod args;
mod boxes;
mod notes;
mod utils;
mod error;
mod config;

fn main() {

    let matches = args::parse_args();

    config::get_config();

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
            match notes::add_note(sub_m) {
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
            match notes::edit_note(sub_m) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("rm", sub_m)) => {
            match notes::rm_note(sub_m) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("show", sub_m)) => {
            match notes::show_note(sub_m) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        Some(("ls", sub_m)) => {
            match notes::list_notes(sub_m) {
                Ok(_) => {},
                Err(e) => {
                    error::warning(&e.to_string());
                }
            }
        },
        _ => {
            println!("No subcommand matched. Run with -h for help.");
        }
    };

}
