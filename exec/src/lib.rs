use clap::*;
use common::*;

#[derive(Parser, Debug)]
#[command(author, version, about="Executes ssh command for every device", long_about = None, trailing_var_arg=true)]
struct Args {
    // Balena command
    #[arg(short = 'c', long = "command", help = "Command to execute. Single quotes not allowed")]
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

    // Run in specific container. If not specified command will run in host os
    #[arg(short = 's', long = "service", help = "Service to run command. Leave empty to run in host os")]
    service: Option<String>,

    // File with devices
    #[arg(short = 'f', long = "file", help = "File with devices, one per line")]
    file: Option<String>,

    // Devices
    #[arg(help = "Devices")]
    devices: Vec<String>,
}

fn create_ssh_command(device_uuid: &String, service: &Option<String>, command: &String) -> String {
    let safe_command = command.replace("\"", "\\\"");
    let ssh_command = if service.is_some() {
        format!(
            "balena exec \\$(balena container ls -q -f label=io.balena.service-name={}) sh -c '{}'",
            service.as_ref().unwrap(),
            safe_command
        )
    } else {
        format!("{}", safe_command)
    };
    return format!(
        "echo \"{}; exit\" | balena ssh {} | tail -n +4",
        ssh_command, device_uuid
    );
}

pub fn execute_command(args: Vec<String>) {
    let args = Args::parse_from(args);
    if args.command.contains("'") {
        println!("Single quotes are not allowed in commands. Please use double quotes only");
        return;
    }
    let all_devices = get_devices(args.update, args.fleet);
    let input_devices = get_input_devices(args.file, Some(args.devices));
    for device in input_devices {
        match get_device_by_name(device.as_str(), &all_devices, false) {
            Some(d) => {
                let command = create_ssh_command(&d.uuid, &args.service, &args.command);
                let output = get_output(&command);
                println!("{}{}{}{}{}", OK_STATUS, SEP, device, SEP, output)
            }
            None => println!("{}{}{}{}", NOT_OK_STATUS, SEP, device, SEP),
        }
    }
}
