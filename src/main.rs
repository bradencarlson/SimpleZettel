use std::process::{Command,Stdio};

pub mod args;
fn main() {

    let matches = args::parse_args();

    match matches.subcommand() {
        Some(("box", sub_matches)) => {
            println!("box command found");
        },
        Some(("add", sub_matches)) => {
            println!("add command found");
        },
        _ => {
            println!("no subcommand matched.");
        }
    };

}
