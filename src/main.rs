#[allow(unused_imports)]
use commit;
#[allow(unused_imports)]
use for_each;
#[allow(unused_imports)]
use tag;
use common::check_balena_installed;
use clap::*;
use std::str::FromStr;

#[derive(Parser, Debug, PartialEq, Clone)]
#[clap(rename_all = "snake_case")]
enum BalenaCommands {
    Tag,
    Commit,
    ForEach
}

impl FromStr for BalenaCommands {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tag" => Ok(BalenaCommands::Tag),
            "commit" => Ok(BalenaCommands::Commit),
            "for_each" => Ok(BalenaCommands::ForEach),
            _ => Err(Self::Err::new(std::io::ErrorKind::NotFound, "Unknown command"))
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about="Some common operations with balena-cli", long_about = None)]
struct Args {
    // Command
    #[clap(flatten=true, help="One of \"tag\", \"commit\", \"for_each\"")]
    command: BalenaCommands
}

fn main() {
    if !check_balena_installed(){
        println!("Balena cli not found, please install balena cli : \n {}", "https://www.balena.io/docs/reference/balena-cli/");
        return;
    }
    let short_args = std::env::args().take(2).collect::<Vec<String>>();
    let rest_args = std::env::args().skip(1).collect::<Vec<String>>();
    let command = Args::parse_from(short_args).command;
    match command {
        BalenaCommands::Tag => tag::execute_command(rest_args),
        BalenaCommands::Commit => commit::execute_command(rest_args),
        BalenaCommands::ForEach => for_each::execute_command(rest_args),
    }
}
