use common::*;
use clap::*;

#[derive(Parser, Debug)]
#[command(author, version, about="Sets tags to device", long_about = None, trailing_var_arg=true)]
struct Args {
    // Tag key to set
    #[arg(short='k',long="key", help="Key of tag to set")]
    tag_key: String,

    // Tag value to set
    #[arg(short='v',long="val", default_value="", help="Value of tag to set, can be empty")]
    tag_value: String,

    // File with devices
    #[arg(short='f', long = "file", help="File with devices, one per line")]
    file: Option<String>,

    // To remove
    #[arg(long="rm", help="Set this flag to remove specific key")]
    remove: bool,

    // Devices
    #[arg(help="Devices")]
    devices: Vec<String>,
}

pub fn set_tag(key : &str, value : &str, to : &Device, remove : bool) {
    if !remove {
        get_output(format!("balena tag set {} {} --device {}", key, value, to.uuid).as_str());
    }
    else {
        get_output(format!("balena tag rm {} --device {}", key, to.uuid).as_str());
    }
}

pub fn execute_command(args : Vec<String>) {
    let args = Args::parse_from(args);
    let all_devices = get_devices();
    let input_devices = get_input_devices(args.file, Some(args.devices));
    for device in input_devices {
        match get_device_by_name(device.as_str(), &all_devices) {
            Some(d) => {
                set_tag(&args.tag_key, &args.tag_value, &d, args.remove);
                println!("{}{}{}", OK_STATUS, SEP, device);
            },
            None => println!("{}{}{}", NOT_OK_STATUS, SEP, device)
        }
    }
}