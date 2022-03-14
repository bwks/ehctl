# ehctl

> **Command Line Interface for ExtraHop Appliances**

## License
Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

## Install
```
TODO
```

## Usage
```
./ehctl --help
ehopctl 0.1.2
Extrahop CLI

USAGE:
    ehctl [OPTIONS]

OPTIONS:
        --backup                Backup ExtraHop customizations
    -g, --get <get-endpoint>    ExtraHop API GET
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
[[eca]]
hostname = "eca01.lan"
allow_insecure_tls = false
# Credentials can be defined in here or 
# as an environment variable.
user_id = "setup" 
api_key = "abcd..."

[[eda]]
hostname = "eda01.lan"
allow_insecure_tls = false

[[eda]]
hostname = "eda02.lan"
allow_insecure_tls = false

[[exa]]
hostname = "exa01.lan"
allow_insecure_tls = false
```

### Environment Variables
Credentials can be defined as environment variables.
The varibales must be defined in the following format:
* user_id: `<UPPERCASE_HOSTNAME>_USER_ID`
* api_key: `<UPPERCASE_HOSTNAME>_API_KEY`

> Note: Dashes (-) and/or dots (.) must be converted to underscores to be a valid environment variable.

```ini
export EDA01_LAN_USER_ID="setup"
export EDA01_LAN_API_KEY="qwer..."
```