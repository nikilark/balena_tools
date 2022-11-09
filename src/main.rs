use clap::*;
#[allow(unused_imports)]
use commit;
use common::{check_balena_installed, update_cache};
#[allow(unused_imports)]
use device;
#[allow(unused_imports)]
use for_each;
use std::str::FromStr;
#[allow(unused_imports)]
use tag;

#[derive(Parser, Debug, PartialEq, Clone)]
#[clap(rename_all = "snake_case")]
enum BalenaCommands {
    Tag,
    Commit,
    ForEach,
    Device,
    UpdateCache,
}

impl FromStr for BalenaCommands {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tag" | "t" => Ok(BalenaCommands::Tag),
            "commit" | "c" => Ok(BalenaCommands::Commit),
            "for_each" | "foreach" | "fe" => Ok(BalenaCommands::ForEach),
            "device" | "dev" | "d" => Ok(BalenaCommands::Device),
            "update" | "u" => Ok(BalenaCommands::UpdateCache),
            _ => Err(Self::Err::new(
                std::io::ErrorKind::NotFound,
                "Unknown command",
            )),
        }
    }
}

const COMMAND_HELP : &str = "One of:\n
tag | t -- to set/remove tag\n
commit | c -- to check commit of devices\n
for_each | foreach | fe -- to execute general command for list of devices\n
device | dev | d -- to show info about devices\n
update | u -- to update cache";

#[derive(Parser, Debug)]
#[command(author, version, about="Some common operations with balena-cli", long_about = None)]
struct Args {
    // Command
    #[clap(flatten = true, help = COMMAND_HELP)]
    command: BalenaCommands,
}

fn main() {
    if !check_balena_installed() {
        println!(
            "Balena cli not found, please install balena cli : \n {}",
            "https://www.balena.io/docs/reference/balena-cli/"
        );
        return;
    }
    let short_args = std::env::args().take(2).collect::<Vec<String>>();
    let rest_args = std::env::args().skip(1).collect::<Vec<String>>();
    let command = Args::parse_from(short_args).command;
    match command {
        BalenaCommands::Tag => tag::execute_command(rest_args),
        BalenaCommands::Commit => commit::execute_command(rest_args),
        BalenaCommands::ForEach => for_each::execute_command(rest_args),
        BalenaCommands::Device => device::execute_command(rest_args),
        BalenaCommands::UpdateCache => {update_cache();},
    }
}
