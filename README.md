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
ehopctl 0.1.5
Extrahop CLI

USAGE:
    ehctl [OPTIONS]

OPTIONS:
        --backup                Backup customizations
    -g, --get <get-endpoint>    Get [options...]
    -h, --help                  Print help information
    -V, --version               Print version information
```

## Config
A config file is required in the following location 
* Linux/Mac - `$HOME/.config/ehctl/config.toml`
* Windows - `$HOME\.config\ehctl\config.toml`

### Config File
Configs are defined in the `TOML` format. 

```toml
[[ccp]]
hostname = "customer.api.cloud.extrahop.com"
allow_insecure_tls = false

[[eca]]
hostname = "eca01.lan"
allow_insecure_tls = true
# Credentials can be defined in here or 
# as an environment variable.
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

```ini
# eda01.lan
export EDA01_LAN_USER_ID="setup"
export EDA01_LAN_API_KEY="qwer..."
```