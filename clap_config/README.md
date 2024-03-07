# Clap Config

A crate to allow automatically merging defaults from a config file with clap-processed
command-line argument and environment variable defaults.

The goal is to be easy to use, and to leave all the clap configuration for clap to deal with,
while removing boilerplate around creating the config struct and merging it with the clap one.

## TODO

- Support for setting serde flags like `#[serde(rename_all = "kebab-case",
deny_unknown_fields)]` on the generated struct.
- Support for subcommands.
- Support for setting required arguments in the config file.
