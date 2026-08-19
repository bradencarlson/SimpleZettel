use clap::{Command, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches {
    Command::new("zk")
        .subcommand(
            Command::new("box")
                .arg(Arg::new("list")
                    .short('l')
                )
        )
        .get_matches()
}
