# Clap Config

Automatically merge CLI args, environment variables, config files, and default values in clap apps.

The goal is to be easy to use, and to leave all the clap configuration for clap to deal with,
while removing boilerplate around creating the config struct and merging it with the clap one.

## Usage

```rust
use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use std::fs;

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    flag: String,
}

// You can use any file format that implements Deserialize.
let config_str = fs::read_to_string("/path/to/config.yaml").unwrap();

// Build an ArgMatches so we can see where each value comes from.
let matches = <Opts as CommandFactory>::command().get_matches();
// Build an instance of the auto-generated <YourStruct>Config struct
let config: OptsConfig = serde_yaml::from_str(&config_str).unwrap();

// Merge the two together into your actual struct.
let opts = Opts::from_merged(matches, Some(config));
```

## Todos

- Set serde flags like `#[serde(rename_all = "kebab-case", deny_unknown_fields)]` on the generated struct.
- Set required arguments in the config file.
- Set positional arguments in the config file
- support clap aliases
- Support `Box<SubCommandOpts>`
- Support generating a config file spec with [schemars](https://docs.rs/schemars)
- Copy docstring from parent struct.
