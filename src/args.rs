use clap::{Command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    Command::new("zk")
        .subcommand(
            Command::new("box")
                .about("command for managing boxes")
                .arg(Arg::new("list")
                    .short('l')
                )
        )
        .subcommand(
            Command::new("add")
                .about("Add a new note to the current box")
        )
        .get_matches()
}
