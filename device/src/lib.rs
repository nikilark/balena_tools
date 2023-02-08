use clap::*;
use common::*;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about="Shows info about device in cmd or balena", long_about = None, trailing_var_arg=true)]
struct Args {
    // Format of info
    #[arg(long="format", help=DeviceCommand::FORMAT_HELP, default_value="")]
    format: String,

    // To update
    #[arg(short = 'u', long = "update", help = "Update cache before operation")]
    update: bool,

    // To update with specific fleet
    #[arg(
        long = "fleet",
        help = "Update cache with specific fleet before operation"
    )]
    fleet: Option<String>,

    // To search
    #[arg(short = 's', long = "search", help = "Search if name is incomplete")]
    search: bool,

    // File with devices
    #[arg(short = 'f', long = "file", help = "File with devices, one per line")]
    file: Option<String>,

    // Devices
    #[arg(help = "Devices")]
    devices: Vec<String>,
}

pub struct DeviceCommand {}

impl DeviceCommand {
    const FORMAT_HELP: &str =
        "String with format. Leave empty to open device in browser. Possible arguments are: \n
%name, %n,\n
%id, %i,\n
%type, %t -- device type,\n
%status, %s -- device status, will be \"not_found\" if not found,\n
%online, %o,\n
%ip -- list of IPs, coma separated,\n
%pip -- public IP,\n
%mac -- list of MACs, coma separated,\n
%fleet, %f\n
%last_seen, %ls,\n
%uuid, %ui,\n
%commit, %c,\n
%supervisor, %sup,\n
%web -- web accessible,\n
%os,\n
%url,\n
%cpu_usage_percent, %cpu_p,\n
%cpu_temp, %cpu_t,\n
%cpu_id,\n
%memory_used_mb, %memu,\n
%memory_total_mb, %memt,\n
%memory_usage_percent, %memp,\n
%storage_block_device, %stordev,\n
%storage_usage_mb, %storu,\n
%storage_total_mb, %stort,\n
%storage_usage_percent, %storp\n";

    const ALL_ARGS: [&str; 45] = [
        "%name",
        "%n",
        "%id",
        "%i",
        "%type",
        "%t",
        "%status",
        "%s",
        "%online",
        "%o",
        "%ip",
        "%pip",
        "%mac",
        "%fleet",
        "%f",
        "%last_seen",
        "%ls",
        "%uuid",
        "%ui",
        "%commit",
        "%c",
        "%supervisor",
        "%sup",
        "%web",
        "%os",
        "%url",
        "%cpu_usage_percent",
        "%cpu_p",
        "%cpu_temp",
        "%cpu_t",
        "%cpu_id",
        "%memory_used_mb",
        "%memu",
        "%memory_total_mb",
        "%memt",
        "%memory_usage_percent",
        "%memp",
        "%storage_block_device",
        "%stordev",
        "%storage_usage_mb",
        "%storu",
        "%storage_total_mb",
        "%stort",
        "%storage_usage_percent",
        "%storp",
    ];

    fn give_relevant(&self, arg: &str, info: &DeviceLong, name: &String) -> String {
        match arg {
            "%name" | "%n" => name.clone(),
            "%id" | "%i" => info.id.clone(),
            "%type" | "%t" => info.device_type.clone(),
            "%status" | "%s" => info.status.clone(),
            "%online" | "%o" => info.is_online.to_string().clone(),
            "%ip" => info.ip.join(","),
            "%pip" => info.public_ip.clone(),
            "%mac" => info.mac.join(","),
            "%fleet" | "%f" => info.fleet.clone(),
            "%last_seen" | "%ls" => info.last_seen.clone(),
            "%uuid" | "%ui" => info.uuid.clone(),
            "%commit" | "%c" => info.commit.clone(),
            "%supervisor" | "%sup" => info.supervisor.clone(),
            "%web" => info.web_accessible.to_string().clone(),
            "%os" => info.os.clone(),
            "%url" => info.url.clone(),
            "%cpu_usage_percent" | "%cpu_p" => info.cpu_usage_percent.to_string().clone(),
            "%cpu_temp" | "%cpu_t" => info.cpu_temp.to_string().clone(),
            "%cpu_id" => info.cpu_id.clone(),
            "%memory_used_mb" | "%memu" => info.memory_used_mb.to_string().clone(),
            "%memory_total_mb" | "%memt" => info.memory_total_mb.to_string().clone(),
            "%memory_usage_percent" | "%memp" => info.memory_usage_percent.to_string().clone(),
            "%storage_block_device" | "%stordev" => info.storage_block_device.clone(),
            "%storage_usage_mb" | "%storu" => info.storage_usage_mb.to_string().clone(),
            "%storage_total_mb" | "%stort" => info.storage_total_mb.to_string().clone(),
            "%storage_usage_percent" | "%storp" => info.storage_usage_percent.to_string().clone(),
            _ => String::new(),
        }
    }
    pub fn open_url(&self, device: Device) {
        let open_with = match env::consts::OS {
            "windows" => "Start-Process",
            "linux" => "xdg-open",
            _ => "",
        };
        get_output(&format!("{} {}", open_with, device.url));
    }
    fn get_device_info(&self, d: Option<Device>) -> DeviceLong {
        let mut not_found_device = DeviceLong::default();
        not_found_device.status = "not_found".to_string();
        match d {
            Some(d) => get_device_long_info(d).unwrap_or(not_found_device),
            None => not_found_device,
        }
    }
}

impl BalenaCommand for DeviceCommand {
    fn execute(&self, args: Vec<String>) {
        let args = Args::parse_from(args);
        let all_devices = get_devices(args.update, args.fleet);
        let input_devices = get_input_devices(args.file, Some(args.devices));
        for device in input_devices {
            let device_short = get_device_by_name(device.as_str(), &all_devices, args.search);
            let device_name = match device_short.clone() {
                Some(d) => d.name,
                None => device.clone()
            };
            if args.format.is_empty() {
                if device_short.is_some() {
                    self.open_url(device_short.unwrap());
                    println!("{}{}{}", OK_STATUS, SEP, device_name);
                } else {
                    println!("{}{}{}", NOT_OK_STATUS, SEP, device_name);
                }
            } else {
                let device_info = self.get_device_info(device_short);
                let mut result = args.format.clone();
                for arg in Self::ALL_ARGS {
                    result = result.replace(arg, &self.give_relevant(arg, &device_info, &device_name));
                }
                result = result.replace("\\t", "\t");
                println!("{}", result);
            }
        }
    }
}
