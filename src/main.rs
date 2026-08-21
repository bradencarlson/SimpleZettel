mod args;
mod boxes;
mod notes;
mod utils;
mod error;

fn main() {

    let matches = args::parse_args();

    match matches.subcommand() {
        Some(("box", sub_m)) => {
            match boxes::handle_subcommand(sub_m) {
                Ok(_) => {
                    println!("Box added successfully.");
                },
                Err(e) => {
                    println!("Error: {e}");
                }
            };
        },
        Some(("add", sub_m)) => {
            match notes::add_note(sub_m) {
                Ok(_) => {
                    println!("Note added successfully.");
                },
                Err(e) => {
                    println!("Error: {e}");
                },
            }
        },
        Some(("edit", sub_m)) => {
            println!("edit command found");
        },
        Some(("rm", sub_m)) => {
            println!("rm command found");
        },
        Some(("show", sub_m)) => {
            println!("show command found");
        },
        Some(("ls", sub_m)) => {
            println!("ls command found");
        },
        _ => {
            println!("no subcommand matched.");
        }
    };

}
