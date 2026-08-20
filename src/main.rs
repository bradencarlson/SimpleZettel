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
        Some(("edit", sub_matches)) => {
            println!("edit command found");
        },
        Some(("rm", sub_matches)) => {
            println!("rm command found");
        },
        Some(("show", sub_matches)) => {
            println!("show command found");
        },
        Some(("ls", sub_matches)) => {
            println!("ls command found");
        },
        _ => {
            println!("no subcommand matched.");
        }
    };

}
