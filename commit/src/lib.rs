use clap::*;
use common::*;

#[derive(Parser, Debug)]
#[command(author, version, about="Checks release on devices", long_about = None, trailing_var_arg=true)]
struct Args {
    // Release commit
    #[arg(short = 'c', long = "commit", help = "Commit to check")]
    commit: String,

    // File with devices
    #[arg(short = 'f', long = "file", help = "File with devices, one per line")]
    file: Option<String>,

    // To update
    #[arg(short = 'u', long = "update", help = "Update cache before operation")]
    update: bool,

    // Not equal
    #[arg(
        long = "ne",
        help = "Set this flat to check if commit not equal instead"
    )]
    ne: bool,

    // Devices
    #[arg(help = "Devices")]
    devices: Vec<String>,
}

pub fn execute_command(args: Vec<String>) {
    let args = Args::parse_from(args);
    let all_devices = get_devices(args.update);
    let input_devices = get_input_devices(args.file, Some(args.devices));
    for device in input_devices {
        match get_device_by_name(device.as_str(), &all_devices, false) {
            Some(d) => match get_device_long_info(d) {
                Some(info) => {
                    let passed = |c: &String| {
                        if args.ne {
                            *c != args.commit
                        } else {
                            *c == args.commit
                        }
                    };
                    if passed(&info.commit) {
                        println!("{}{}{}{}{}", OK_STATUS, SEP, device, SEP, info.commit);
                    } else {
                        println!("{}{}{}{}{}", NOT_OK_STATUS, SEP, device, SEP, info.commit);
                    }
                }
                None => println!("{}{}{}{}", NOT_OK_STATUS, SEP, device, SEP),
            },
            None => println!("{}{}{}{}", NOT_OK_STATUS, SEP, device, SEP),
        }
    }
}
