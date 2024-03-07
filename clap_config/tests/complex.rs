/*!
Tests that the macro produces usable output for a complex clap derived struct (that doesn't
include any subcommands).
*/

use bytesize::ByteSize;
use clap::ArgAction;
use clap::CommandFactory;
use clap::Parser;
use clap::ValueEnum;
use clap_config::ClapConfig;
use color_eyre::Result;
use const_format::formatcp;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

const FLAG_STRING_DEFAULT: &str = "flag-string-default";
const FLAG_STRING_ARG: &str = "flag-string-arg";
const FLAG_STRING_CONFIG: &str = "flag-string-config";

const FLAG_BOOL_A_DEFAULT: bool = false;
const FLAG_BOOL_A_ARG: bool = true;
const FLAG_BOOL_A_CONFIG: bool = false;

const FLAG_BOOL_B_DEFAULT: bool = false;
const FLAG_BOOL_B_ARG: bool = false;
const FLAG_BOOL_B_CONFIG: bool = true;

const FLAG_OPTION_STRING_DEFAULT: Option<String> = None;
const FLAG_OPTION_STRING_ARG: &str = "flag-option-string-arg";
const FLAG_OPTION_STRING_CONFIG: &str = "flag-option-string-config";

const FLAG_VEC_SINGLE_DEFAULT: Vec<String> = Vec::new();
const FLAG_VEC_SINGLE_ARG: &str = "flag-vec-single-arg";
const FLAG_VEC_SINGLE_CONFIG: &str = "flag-vec-single-config";

const FLAG_VEC_MULTIPLE_DEFAULT_A: &str = "flag-vec-multiple-default-a";
const FLAG_VEC_MULTIPLE_DEFAULT_B: &str = "flag-vec-multiple-default-b";
const FLAG_VEC_MULTIPLE_ARG_A: &str = "flag-vec-multiple-arg-a";
const FLAG_VEC_MULTIPLE_ARG_B: &str = "flag-vec-multiple-arg-b";
const FLAG_VEC_MULTIPLE_ARG_C: &str = "flag-vec-multiple-arg-c";
const FLAG_VEC_MULTIPLE_CONFIG_A: &str = "flag-vec-multiple-config-a";
const FLAG_VEC_MULTIPLE_CONFIG_B: &str = "flag-vec-multiple-config-b";
const FLAG_VEC_MULTIPLE_CONFIG_C: &str = "flag-vec-multiple-config-c";
const FLAG_VEC_MULTIPLE_CONFIG_D: &str = "flag-vec-multiple-config-d";

const FLAG_BYTESIZE_DEFAULT: &str = "200GB";
const FLAG_BYTESIZE_ARG: &str = "100MB";
const FLAG_BYTESIZE_CONFIG: &str = "20Gi";

const FLAG_ENUM_DEFAULT: FlagEnum = FlagEnum::DefaultValue;
const FLAG_ENUM_ARG: &str = "arg-value";
const FLAG_ENUM_CONFIG: &str = "ConfigValue";

const POSITIONAL_STRING_ARG: &str = "positional-string-arg";

const UNSET_ARGS: [&str; 1] = ["myapp"];
const SET_ARGS: [&str; 22] = [
    "myapp",
    "--flag-string",
    FLAG_STRING_ARG,
    "--flag-bool-a",
    "--flag-bool-b=false",
    "--flag-option-string",
    FLAG_OPTION_STRING_ARG,
    "--flag-vec-single",
    FLAG_VEC_SINGLE_ARG,
    "--flag-vec-multiple",
    FLAG_VEC_MULTIPLE_ARG_A,
    "--flag-vec-multiple",
    FLAG_VEC_MULTIPLE_ARG_B,
    "--flag-vec-multiple",
    FLAG_VEC_MULTIPLE_ARG_C,
    "--flag-bytesize",
    FLAG_BYTESIZE_ARG,
    "--flag-enum",
    FLAG_ENUM_ARG,
    "--flag-option-enum",
    FLAG_ENUM_ARG,
    POSITIONAL_STRING_ARG,
];

const UNSET_CONFIG: &str = "";
const SET_CONFIG: &str = formatcp!(
    "---
flag_string: {FLAG_STRING_CONFIG}

flag_bool_a: {FLAG_BOOL_A_CONFIG}

flag_bool_b: {FLAG_BOOL_B_CONFIG}

flag_option_string: {FLAG_OPTION_STRING_CONFIG}

flag_vec_single: [{FLAG_VEC_SINGLE_CONFIG}]

flag_vec_multiple:
- {FLAG_VEC_MULTIPLE_CONFIG_A}
- {FLAG_VEC_MULTIPLE_CONFIG_B}
- {FLAG_VEC_MULTIPLE_CONFIG_C}
- {FLAG_VEC_MULTIPLE_CONFIG_D}

flag_bytesize: {FLAG_BYTESIZE_CONFIG}

flag_enum: {FLAG_ENUM_CONFIG}

flag_option_enum: {FLAG_ENUM_CONFIG}
"
);

#[derive(ClapConfig, Parser, Debug, PartialEq)]
pub struct Opts {
    #[clap(long, default_value = FLAG_STRING_DEFAULT)]
    flag_string: String,

