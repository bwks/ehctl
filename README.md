# ehctl
Command Line Interface for ExtraHop Appliances

## Important
> This software is not supported by ExtraHop.

## License
Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

## Goals
Concurrently perform common tasks across multiple ExtraHop devices.
* Health Check
* Backup/Restore
* Firmware Update
* Packet Download
* Report Generation

## Install
```
TODO
```

## Usage
```
./ehctl --help
ehctl 0.1.10
Extrahop CLI

USAGE:
    ehctl.exe [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    backup           Backup device customizations (currently `all` devices are backed up)
    get              Get data from a HTTP GET <endpoint>
    help             Print this message or the help of the given subcommand(s)
    packet-search    Download a packet capture
```

## Config

### Config File
Configs are defined in the `TOML` format. 

Config must be located in: 
* Linux/Mac - `$HOME/.ehctl/config.toml`
* Windows - `$HOMEPATH\.ehctl\config.toml`

```toml
[[ccp]]
hostname = "customer.api.cloud.extrahop.com"
allow_insecure_tls = false

[[eca]]
hostname = "eca01.lan"
allow_insecure_tls = true
# Credentials can be defined in here or as an environment variable.
user_id = "setup" 
api_key = "abcd..."

[[eda]]
hostname = "eda01.lan"
allow_insecure_tls = true

[[eda]]
hostname = "eda02.lan"
allow_insecure_tls = true

[[exa]]
hostname = "exa01.lan"
allow_insecure_tls = true
```

### Environment Variables
Credentials can be defined as environment variables.
The variables must be defined in the following format:
* **user_id**: `<UPPERCASE_HOSTNAME>_USER_ID`
* **api_key**: `<UPPERCASE_HOSTNAME>_API_KEY`

> Note: Dashes (-) and/or dots (.) must be converted to underscores to be a valid environment variable.

#### Linux/Mac
Set environment variables in your `~/.bashrc`, `~/.zshrc`, etc profile.
```ini
# eda01.lan
export EHCTL_CONFIG="${HOME}/.ehctl/config.toml"
export EDA01_LAN_USER_ID="setup"
export EDA01_LAN_API_KEY="qwer..."
```

#### Windows Powershell
Set environment variables in your `$profile`.
```powershell
$env:EHCTL_CONFIG = "$Env:HOMEPATH\.ehctl\config.toml"
$env:EDA01_LAN_USER_ID = "setup"
$env:EDA01_LAN_API_KEY = "qwer..."
```

#### Windows CMD
Set environment variables manually.
```bat
setx EHCTL_CONFIG "%HOMEPATH%\.ehctl\config.toml"
setx EDA01_LAN_USER_ID "setup"
setx EDA01_LAN_API_KEY "qwer..."
```