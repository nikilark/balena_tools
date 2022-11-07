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
## Setting tag on devices
```
balena_tools tag [--rm] -k <KEY> [-v <VALUE>] [-f <FILE>] [DEVICES]
```
Creates (or removes) tag with key==KEY and optional value for each device in FILE and in trail of command.

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools tag -k "my key" -v "my value" -f devices.txt
```

## Checking if device runs/doesn't run release
```
balena_tools commit [--ne] -c <COMMIT> [-f <FILE>] [DEVICES]
```
Checks if commit on devices is equal (or not equal) to COMMIT.

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools commit -c "12345_rest_of_hash" -f devices.txt
```

## Running balena command for every device
```
balena_tools for_each -c <COMMAND> [-f <FILE>] [DEVICES]
```
Runs command for every device.

Both DEVICES and FILE should contain **names** of devices. FILE should contain one name per line
### Example
```
balena_tools for_each -c "tag set my_key my_val" -f devices.txt
```
which is equal to (pseudocode)
```
foreach uuid in devices
{
    balena tag set my_key my_val --device $uuid
}
```