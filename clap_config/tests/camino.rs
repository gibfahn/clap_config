//! Tests that the macro works with a field named `config` of type `camino::Utf8PathBuf`.
//!
//! This exercises the fix for parameter name shadowing: if a field is named `config` (or
//! `matches`), the generated `from_merged` method's parameters must not clash with those names.

use camino::Utf8PathBuf;
use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;

const CONFIG_DEFAULT: &str = "/default/path";
const CONFIG_ARG: &str = "/arg/path";
const CONFIG_CONFIG: &str = "/config/path";

const FLAG_DEFAULT: &str = "flag-default";
const FLAG_ARG: &str = "flag-arg";
const FLAG_CONFIG: &str = "flag-config";

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
    /// Path to the config file.
    #[clap(long, default_value = CONFIG_DEFAULT)]
    config: Utf8PathBuf,

    #[clap(long, default_value = FLAG_DEFAULT)]
    flag: String,
}

#[test]
fn main() -> Result<()> {
    let unset_args = ["myapp"];
    let set_args = ["myapp", "--config", CONFIG_ARG, "--flag", FLAG_ARG];

    let unset_config = "";
    let set_config = &format!("config: {CONFIG_CONFIG}\nflag: {FLAG_CONFIG}");

    // Not set anywhere, use default value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(unset_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(Utf8PathBuf::from(CONFIG_DEFAULT), opts.config);
        assert_eq!(FLAG_DEFAULT, opts.flag);
    }

    // Set in args, use that value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(set_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(Utf8PathBuf::from(CONFIG_ARG), opts.config);
        assert_eq!(FLAG_ARG, opts.flag);
    }

    // Set in config, use that value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(unset_args);
        let config: OptsConfig = serde_yaml::from_str(set_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(Utf8PathBuf::from(CONFIG_CONFIG), opts.config);
        assert_eq!(FLAG_CONFIG, opts.flag);
    }

    // Set in both, use args value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(set_args);
        let config: OptsConfig = serde_yaml::from_str(set_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(Utf8PathBuf::from(CONFIG_ARG), opts.config);
        assert_eq!(FLAG_ARG, opts.flag);
    }

    Ok(())
}