    #[clap(long)]
    flag_bool_a: bool,

    #[clap(
        long,
        default_value_t = FLAG_BOOL_B_DEFAULT,
        // If the user passes --flag-bool-b with no argument, set this to true.
        default_missing_value = "true",
        require_equals = true,
        action = ArgAction::Set,
        num_args(0..=1),
    )]
    flag_bool_b: bool,

    #[clap(long)]
    flag_option_string: Option<String>,

    #[clap(long)]
    flag_vec_single: Vec<String>,

    #[clap(long, default_values_t = vec![FLAG_VEC_MULTIPLE_DEFAULT_A.to_owned(), FLAG_VEC_MULTIPLE_DEFAULT_B.to_owned()])]
    flag_vec_multiple: Vec<String>,

    #[clap(long, default_value = FLAG_BYTESIZE_DEFAULT)]
    flag_bytesize: ByteSize,

    #[clap(value_enum, long, default_value_t)]
    flag_enum: FlagEnum,

    #[clap(value_enum, long)]
    flag_option_enum: Option<FlagEnum>,

    // No clap(long) here, this is a positional arg, which can't be set via config.
    positional_string: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Default, Serialize, Deserialize, PartialEq)]
pub enum FlagEnum {
    #[default]
    DefaultValue,
    ArgValue,
    ConfigValue,
}

/// Nothing set anywhere.
#[test]
fn test_nothing_set() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        flag_string: FLAG_STRING_DEFAULT.to_owned(),
        flag_bool_a: FLAG_BOOL_A_DEFAULT,
        flag_bool_b: FLAG_BOOL_B_DEFAULT,
        flag_option_string: FLAG_OPTION_STRING_DEFAULT,
        flag_vec_single: FLAG_VEC_SINGLE_DEFAULT,
        flag_vec_multiple: vec![
            FLAG_VEC_MULTIPLE_DEFAULT_A.to_owned(),
            FLAG_VEC_MULTIPLE_DEFAULT_B.to_owned(),
        ],
        flag_bytesize: ByteSize::from_str(FLAG_BYTESIZE_DEFAULT).unwrap(),
        flag_enum: FLAG_ENUM_DEFAULT,
        flag_option_enum: None,
        positional_string: None,
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
        flag_bool_a: FLAG_BOOL_A_ARG,
        flag_bool_b: FLAG_BOOL_B_ARG,
        flag_option_string: Some(FLAG_OPTION_STRING_ARG.to_owned()),
        flag_vec_single: vec![FLAG_VEC_SINGLE_ARG.to_owned()],
        flag_vec_multiple: vec![
            FLAG_VEC_MULTIPLE_ARG_A.to_owned(),
            FLAG_VEC_MULTIPLE_ARG_B.to_owned(),
            FLAG_VEC_MULTIPLE_ARG_C.to_owned(),
        ],
        flag_bytesize: ByteSize::from_str(FLAG_BYTESIZE_ARG).unwrap(),
        flag_enum: FlagEnum::ArgValue,
        flag_option_enum: Some(FlagEnum::ArgValue),
        positional_string: Some(POSITIONAL_STRING_ARG.to_owned()),
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
        flag_bool_a: FLAG_BOOL_A_CONFIG,
        flag_bool_b: FLAG_BOOL_B_CONFIG,
        flag_option_string: Some(FLAG_OPTION_STRING_CONFIG.to_owned()),
        flag_vec_single: vec![FLAG_VEC_SINGLE_CONFIG.to_owned()],
        flag_vec_multiple: vec![
            FLAG_VEC_MULTIPLE_CONFIG_A.to_owned(),
            FLAG_VEC_MULTIPLE_CONFIG_B.to_owned(),
            FLAG_VEC_MULTIPLE_CONFIG_C.to_owned(),
            FLAG_VEC_MULTIPLE_CONFIG_D.to_owned(),
        ],
        flag_bytesize: ByteSize::from_str(FLAG_BYTESIZE_CONFIG).unwrap(),
        flag_enum: FlagEnum::ConfigValue,
        flag_option_enum: Some(FlagEnum::ConfigValue),
        positional_string: None,
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
        flag_bool_a: FLAG_BOOL_A_ARG,
        flag_bool_b: FLAG_BOOL_B_ARG,
        flag_option_string: Some(FLAG_OPTION_STRING_ARG.to_owned()),
        flag_vec_single: vec![FLAG_VEC_SINGLE_ARG.to_owned()],
        flag_vec_multiple: vec![
            FLAG_VEC_MULTIPLE_ARG_A.to_owned(),
            FLAG_VEC_MULTIPLE_ARG_B.to_owned(),
            FLAG_VEC_MULTIPLE_ARG_C.to_owned(),
        ],
        flag_bytesize: ByteSize::from_str(FLAG_BYTESIZE_ARG).unwrap(),
        flag_enum: FlagEnum::ArgValue,
        flag_option_enum: Some(FlagEnum::ArgValue),
        positional_string: Some(POSITIONAL_STRING_ARG.to_owned()),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}
