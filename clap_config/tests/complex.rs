//! Tests that the macro produces usable output for a complex clap derived struct (that doesn't
//! include any subcommands).

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;
use const_format::formatcp;
use pretty_assertions::assert_eq;

const FLAG_STRING_DEFAULT: &str = "flag-string-default";
const FLAG_STRING_ARG: &str = "flag-string-arg";
const FLAG_STRING_CONFIG: &str = "flag-string-config";

const FLAG_OPTIONAL_DEFAULT: Option<String> = None;
const FLAG_OPTIONAL_ARG: &str = "flag-optional-arg";
const FLAG_OPTIONAL_CONFIG: &str = "flag-optional-config";

const UNSET_ARGS: [&str; 1] = ["myapp"];
const SET_ARGS: [&str; 5] = [
    "myapp",
    "--flag-string",
    FLAG_STRING_ARG,
    "--flag-optional",
    FLAG_OPTIONAL_ARG,
];
const UNSET_CONFIG: &str = "";
const SET_CONFIG: &str = formatcp!(
    "flag_string: {FLAG_STRING_CONFIG}
flag_optional: {FLAG_OPTIONAL_CONFIG}"
);

#[derive(ClapConfig, Parser, Debug, PartialEq)]
pub struct Opts {
    #[clap(long, default_value = FLAG_STRING_DEFAULT)]
    flag_string: String,
    #[clap(long)]
    flag_optional: Option<String>,
}

/// Nothing set anywhere.
#[test]
fn test_nothing_set() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        flag_string: FLAG_STRING_DEFAULT.to_owned(),
        flag_optional: FLAG_OPTIONAL_DEFAULT,
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in args, none in config.
#[test]
fn test_args_set() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        flag_string: FLAG_STRING_ARG.to_owned(),
        flag_optional: Some(FLAG_OPTIONAL_ARG.to_owned()),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in config, none in args.
#[test]
fn test_config_set() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        flag_string: FLAG_STRING_CONFIG.to_owned(),
        flag_optional: Some(FLAG_OPTIONAL_CONFIG.to_owned()),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in both.
#[test]
fn test_both_set() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        flag_string: FLAG_STRING_ARG.to_owned(),
        flag_optional: Some(FLAG_OPTIONAL_ARG.to_owned()),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}
