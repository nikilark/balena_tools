# Balena tools
### Some common operations with balena-cli
# Installing
Can be built with cargo for your platform. From root
```
cargo build --release
```
Packages for x86_64_windows and x86_64_linux can be found in releases folder
# General usage
```
balena_tools <COMMAND> args
```
make sure you have balena-cli installed and added to PATH. You can check it by running
```
balena --version
```
and don't forget to login
```
balena login
```
## Caching
Getting list of balena devices takes some time, so tool saves list to file and then reads it. If some device wasn't found you can refresh list of devices with
```
balena_tools update
```
Also every command supports flag `-u` and arg '--fleet' to do update list before execution

## Getting information about devices
```
balena_tools device [-u] [-s] [--fleet <FLEET>] [--format <FORMAT_STRING>] [-f <FILE>] [DEVICES]
```
Aliases :
- device
- dev
- d

Prints information about devices **OR** opens device url if format is empty. Please check 
```
balena_tools device --help
```
for all formatting options. In short, it supports every field found in `balena device <uuid>`

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools dev DEVICE_NAME
```
will open url of DEVICE_NAME in browser
```
balena_tools dev --format "%name\t%status\t%commit" -f devices.txt
```
will list information about devices in devices.txt, columns : name, status, commit

## Setting tag on devices
```
balena_tools tag [--rm] [-u] [--fleet <FLEET>] -k <KEY> [-v <VALUE>] [-f <FILE>] [DEVICES]
```
Aliases :
- tag
- t

Creates (or removes) tag with key==KEY and optional value for each device in FILE and in trail of command.

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools tag -k "my key" -v "my value" -f devices.txt
```

## Checking if device runs/doesn't run release
```
balena_tools commit [--ne] [-u] [--fleet <FLEET>] -c <COMMIT> [-f <FILE>] [DEVICES]
```
Aliases :
- commit
- c

Checks if commit on devices is equal (or not equal) to COMMIT.

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools commit -c "12345_rest_of_hash" -f devices.txt
```


## Execute command through balena ssh
```
balena_tools exec [-u] [--fleet <FLEET>] [--service <SERVICE>] --command <COMMAND> [DEVICES]
```
Aliases :
- execute
- e

Executes shell command through balena ssh

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools exec --command "uptime" --service myservice -f devices.txt
```

## Running general balena command for every device
```
balena_tools for_each [-u] [-d] [--fleet <FLEET>] -c <COMMAND> [-f <FILE>] [DEVICES]
```
Aliases (case **not** sensitive):
- for_each
- foreach
- fe

Runs command for every device.

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line

Please note, that some balena commands require putting `--device` before uuid and others not. Please check the [balena cli doc](https://www.balena.io/docs/reference/balena-cli/). To add `--device` flag before command you need to specify `-d` in balena_tools
### Example
```
balena_tools for_each -d -c "tag set my_key my_val" -f devices.txt
```
which is equal to (pseudocode)
```
foreach uuid in devices
{
    balena tag set my_key my_val --device $uuid
}
```

```
balena_tools for_each -c "device reboot" MY_DEVICE_FIRST MY_DEVICE_SECOND
```
which is equal to (pseudocode)
```
foreach uuid in [MY_DEVICE_FIRST.uuid, MY_DEVICE_SECOND.uuid]
{
    balena device reboot $uuid
}
```

## Updating cache
```
balena_tools update [--fleet <FLEET>] 
```
Aliases
- update
- u

Writes output of `balena devices [--fleet <FLEET>]` to <exe_path>/cache/devices.txt
