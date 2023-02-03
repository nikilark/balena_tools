use clap::*;
#[allow(unused_imports)]
use commit;
use common::{check_balena_installed, BalenaCommand};
#[allow(unused_imports)]
use device;
#[allow(unused_imports)]
use exec;
#[allow(unused_imports)]
use for_each;
use std::collections::HashMap;
use std::str::FromStr;
#[allow(unused_imports)]
use tag;
#[allow(unused_imports)]
use update;

#[derive(Parser, Debug, PartialEq, Clone, Hash, Eq)]
#[clap(rename_all = "snake_case")]
enum BalenaCommands {
    Tag,
    Commit,
    ForEach,
    Device,
    UpdateCache,
    Execute,
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
            "execute" | "exec" | "e" => Ok(BalenaCommands::Execute),
            _ => Err(Self::Err::new(
                std::io::ErrorKind::NotFound,
                "Unknown command",
            )),
        }
    }
}

const COMMAND_HELP: &str = "One of:\n
tag | t -- to set/remove tag\n
commit | c -- to check commit of devices\n
for_each | foreach | fe -- to execute general balena-cli command for a list of devices\n
device | dev | d -- to show info about devices\n
execute | exec | e -- to execute command through balena ssh\n
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
    let mut commands_dict: HashMap<BalenaCommands, Box<dyn BalenaCommand>> = HashMap::new();
    commands_dict.insert(BalenaCommands::Execute, Box::new(exec::ExecCommand {}));
    commands_dict.insert(BalenaCommands::Device, Box::new(device::DeviceCommand {}));
    commands_dict.insert(BalenaCommands::Tag, Box::new(tag::TagCommand {}));
    commands_dict.insert(BalenaCommands::Commit, Box::new(commit::CommitCommand {}));
    commands_dict.insert(
        BalenaCommands::ForEach,
        Box::new(for_each::ForEachCommand {}),
    );
    commands_dict.insert(
        BalenaCommands::UpdateCache,
        Box::new(update::UpdateCommand {}),
    );
    let value = commands_dict
        .get(&Args::parse_from(std::env::args().take(2).collect::<Vec<String>>()).command)
        .unwrap();
    value.execute(std::env::args().skip(1).collect::<Vec<String>>());
}
