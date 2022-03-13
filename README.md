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
TODO
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
user_id = "setup"
api_key = "abcd..."
allow_insecure_tls = false

[[eda]]
hostname = "eda01.lan"
user_id = "setup"
api_key = "1234..."
allow_insecure_tls = false

[[eda]]
hostname = "eda02.lan"
user_id = "setup"
api_key = "poiu..."
allow_insecure_tls = false

[[exa]]
hostname = "exa01.lan"
user_id = "setup"
api_key = "qwer..."
allow_insecure_tls = false
```