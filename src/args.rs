use clap::{Command, ArgAction, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    Command::new("zk")
        .version("0.1.0")
        .subcommand(
            Command::new("box")
                .about("command for managing boxes")
                .subcommand(
                    Command::new("ls")
                    .about("list boxes")
                )
                .subcommand(
                    Command::new("add")
                    .about("Add a new box")
                    .arg(
                        Arg::new("name")
                        .required(true)
                        .action(ArgAction::Set)
                        .help("The name of the new box")
                    )
                )
                .subcommand(
                    Command::new("rm")
                    .about("Remove a box (does not delete files)")
                    .arg(
                        Arg::new("name")
                        .required(true)
                        .action(ArgAction::Set)
                        .help("The name of the box to remove from tracking")
                    )
                )
        )
        .subcommand(
            Command::new("add")
            .about("Add a new note to the current box")
            .arg(
                Arg::new("name")
                .action(ArgAction::Set)
                .help("The name of the new note")
            )
        )
        .subcommand(
            Command::new("edit")
            .about("Edit a note")
            .arg(
                Arg::new("name")
                .required(true)
                .help("Name of the note to edit.")
            )
        )
        .subcommand(
            Command::new("rm")
            .about("remove a note")
            .arg(
                Arg::new("name")
                .required(true)
                .help("The name of the note to remove")
            )
        )
        .subcommand(
            Command::new("show")
            .about("show a note")
            .arg(
                Arg::new("name")
                .required(true)
                .help("The name of the note to show")
            )

        )
        .subcommand(
            Command::new("ls")
            .about("List notes in box")
        )
        .get_matches()
}
