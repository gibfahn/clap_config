/*!
Tests that parsing a HashMap -> Vec<(String, String)> works.

Special-case pending resolution of one of these issues:
- <https://github.com/clap-rs/clap/issues/3114>
- <https://github.com/clap-rs/clap/discussions/4291>
*/

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use const_format::formatcp;
use pretty_assertions::assert_eq;

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
    /**
    Override a variable to a different value.

    EXAMPLES:

    ❯ my-app --override key=val
    */
    #[clap(long,
        value_delimiter = ',',
        value_name = "KEY_EQ_VAL",
        value_parser = parse_override)]
    pub r#override: Vec<(String, String)>,
}

/// Parse a single `=`-separated key-value pair.
fn parse_override(s: &str) -> Result<(String, String)> {
    s.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or_else(|| eyre!("no `=` found in `{s}`"))
}

const ARG_KEY_1: &str = "key1";
const ARG_KEY_2: &str = "key2";

const ARG_VAL_1: &str = "val1";
const ARG_VAL_2: &str = "val2";

const CONFIG_KEY_1: &str = "key1";
const CONFIG_KEY_2: &str = "key2";
const CONFIG_KEY_3: &str = "key3";

const CONFIG_VAL_1: &str = "val1";
const CONFIG_VAL_2: &str = "val2";
const CONFIG_VAL_3: &str = "val3";

const UNSET_ARGS: [&str; 1] = ["my_app"];
const SET_ARGS: [&str; 5] = [
    "my_app",
    "--override",
    formatcp!("{ARG_KEY_1}={ARG_VAL_1}",),
    "--override",
    formatcp!("{ARG_KEY_2}={ARG_VAL_2}"),
];

const UNSET_CONFIG: &str = "";
const SET_CONFIG: &str = formatcp!(
    "---
override:
  {CONFIG_KEY_1}: {CONFIG_VAL_1}
  {CONFIG_KEY_2}: {CONFIG_VAL_2}
  {CONFIG_KEY_3}: {CONFIG_VAL_3}"
);

/// Not set anywhere, use default value.
#[test]
fn unset() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, Some(config));
    let expected: Vec<(String, String)> = vec![];
    assert_eq!(expected, opts.r#override);
    Ok(())
}

/// Set in config not args, use config.
#[test]
fn set_config() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, Some(config));
    let expected: Vec<(String, String)> = vec![
        (CONFIG_KEY_1.to_owned(), CONFIG_VAL_1.to_owned()),
        (CONFIG_KEY_2.to_owned(), CONFIG_VAL_2.to_owned()),
        (CONFIG_KEY_3.to_owned(), CONFIG_VAL_3.to_owned()),
    ];
    assert_eq!(expected, opts.r#override);
    Ok(())
}

/// Set in args not config, use args.
#[test]
fn set_args() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, Some(config));
    let expected: Vec<(String, String)> = vec![
        (ARG_KEY_1.to_owned(), ARG_VAL_1.to_owned()),
        (ARG_KEY_2.to_owned(), ARG_VAL_2.to_owned()),
    ];
    assert_eq!(expected, opts.r#override);
    Ok(())
}

/// Set in args and config, use args.
#[test]
fn set_both() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, Some(config));
    let expected: Vec<(String, String)> = vec![
        (ARG_KEY_1.to_owned(), ARG_VAL_1.to_owned()),
        (ARG_KEY_2.to_owned(), ARG_VAL_2.to_owned()),
    ];
    assert_eq!(expected, opts.r#override);
    Ok(())
}
