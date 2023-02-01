use std::env;
use std::process::Command;

pub const SEP: &str = "    ";
pub const OK_STATUS: &str = "OK    ";
pub const NOT_OK_STATUS: &str = "NOT_OK";
const DEVICES_CACHE: &str = "/cache/devices.txt";

#[derive(Default, Debug, Clone)]
pub struct Device {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub device_type: String,
    pub fleet: String,
    pub status: String,
    pub online: bool,
    pub supervisor: String,
    pub os: String,
    pub os_version: String,
    pub url: String,
}

#[derive(Default, Debug, Clone)]
pub struct DeviceLong {
    pub id: String,
    pub device_type: String,
    pub status: String,
    pub is_online: bool,
    pub ip: Vec<String>,
    pub public_ip: String,
    pub mac: Vec<String>,
    pub fleet: String,
    pub last_seen: String,
    pub uuid: String,
    pub commit: String,
    pub supervisor: String,
    pub web_accessible: bool,
    pub os: String,
    pub url: String,
    pub cpu_usage_percent: u8,
    pub cpu_temp: u8,
    pub cpu_id: String,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_usage_percent: u8,
    pub storage_block_device: String,
    pub storage_usage_mb: u128,
    pub storage_total_mb: u128,
    pub storage_usage_percent: u8,
}

pub trait BalenaCommand {
    fn execute(&self, args : Vec<String>);
}

fn get_cache_path() -> String {
    return format!(
        "{}{}",
        std::env::current_exe().unwrap().parent().unwrap().display(),
        DEVICES_CACHE
    );
}

pub fn output_to_string(output: std::process::Output) -> String {
    std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .to_string()
}

pub fn get_output(command: &str) -> String {
    let shell = match env::consts::OS {
        "windows" => "powershell",
        "linux" => "bash",
        _ => "sh",
    };
    output_to_string(Command::new(shell).arg("-c").arg(command).output().unwrap())
}

pub fn check_balena_installed() -> bool {
    !get_output("balena --version").is_empty()
}

fn line_to_device(line: &str) -> Device {
    let splitted: Vec<&str> = line.split_whitespace().collect();
    if splitted.len() != 11 {
        Device::default()
    } else {
        Device {
            id: splitted[0].to_string(),
            uuid: splitted[1].to_string(),
            name: splitted[2].to_string(),
            device_type: splitted[3].to_string(),
            fleet: splitted[4].to_string(),
            status: splitted[5].to_string(),
            online: splitted[6].to_string().to_lowercase() == "true",
            supervisor: splitted[7].to_string(),
            os: splitted[8].to_string(),
            os_version: splitted[9].to_string(),
            url: splitted[10].to_string(),
        }
    }
}

pub fn update_cache(fleet: Option<String>) -> String {
    let c = get_cache_path();
    let path = std::path::Path::new(&c);
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    }
    let res = if fleet.is_none() {
        get_output("balena devices")
    } else {
        get_output(format!("balena devices --fleet {}", fleet.unwrap()).as_str())
    };
    if res.contains("BalenaApplicationNotFound") {
        eprintln!("{}", res);
    }
    std::fs::write(c, res.clone()).unwrap();
    res
}

pub fn get_devices(force_update: bool, fleet: Option<String>) -> Vec<Device> {
    let mut file_content = if force_update || fleet.is_some() {
        update_cache(fleet.clone())
    } else {
        std::fs::read_to_string(get_cache_path()).unwrap_or("".to_string())
    };
    if file_content.is_empty() && !force_update {
        file_content = update_cache(fleet);
    }
    file_content
        .split('\n')
        .skip(1)
        .map(line_to_device)
        .collect()
}

pub fn get_device_by_name(name: &str, devices: &Vec<Device>, incomplete: bool) -> Option<Device> {
    let found: Vec<Device> = devices
        .iter()
        .filter(|d| {
            if incomplete {
                d.name.contains(&name)
            } else {
                d.name == name
            }
        })
        .map(|f| f.clone())
        .collect();
    if found.len() == 1 {
        Some(found[0].clone())
    } else if found.len() > 1 {
        eprintln!("Device {} found more than once, skipping it", name);
        None
    } else {
        None
    }
}

pub fn get_device_long_info(device: Device) -> Option<DeviceLong> {
    let command = format!("balena device {}", device.uuid);
    let output_fields: Vec<String> = get_output(command.as_str())
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    if output_fields.len() != 26 {
        return None;
    }
    let get_value = |f: &String| f[f.find(':').unwrap() + 1..].trim().to_string();
    let mut cnt = 0;
    let mut get_next_value = || {
        cnt += 1;
        get_value(&output_fields[cnt])
    };
    Some(DeviceLong {
        id: get_next_value(),
        device_type: get_next_value(),
        status: get_next_value(),
        is_online: get_next_value().to_lowercase() == "true",
        ip: get_next_value().split(' ').map(|s| s.to_string()).collect(),
        public_ip: get_next_value(),
        mac: get_next_value().split(' ').map(|s| s.to_string()).collect(),
        fleet: get_next_value(),
        last_seen: get_next_value(),
        uuid: get_next_value(),
        commit: get_next_value(),
        supervisor: get_next_value(),
        web_accessible: get_next_value().to_lowercase() == "true",
        os: get_next_value(),
        url: get_next_value(),
        cpu_usage_percent: get_next_value().parse().unwrap_or(0),
        cpu_temp: get_next_value().parse().unwrap_or(0),
        cpu_id: get_next_value(),
        memory_used_mb: get_next_value().parse().unwrap_or(0),
        memory_total_mb: get_next_value().parse().unwrap_or(0),
        memory_usage_percent: get_next_value().parse().unwrap_or(0),
        storage_block_device: get_next_value(),
        storage_usage_mb: get_next_value().parse().unwrap_or(0),
        storage_total_mb: get_next_value().parse().unwrap_or(0),
        storage_usage_percent: get_next_value().parse().unwrap_or(0),
    })
}

pub fn get_input_devices(file: Option<String>, other: Option<Vec<String>>) -> Vec<String> {
    let mut devices: Vec<String> = Vec::new();
    if file.is_some() {
        let file = file.unwrap();
        match std::fs::read_to_string(file.clone()) {
            Ok(list) => {
                devices.append(
                    &mut list
                        .trim()
                        .split('\n')
                        .map(|d| d.trim().to_string())
                        .filter(|d| !d.is_empty())
                        .collect(),
                );
            }
            Err(_) => {
                eprintln!("Failed to read file {}", file);
            }
        }
    }
    if other.is_some() {
        devices.append(&mut other.unwrap());
    }
    devices.sort();
    devices.dedup();
    if devices.contains(&"*".to_string()) {
        get_devices(false, None)
            .iter()
            .map(|d| d.name.clone())
            .collect()
    } else {
        devices
    }
}
