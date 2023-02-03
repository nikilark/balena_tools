use clap::*;
use common::*;

#[derive(Parser, Debug)]
#[command(author, version, about="Runs command for every device. Eq to \"foreach uuid in devices {balena $command $uuid}\"", long_about = None, trailing_var_arg=true)]
struct Args {
    // Balena command
    #[arg(short = 'c', long = "command", help = "Command to execute")]
    command: String,

    // To update
    #[arg(short = 'u', long = "update", help = "Update cache before operation")]
    update: bool,

    // To update with specific fleet
    #[arg(
        long = "fleet",
        help = "Update cache with specific fleet before operation"
    )]
    fleet: Option<String>,

    // To add --device flag
    #[arg(short = 'd', long = "device", help = "Add --device flag before uuid")]
    device_uuid: bool,

    // File with devices
    #[arg(short = 'f', long = "file", help = "File with devices, one per line")]
    file: Option<String>,

    // Devices
    #[arg(help = "Devices")]
    devices: Vec<String>,
}

pub struct ForEachCommand {}
impl BalenaCommand for ForEachCommand {
    fn execute(&self, args: Vec<String>) {
        let args = Args::parse_from(args);
        let all_devices = get_devices(args.update, args.fleet);
        let input_devices = get_input_devices(args.file, Some(args.devices));
        for device in input_devices {
            match get_device_by_name(device.as_str(), &all_devices, false) {
                Some(d) => {
                    let command = format!(
                        "balena {} {} {}",
                        args.command,
                        if args.device_uuid { "--device" } else { "" },
                        d.uuid
                    );
                    get_output(&command);
                    println!("{}{}{}", OK_STATUS, SEP, device)
                }
                None => println!("{}{}{}", NOT_OK_STATUS, SEP, device),
            }
        }
    }
}
