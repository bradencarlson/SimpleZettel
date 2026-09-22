use clap::{Command, ArgAction, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    Command::new("szettel")
        .version("0.1.0")
        .arg(
            Arg::new("box")
            .required(false)
            .short('b')
            .long("box")
            .action(ArgAction::Set)
            .help("specify a box other than the current one")
        )
        .subcommand(
            Command::new("box")
                .about("command for managing boxes")
                .subcommand(
                    Command::new("ls")
                    .about("list boxes")
                    .arg(
                        Arg::new("all")
                        .short('a')
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("List all boxes, even those which are not tracked")
                    )
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
                .subcommand(
                    Command::new("track")
                    .about("track a previously removed box")
                    .arg(
                        Arg::new("name")
                        .required(true)
                        .help("name of the box to track")
                    )
                )
        )
        .subcommand(
            Command::new("use")
            .about("set a box as currently in use")
            .arg(
                Arg::new("name")
                .required(true)
                .action(ArgAction::Set)
                .help("the name of the box to use")
            )
        )
        .subcommand(
            Command::new("add")
            .about("Add a new note to the current box")
            .arg(
                Arg::new("name")
                .required(false)
                .action(ArgAction::Set)
                .help("The name of the new note")
            )
            .arg(
                Arg::new("number")
                .required(false)
                .short('n')
                .long("number")
                .action(ArgAction::Set)
                .help("The number of the new note")
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
            .arg(
                Arg::new("pattern")
                .required(false)
                .action(ArgAction::Set)
                .help("pattern to use to match filenames")
            )
            .arg(
                Arg::new("number")
                .required(false)
                .short('n')
                .long("number")
                .conflicts_with("pattern")
                // TODO: Add parser here to ensure it is a valid ZkNumber?
                .help("list files whose number starts with the provided value")
                .action(ArgAction::Set)
            )
        )
        .subcommand(
            Command::new("config")
            .about("Check config file for errors")
        )
        .subcommand(
            Command::new("import")
            .about("Copy a file to the current box")
            .arg(
                Arg::new("path")
                .required(true)
                .help("path of the file to import")
            )
        )
        .subcommand(
            Command::new("search")
            .about("Search for a string amoung notes")
            .arg(
                Arg::new("needle")
                .required(true)
                .help("string to search for")
            )
        )
        .get_matches()
}
