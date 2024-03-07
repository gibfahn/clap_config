/*!
Tests that the macro handles a complex clap derived struct with subcommands.
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

// Toplevel

const TOPLEVEL_FLAG_STRING_DEFAULT: &str = "toplevel-flag-string-default";
const TOPLEVEL_FLAG_STRING_ARG: &str = "toplevel-flag-string-arg";
const TOPLEVEL_FLAG_STRING_CONFIG: &str = "toplevel-flag-string-config";

const TOPLEVEL_FLAG_BOOL_ONE_DEFAULT: bool = false;
const TOPLEVEL_FLAG_BOOL_ONE_ARG: bool = true;
const TOPLEVEL_FLAG_BOOL_ONE_CONFIG: bool = false;

const TOPLEVEL_FLAG_BOOL_TWO_DEFAULT: bool = false;
const TOPLEVEL_FLAG_BOOL_TWO_ARG: bool = false;
const TOPLEVEL_FLAG_BOOL_TWO_CONFIG: bool = true;

const TOPLEVEL_FLAG_OPTION_STRING_DEFAULT: Option<String> = None;
const TOPLEVEL_FLAG_OPTION_STRING_ARG: &str = "toplevel-flag-option-string-arg";
const TOPLEVEL_FLAG_OPTION_STRING_CONFIG: &str = "toplevel-flag-option-string-config";

const TOPLEVEL_FLAG_VEC_SINGLE_DEFAULT: Vec<String> = Vec::new();
const TOPLEVEL_FLAG_VEC_SINGLE_ARG: &str = "toplevel-flag-vec-single-arg";
const TOPLEVEL_FLAG_VEC_SINGLE_CONFIG: &str = "toplevel-flag-vec-single-config";

const TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_ONE: &str = "toplevel-flag-vec-multiple-default-one";
const TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_TWO: &str = "toplevel-flag-vec-multiple-default-two";
const TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE: &str = "toplevel-flag-vec-multiple-arg-one";
const TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO: &str = "toplevel-flag-vec-multiple-arg-two";
const TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE: &str = "toplevel-flag-vec-multiple-arg-three";
const TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_ONE: &str = "toplevel-flag-vec-multiple-config-one";
const TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_TWO: &str = "toplevel-flag-vec-multiple-config-two";
const TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_THREE: &str = "toplevel-flag-vec-multiple-config-three";
const TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_FOUR: &str = "toplevel-flag-vec-multiple-config-four";

const TOPLEVEL_FLAG_BYTESIZE_DEFAULT: &str = "200GB";
const TOPLEVEL_FLAG_BYTESIZE_ARG: &str = "100MB";
const TOPLEVEL_FLAG_BYTESIZE_CONFIG: &str = "20Gi";

const TOPLEVEL_FLAG_ENUM_DEFAULT: ToplevelFlagEnum = ToplevelFlagEnum::ToplevelDefaultValue;
const TOPLEVEL_FLAG_ENUM_ARG: &str = "toplevel-arg-value";
const TOPLEVEL_FLAG_ENUM_CONFIG: &str = "ToplevelConfigValue";

const TOPLEVEL_POSITIONAL_STRING_ARG: &str = "toplevel-positional-string-arg";

// Subcommand A

const SUBCOMMAND_A_FLAG_STRING_DEFAULT: &str = "subcommand-a-flag-string-default";
const SUBCOMMAND_A_FLAG_STRING_ARG: &str = "subcommand-a-flag-string-arg";
const SUBCOMMAND_A_FLAG_STRING_CONFIG: &str = "subcommand-a-flag-string-config";

const SUBCOMMAND_A_FLAG_BOOL_ONE_DEFAULT: bool = false;
const SUBCOMMAND_A_FLAG_BOOL_ONE_ARG: bool = true;
const SUBCOMMAND_A_FLAG_BOOL_ONE_CONFIG: bool = false;

const SUBCOMMAND_A_FLAG_BOOL_TWO_DEFAULT: bool = false;
const SUBCOMMAND_A_FLAG_BOOL_TWO_ARG: bool = false;
const SUBCOMMAND_A_FLAG_BOOL_TWO_CONFIG: bool = true;

const SUBCOMMAND_A_FLAG_OPTION_STRING_DEFAULT: Option<String> = None;
const SUBCOMMAND_A_FLAG_OPTION_STRING_ARG: &str = "subcommand-a-flag-option-string-arg";
const SUBCOMMAND_A_FLAG_OPTION_STRING_CONFIG: &str = "subcommand-a-flag-option-string-config";

const SUBCOMMAND_A_FLAG_VEC_SINGLE_DEFAULT: Vec<String> = Vec::new();
const SUBCOMMAND_A_FLAG_VEC_SINGLE_ARG: &str = "subcommand-a-flag-vec-single-arg";
const SUBCOMMAND_A_FLAG_VEC_SINGLE_CONFIG: &str = "subcommand-a-flag-vec-single-config";

const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_DEFAULT_ONE: &str =
    "subcommand-a-flag-vec-multiple-default-one";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_DEFAULT_TWO: &str =
    "subcommand-a-flag-vec-multiple-default-two";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_ONE: &str = "subcommand-a-flag-vec-multiple-arg-one";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_TWO: &str = "subcommand-a-flag-vec-multiple-arg-two";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_THREE: &str = "subcommand-a-flag-vec-multiple-arg-three";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_ONE: &str = "subcommand-a-flag-vec-multiple-config-one";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_TWO: &str = "subcommand-a-flag-vec-multiple-config-two";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_THREE: &str =
    "subcommand-a-flag-vec-multiple-config-three";
const SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_FOUR: &str =
    "subcommand-a-flag-vec-multiple-config-four";

const SUBCOMMAND_A_FLAG_BYTESIZE_DEFAULT: &str = "200GB";
const SUBCOMMAND_A_FLAG_BYTESIZE_ARG: &str = "100MB";
const SUBCOMMAND_A_FLAG_BYTESIZE_CONFIG: &str = "20Gi";

const SUBCOMMAND_A_FLAG_ENUM_DEFAULT: SubcommandAFlagEnum =
    SubcommandAFlagEnum::SubcommandADefaultValue;
const SUBCOMMAND_A_FLAG_ENUM_ARG: &str = "subcommand-a-arg-value";
const SUBCOMMAND_A_FLAG_ENUM_CONFIG: &str = "SubcommandAConfigValue";

const SUBCOMMAND_A_POSITIONAL_STRING_ARG: &str = "subcommand-a-positional-string-arg";

// Subcommand B

const SUBCOMMAND_B_FLAG_STRING_DEFAULT: &str = "subcommand-b-flag-string-default";
const SUBCOMMAND_B_FLAG_STRING_ARG: &str = "subcommand-b-flag-string-arg";
const SUBCOMMAND_B_FLAG_STRING_CONFIG: &str = "subcommand-b-flag-string-config";

const SUBCOMMAND_B_FLAG_BOOL_ONE_DEFAULT: bool = false;
const SUBCOMMAND_B_FLAG_BOOL_ONE_ARG: bool = true;
const SUBCOMMAND_B_FLAG_BOOL_ONE_CONFIG: bool = false;

const SUBCOMMAND_B_FLAG_BOOL_TWO_DEFAULT: bool = false;
const SUBCOMMAND_B_FLAG_BOOL_TWO_ARG: bool = false;
const SUBCOMMAND_B_FLAG_BOOL_TWO_CONFIG: bool = true;

const SUBCOMMAND_B_FLAG_OPTION_STRING_DEFAULT: Option<String> = None;
const SUBCOMMAND_B_FLAG_OPTION_STRING_ARG: &str = "subcommand-b-flag-option-string-arg";
const SUBCOMMAND_B_FLAG_OPTION_STRING_CONFIG: &str = "subcommand-b-flag-option-string-config";

const SUBCOMMAND_B_FLAG_VEC_SINGLE_DEFAULT: Vec<String> = Vec::new();
const SUBCOMMAND_B_FLAG_VEC_SINGLE_ARG: &str = "subcommand-b-flag-vec-single-arg";
const SUBCOMMAND_B_FLAG_VEC_SINGLE_CONFIG: &str = "subcommand-b-flag-vec-single-config";

const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_DEFAULT_ONE: &str =
    "subcommand-b-flag-vec-multiple-default-one";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_DEFAULT_TWO: &str =
    "subcommand-b-flag-vec-multiple-default-two";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_ONE: &str = "subcommand-b-flag-vec-multiple-arg-one";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_TWO: &str = "subcommand-b-flag-vec-multiple-arg-two";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_THREE: &str = "subcommand-b-flag-vec-multiple-arg-three";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_ONE: &str = "subcommand-b-flag-vec-multiple-config-one";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_TWO: &str = "subcommand-b-flag-vec-multiple-config-two";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_THREE: &str =
    "subcommand-b-flag-vec-multiple-config-three";
const SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_FOUR: &str =
    "subcommand-b-flag-vec-multiple-config-four";

const SUBCOMMAND_B_FLAG_BYTESIZE_DEFAULT: &str = "200GB";
const SUBCOMMAND_B_FLAG_BYTESIZE_ARG: &str = "100MB";
const SUBCOMMAND_B_FLAG_BYTESIZE_CONFIG: &str = "20Gi";

const SUBCOMMAND_B_FLAG_ENUM_DEFAULT: SubcommandBFlagEnum =
    SubcommandBFlagEnum::SubcommandBDefaultValue;
const SUBCOMMAND_B_FLAG_ENUM_ARG: &str = "subcommand-b-arg-value";
const SUBCOMMAND_B_FLAG_ENUM_CONFIG: &str = "SubcommandBConfigValue";

const SUBCOMMAND_B_POSITIONAL_STRING_ARG: &str = "subcommand-b-positional-string-arg";

// Subcommand A

const SUBCOMMAND_A_UNSET_ARGS: [&str; 2] = ["myapp", "subcommand-a"];
const SUBCOMMAND_A_SET_ARGS: [&str; 43] = [
    "myapp",
    "--toplevel-flag-string",
    TOPLEVEL_FLAG_STRING_ARG,
    "--toplevel-flag-bool-one",
    "--toplevel-flag-bool-two=false",
    "--toplevel-flag-option-string",
    TOPLEVEL_FLAG_OPTION_STRING_ARG,
    "--toplevel-flag-vec-single",
    TOPLEVEL_FLAG_VEC_SINGLE_ARG,
    "--toplevel-flag-vec-multiple",
    TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE,
    "--toplevel-flag-vec-multiple",
    TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO,
    "--toplevel-flag-vec-multiple",
    TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE,
    "--toplevel-flag-bytesize",
    TOPLEVEL_FLAG_BYTESIZE_ARG,
    "--toplevel-flag-enum",
    TOPLEVEL_FLAG_ENUM_ARG,
    "--toplevel-flag-option-enum",
    TOPLEVEL_FLAG_ENUM_ARG,
    "subcommand-a",
    "--subcommand-a-flag-string",
    SUBCOMMAND_A_FLAG_STRING_ARG,
    "--subcommand-a-flag-bool-one",
    "--subcommand-a-flag-bool-two=false",
    "--subcommand-a-flag-option-string",
    SUBCOMMAND_A_FLAG_OPTION_STRING_ARG,
    "--subcommand-a-flag-vec-single",
    SUBCOMMAND_A_FLAG_VEC_SINGLE_ARG,
    "--subcommand-a-flag-vec-multiple",
    SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_ONE,
    "--subcommand-a-flag-vec-multiple",
    SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_TWO,
    "--subcommand-a-flag-vec-multiple",
    SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_THREE,
    "--subcommand-a-flag-bytesize",
    SUBCOMMAND_A_FLAG_BYTESIZE_ARG,
    "--subcommand-a-flag-enum",
    SUBCOMMAND_A_FLAG_ENUM_ARG,
    "--subcommand-a-flag-option-enum",
    SUBCOMMAND_A_FLAG_ENUM_ARG,
    SUBCOMMAND_A_POSITIONAL_STRING_ARG,
];

// Subcommand B

const SUBCOMMAND_B_UNSET_ARGS: [&str; 2] = ["myapp", "subcommand-b"];
const SUBCOMMAND_B_SET_ARGS: [&str; 44] = [
    "myapp",
    "--toplevel-flag-string",
    TOPLEVEL_FLAG_STRING_ARG,
    "--toplevel-flag-bool-one",
    "--toplevel-flag-bool-two=false",
    "--toplevel-flag-option-string",
    TOPLEVEL_FLAG_OPTION_STRING_ARG,
    "--toplevel-flag-vec-single",
    TOPLEVEL_FLAG_VEC_SINGLE_ARG,
    "--toplevel-flag-vec-multiple",
    TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE,
    "--toplevel-flag-vec-multiple",
    TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO,
    "--toplevel-flag-vec-multiple",
    TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE,
    "--toplevel-flag-bytesize",
    TOPLEVEL_FLAG_BYTESIZE_ARG,
    "--toplevel-flag-enum",
    TOPLEVEL_FLAG_ENUM_ARG,
    "--toplevel-flag-option-enum",
    TOPLEVEL_FLAG_ENUM_ARG,
    TOPLEVEL_POSITIONAL_STRING_ARG,
    "subcommand-b",
    "--subcommand-b-flag-string",
    SUBCOMMAND_B_FLAG_STRING_ARG,
    "--subcommand-b-flag-bool-one",
    "--subcommand-b-flag-bool-two=false",
    "--subcommand-b-flag-option-string",
    SUBCOMMAND_B_FLAG_OPTION_STRING_ARG,
    "--subcommand-b-flag-vec-single",
    SUBCOMMAND_B_FLAG_VEC_SINGLE_ARG,
    "--subcommand-b-flag-vec-multiple",
    SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_ONE,
    "--subcommand-b-flag-vec-multiple",
    SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_TWO,
    "--subcommand-b-flag-vec-multiple",
    SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_THREE,
    "--subcommand-b-flag-bytesize",
    SUBCOMMAND_B_FLAG_BYTESIZE_ARG,
    "--subcommand-b-flag-enum",
    SUBCOMMAND_B_FLAG_ENUM_ARG,
    "--subcommand-b-flag-option-enum",
    SUBCOMMAND_B_FLAG_ENUM_ARG,
    SUBCOMMAND_B_POSITIONAL_STRING_ARG,
];

const UNSET_CONFIG: &str = "";
const SET_CONFIG: &str = formatcp!(
    "---
toplevel_flag_string: {TOPLEVEL_FLAG_STRING_CONFIG}

toplevel_flag_bool_one: {TOPLEVEL_FLAG_BOOL_ONE_CONFIG}

toplevel_flag_bool_two: {TOPLEVEL_FLAG_BOOL_TWO_CONFIG}

toplevel_flag_option_string: {TOPLEVEL_FLAG_OPTION_STRING_CONFIG}

toplevel_flag_vec_single: [{TOPLEVEL_FLAG_VEC_SINGLE_CONFIG}]

toplevel_flag_vec_multiple:
- {TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_ONE}
- {TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_TWO}
- {TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_THREE}
- {TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_FOUR}

toplevel_flag_bytesize: {TOPLEVEL_FLAG_BYTESIZE_CONFIG}

toplevel_flag_enum: {TOPLEVEL_FLAG_ENUM_CONFIG}

toplevel_flag_option_enum: {TOPLEVEL_FLAG_ENUM_CONFIG}

subcommand_a:

    subcommand_a_flag_string: {SUBCOMMAND_A_FLAG_STRING_CONFIG}

    subcommand_a_flag_bool_one: {SUBCOMMAND_A_FLAG_BOOL_ONE_CONFIG}

    subcommand_a_flag_bool_two: {SUBCOMMAND_A_FLAG_BOOL_TWO_CONFIG}

    subcommand_a_flag_option_string: {SUBCOMMAND_A_FLAG_OPTION_STRING_CONFIG}

    subcommand_a_flag_vec_single: [{SUBCOMMAND_A_FLAG_VEC_SINGLE_CONFIG}]

    subcommand_a_flag_vec_multiple:
    - {SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_ONE}
    - {SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_TWO}
    - {SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_THREE}
    - {SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_FOUR}

    subcommand_a_flag_bytesize: {SUBCOMMAND_A_FLAG_BYTESIZE_CONFIG}

    subcommand_a_flag_enum: {SUBCOMMAND_A_FLAG_ENUM_CONFIG}

    subcommand_a_flag_option_enum: {SUBCOMMAND_A_FLAG_ENUM_CONFIG}

subcommand_b:

    subcommand_b_flag_string: {SUBCOMMAND_B_FLAG_STRING_CONFIG}

    subcommand_b_flag_bool_one: {SUBCOMMAND_B_FLAG_BOOL_ONE_CONFIG}

    subcommand_b_flag_bool_two: {SUBCOMMAND_B_FLAG_BOOL_TWO_CONFIG}

    subcommand_b_flag_option_string: {SUBCOMMAND_B_FLAG_OPTION_STRING_CONFIG}

    subcommand_b_flag_vec_single: [{SUBCOMMAND_B_FLAG_VEC_SINGLE_CONFIG}]

    subcommand_b_flag_vec_multiple:
    - {SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_ONE}
    - {SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_TWO}
    - {SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_THREE}
    - {SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_FOUR}

    subcommand_b_flag_bytesize: {SUBCOMMAND_B_FLAG_BYTESIZE_CONFIG}

    subcommand_b_flag_enum: {SUBCOMMAND_B_FLAG_ENUM_CONFIG}

    subcommand_b_flag_option_enum: {SUBCOMMAND_B_FLAG_ENUM_CONFIG}

"
);

#[derive(ClapConfig, Parser, Debug, PartialEq)]
#[clap(version, subcommand_required = true)]
pub struct Opts {
    #[clap(long, default_value = TOPLEVEL_FLAG_STRING_DEFAULT)]
    toplevel_flag_string: String,

    #[clap(long)]
    toplevel_flag_bool_one: bool,

    #[clap(
        long,
        default_value_t = TOPLEVEL_FLAG_BOOL_TWO_DEFAULT,
        // If the user passes --toplevel-flag-bool-b with no argument, set this to true.
        default_missing_value = "true",
        require_equals = true,
        action = ArgAction::Set,
        num_args(0..=1),
    )]
    toplevel_flag_bool_two: bool,

    #[clap(long)]
    toplevel_flag_option_string: Option<String>,

    #[clap(long)]
    toplevel_flag_vec_single: Vec<String>,

    #[clap(long, default_values_t = vec![TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(), TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned()])]
    toplevel_flag_vec_multiple: Vec<String>,

    #[clap(long, default_value = TOPLEVEL_FLAG_BYTESIZE_DEFAULT)]
    toplevel_flag_bytesize: ByteSize,

    #[clap(value_enum, long, default_value_t)]
    toplevel_flag_enum: ToplevelFlagEnum,

    #[clap(value_enum, long)]
    toplevel_flag_option_enum: Option<ToplevelFlagEnum>,

    // No clap(long) here, this is a positional arg, which can't be set via config.
    toplevel_positional_string: Option<String>,

    #[clap(subcommand)]
    pub(crate) cmd: Option<SubCommand>,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub enum SubCommand {
    SubcommandA(SubcommandAOptions),
    SubcommandB(SubcommandBOptions),
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub struct SubcommandAOptions {
    #[clap(long, default_value = SUBCOMMAND_A_FLAG_STRING_DEFAULT)]
    subcommand_a_flag_string: String,

    #[clap(long)]
    subcommand_a_flag_bool_one: bool,

    #[clap(
        long,
        default_value_t = SUBCOMMAND_A_FLAG_BOOL_TWO_DEFAULT,
        // If the user passes --toplevel-flag-bool-b with no argument, set this to true.
        default_missing_value = "true",
        require_equals = true,
        action = ArgAction::Set,
        num_args(0..=1),
    )]
    subcommand_a_flag_bool_two: bool,

    #[clap(long)]
    subcommand_a_flag_option_string: Option<String>,

    #[clap(long)]
    subcommand_a_flag_vec_single: Vec<String>,

    #[clap(long, default_values_t = vec![SUBCOMMAND_A_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(), SUBCOMMAND_A_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned()])]
    subcommand_a_flag_vec_multiple: Vec<String>,

    #[clap(long, default_value = SUBCOMMAND_A_FLAG_BYTESIZE_DEFAULT)]
    subcommand_a_flag_bytesize: ByteSize,

    #[clap(value_enum, long, default_value_t)]
    subcommand_a_flag_enum: SubcommandAFlagEnum,

    #[clap(value_enum, long)]
    subcommand_a_flag_option_enum: Option<SubcommandAFlagEnum>,

    // No clap(long) here, this is a positional arg, which can't be set via config.
    subcommand_a_positional_string: Option<String>,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub struct SubcommandBOptions {
    #[clap(long, default_value = SUBCOMMAND_B_FLAG_STRING_DEFAULT)]
    subcommand_b_flag_string: String,

    #[clap(long)]
    subcommand_b_flag_bool_one: bool,

    #[clap(
        long,
        default_value_t = SUBCOMMAND_B_FLAG_BOOL_TWO_DEFAULT,
        // If the user passes --toplevel-flag-bool-b with no argument, set this to true.
        default_missing_value = "true",
        require_equals = true,
        action = ArgAction::Set,
        num_args(0..=1),
    )]
    subcommand_b_flag_bool_two: bool,

    #[clap(long)]
    subcommand_b_flag_option_string: Option<String>,

    #[clap(long)]
    subcommand_b_flag_vec_single: Vec<String>,

    #[clap(long, default_values_t = vec![SUBCOMMAND_B_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(), SUBCOMMAND_B_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned()])]
    subcommand_b_flag_vec_multiple: Vec<String>,

    #[clap(long, default_value = SUBCOMMAND_B_FLAG_BYTESIZE_DEFAULT)]
    subcommand_b_flag_bytesize: ByteSize,

    #[clap(value_enum, long, default_value_t)]
    subcommand_b_flag_enum: SubcommandBFlagEnum,

    #[clap(value_enum, long)]
    subcommand_b_flag_option_enum: Option<SubcommandBFlagEnum>,

    // No clap(long) here, this is a positional arg, which can't be set via config.
    subcommand_b_positional_string: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Default, Serialize, Deserialize, PartialEq)]
pub enum ToplevelFlagEnum {
    #[default]
    ToplevelDefaultValue,
    ToplevelArgValue,
    ToplevelConfigValue,
}

#[derive(Debug, Clone, ValueEnum, Default, Serialize, Deserialize, PartialEq)]
pub enum SubcommandAFlagEnum {
    #[default]
    SubcommandADefaultValue,
    SubcommandAArgValue,
    SubcommandAConfigValue,
}

#[derive(Debug, Clone, ValueEnum, Default, Serialize, Deserialize, PartialEq)]
pub enum SubcommandBFlagEnum {
    #[default]
    SubcommandBDefaultValue,
    SubcommandBArgValue,
    SubcommandBConfigValue,
}

/// Nothing set anywhere.
#[test]
fn test_nothing_set_subcommand_a() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_A_UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_DEFAULT.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_DEFAULT,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_DEFAULT,
        toplevel_flag_option_string: TOPLEVEL_FLAG_OPTION_STRING_DEFAULT,
        toplevel_flag_vec_single: TOPLEVEL_FLAG_VEC_SINGLE_DEFAULT,
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_DEFAULT).unwrap(),
        toplevel_flag_enum: TOPLEVEL_FLAG_ENUM_DEFAULT,
        toplevel_flag_option_enum: None,
        toplevel_positional_string: None,

        cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
            subcommand_a_flag_string: SUBCOMMAND_A_FLAG_STRING_DEFAULT.to_owned(),
            subcommand_a_flag_bool_one: SUBCOMMAND_A_FLAG_BOOL_ONE_DEFAULT,
            subcommand_a_flag_bool_two: SUBCOMMAND_A_FLAG_BOOL_TWO_DEFAULT,
            subcommand_a_flag_option_string: SUBCOMMAND_A_FLAG_OPTION_STRING_DEFAULT,
            subcommand_a_flag_vec_single: SUBCOMMAND_A_FLAG_VEC_SINGLE_DEFAULT,
            subcommand_a_flag_vec_multiple: vec![
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned(),
            ],
            subcommand_a_flag_bytesize: ByteSize::from_str(SUBCOMMAND_A_FLAG_BYTESIZE_DEFAULT)
                .unwrap(),
            subcommand_a_flag_enum: SUBCOMMAND_A_FLAG_ENUM_DEFAULT,
            subcommand_a_flag_option_enum: None,
            subcommand_a_positional_string: None,
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// Nothing set anywhere.
#[test]
fn test_nothing_set_subcommand_b() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_B_UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_DEFAULT.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_DEFAULT,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_DEFAULT,
        toplevel_flag_option_string: TOPLEVEL_FLAG_OPTION_STRING_DEFAULT,
        toplevel_flag_vec_single: TOPLEVEL_FLAG_VEC_SINGLE_DEFAULT,
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_DEFAULT).unwrap(),
        toplevel_flag_enum: TOPLEVEL_FLAG_ENUM_DEFAULT,
        toplevel_flag_option_enum: None,
        toplevel_positional_string: None,

        cmd: Some(SubCommand::SubcommandB(SubcommandBOptions {
            subcommand_b_flag_string: SUBCOMMAND_B_FLAG_STRING_DEFAULT.to_owned(),
            subcommand_b_flag_bool_one: SUBCOMMAND_B_FLAG_BOOL_ONE_DEFAULT,
            subcommand_b_flag_bool_two: SUBCOMMAND_B_FLAG_BOOL_TWO_DEFAULT,
            subcommand_b_flag_option_string: SUBCOMMAND_B_FLAG_OPTION_STRING_DEFAULT,
            subcommand_b_flag_vec_single: SUBCOMMAND_B_FLAG_VEC_SINGLE_DEFAULT,
            subcommand_b_flag_vec_multiple: vec![
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_DEFAULT_ONE.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_DEFAULT_TWO.to_owned(),
            ],
            subcommand_b_flag_bytesize: ByteSize::from_str(SUBCOMMAND_B_FLAG_BYTESIZE_DEFAULT)
                .unwrap(),
            subcommand_b_flag_enum: SUBCOMMAND_B_FLAG_ENUM_DEFAULT,
            subcommand_b_flag_option_enum: None,
            subcommand_b_positional_string: None,
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in args, none in config.
#[test]
fn test_args_set_subcommand_a() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_A_SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_ARG.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_ARG,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_ARG,
        toplevel_flag_option_string: Some(TOPLEVEL_FLAG_OPTION_STRING_ARG.to_owned()),
        toplevel_flag_vec_single: vec![TOPLEVEL_FLAG_VEC_SINGLE_ARG.to_owned()],
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_ARG).unwrap(),
        toplevel_flag_enum: ToplevelFlagEnum::ToplevelArgValue,
        toplevel_flag_option_enum: Some(ToplevelFlagEnum::ToplevelArgValue),
        toplevel_positional_string: None,

        cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
            subcommand_a_flag_string: SUBCOMMAND_A_FLAG_STRING_ARG.to_owned(),
            subcommand_a_flag_bool_one: SUBCOMMAND_A_FLAG_BOOL_ONE_ARG,
            subcommand_a_flag_bool_two: SUBCOMMAND_A_FLAG_BOOL_TWO_ARG,
            subcommand_a_flag_option_string: Some(SUBCOMMAND_A_FLAG_OPTION_STRING_ARG.to_owned()),
            subcommand_a_flag_vec_single: vec![SUBCOMMAND_A_FLAG_VEC_SINGLE_ARG.to_owned()],
            subcommand_a_flag_vec_multiple: vec![
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
            ],
            subcommand_a_flag_bytesize: ByteSize::from_str(SUBCOMMAND_A_FLAG_BYTESIZE_ARG).unwrap(),
            subcommand_a_flag_enum: SubcommandAFlagEnum::SubcommandAArgValue,
            subcommand_a_flag_option_enum: Some(SubcommandAFlagEnum::SubcommandAArgValue),
            subcommand_a_positional_string: Some(SUBCOMMAND_A_POSITIONAL_STRING_ARG.to_owned()),
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in args, none in config.
#[test]
fn test_args_set_subcommand_b() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_B_SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_ARG.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_ARG,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_ARG,
        toplevel_flag_option_string: Some(TOPLEVEL_FLAG_OPTION_STRING_ARG.to_owned()),
        toplevel_flag_vec_single: vec![TOPLEVEL_FLAG_VEC_SINGLE_ARG.to_owned()],
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_ARG).unwrap(),
        toplevel_flag_enum: ToplevelFlagEnum::ToplevelArgValue,
        toplevel_flag_option_enum: Some(ToplevelFlagEnum::ToplevelArgValue),
        toplevel_positional_string: Some(TOPLEVEL_POSITIONAL_STRING_ARG.to_owned()),

        cmd: Some(SubCommand::SubcommandB(SubcommandBOptions {
            subcommand_b_flag_string: SUBCOMMAND_B_FLAG_STRING_ARG.to_owned(),
            subcommand_b_flag_bool_one: SUBCOMMAND_B_FLAG_BOOL_ONE_ARG,
            subcommand_b_flag_bool_two: SUBCOMMAND_B_FLAG_BOOL_TWO_ARG,
            subcommand_b_flag_option_string: Some(SUBCOMMAND_B_FLAG_OPTION_STRING_ARG.to_owned()),
            subcommand_b_flag_vec_single: vec![SUBCOMMAND_B_FLAG_VEC_SINGLE_ARG.to_owned()],
            subcommand_b_flag_vec_multiple: vec![
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
            ],
            subcommand_b_flag_bytesize: ByteSize::from_str(SUBCOMMAND_B_FLAG_BYTESIZE_ARG).unwrap(),
            subcommand_b_flag_enum: SubcommandBFlagEnum::SubcommandBArgValue,
            subcommand_b_flag_option_enum: Some(SubcommandBFlagEnum::SubcommandBArgValue),
            subcommand_b_positional_string: Some(SUBCOMMAND_B_POSITIONAL_STRING_ARG.to_owned()),
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in config, none in args.
#[test]
fn test_config_set_subcommand_a() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_A_UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_CONFIG.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_CONFIG,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_CONFIG,
        toplevel_flag_option_string: Some(TOPLEVEL_FLAG_OPTION_STRING_CONFIG.to_owned()),
        toplevel_flag_vec_single: vec![TOPLEVEL_FLAG_VEC_SINGLE_CONFIG.to_owned()],
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_TWO.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_THREE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_FOUR.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_CONFIG).unwrap(),
        toplevel_flag_enum: ToplevelFlagEnum::ToplevelConfigValue,
        toplevel_flag_option_enum: Some(ToplevelFlagEnum::ToplevelConfigValue),
        toplevel_positional_string: None,

        cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
            subcommand_a_flag_string: SUBCOMMAND_A_FLAG_STRING_CONFIG.to_owned(),
            subcommand_a_flag_bool_one: SUBCOMMAND_A_FLAG_BOOL_ONE_CONFIG,
            subcommand_a_flag_bool_two: SUBCOMMAND_A_FLAG_BOOL_TWO_CONFIG,
            subcommand_a_flag_option_string: Some(
                SUBCOMMAND_A_FLAG_OPTION_STRING_CONFIG.to_owned(),
            ),
            subcommand_a_flag_vec_single: vec![SUBCOMMAND_A_FLAG_VEC_SINGLE_CONFIG.to_owned()],
            subcommand_a_flag_vec_multiple: vec![
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_ONE.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_TWO.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_THREE.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_CONFIG_FOUR.to_owned(),
            ],
            subcommand_a_flag_bytesize: ByteSize::from_str(SUBCOMMAND_A_FLAG_BYTESIZE_CONFIG)
                .unwrap(),
            subcommand_a_flag_enum: SubcommandAFlagEnum::SubcommandAConfigValue,
            subcommand_a_flag_option_enum: Some(SubcommandAFlagEnum::SubcommandAConfigValue),
            subcommand_a_positional_string: None,
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in config, none in args.
#[test]
fn test_config_set_subcommand_b() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_B_UNSET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_CONFIG.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_CONFIG,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_CONFIG,
        toplevel_flag_option_string: Some(TOPLEVEL_FLAG_OPTION_STRING_CONFIG.to_owned()),
        toplevel_flag_vec_single: vec![TOPLEVEL_FLAG_VEC_SINGLE_CONFIG.to_owned()],
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_TWO.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_THREE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_CONFIG_FOUR.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_CONFIG).unwrap(),
        toplevel_flag_enum: ToplevelFlagEnum::ToplevelConfigValue,
        toplevel_flag_option_enum: Some(ToplevelFlagEnum::ToplevelConfigValue),
        toplevel_positional_string: None,

        cmd: Some(SubCommand::SubcommandB(SubcommandBOptions {
            subcommand_b_flag_string: SUBCOMMAND_B_FLAG_STRING_CONFIG.to_owned(),
            subcommand_b_flag_bool_one: SUBCOMMAND_B_FLAG_BOOL_ONE_CONFIG,
            subcommand_b_flag_bool_two: SUBCOMMAND_B_FLAG_BOOL_TWO_CONFIG,
            subcommand_b_flag_option_string: Some(
                SUBCOMMAND_B_FLAG_OPTION_STRING_CONFIG.to_owned(),
            ),
            subcommand_b_flag_vec_single: vec![SUBCOMMAND_B_FLAG_VEC_SINGLE_CONFIG.to_owned()],
            subcommand_b_flag_vec_multiple: vec![
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_ONE.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_TWO.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_THREE.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_CONFIG_FOUR.to_owned(),
            ],
            subcommand_b_flag_bytesize: ByteSize::from_str(SUBCOMMAND_B_FLAG_BYTESIZE_CONFIG)
                .unwrap(),
            subcommand_b_flag_enum: SubcommandBFlagEnum::SubcommandBConfigValue,
            subcommand_b_flag_option_enum: Some(SubcommandBFlagEnum::SubcommandBConfigValue),
            subcommand_b_positional_string: None,
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in both.
#[test]
fn test_both_set_subcommand_a() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_A_SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_ARG.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_ARG,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_ARG,
        toplevel_flag_option_string: Some(TOPLEVEL_FLAG_OPTION_STRING_ARG.to_owned()),
        toplevel_flag_vec_single: vec![TOPLEVEL_FLAG_VEC_SINGLE_ARG.to_owned()],
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_ARG).unwrap(),
        toplevel_flag_enum: ToplevelFlagEnum::ToplevelArgValue,
        toplevel_flag_option_enum: Some(ToplevelFlagEnum::ToplevelArgValue),
        toplevel_positional_string: None,

        cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
            subcommand_a_flag_string: SUBCOMMAND_A_FLAG_STRING_ARG.to_owned(),
            subcommand_a_flag_bool_one: SUBCOMMAND_A_FLAG_BOOL_ONE_ARG,
            subcommand_a_flag_bool_two: SUBCOMMAND_A_FLAG_BOOL_TWO_ARG,
            subcommand_a_flag_option_string: Some(SUBCOMMAND_A_FLAG_OPTION_STRING_ARG.to_owned()),
            subcommand_a_flag_vec_single: vec![SUBCOMMAND_A_FLAG_VEC_SINGLE_ARG.to_owned()],
            subcommand_a_flag_vec_multiple: vec![
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
                SUBCOMMAND_A_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
            ],
            subcommand_a_flag_bytesize: ByteSize::from_str(SUBCOMMAND_A_FLAG_BYTESIZE_ARG).unwrap(),
            subcommand_a_flag_enum: SubcommandAFlagEnum::SubcommandAArgValue,
            subcommand_a_flag_option_enum: Some(SubcommandAFlagEnum::SubcommandAArgValue),
            subcommand_a_positional_string: Some(SUBCOMMAND_A_POSITIONAL_STRING_ARG.to_owned()),
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}

/// All values set in both.
#[test]
fn test_both_set_subcommand_b() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(SUBCOMMAND_B_SET_ARGS);
    let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
    let opts = Opts::from_merged(matches, config);

    let expected_opts = Opts {
        toplevel_flag_string: TOPLEVEL_FLAG_STRING_ARG.to_owned(),
        toplevel_flag_bool_one: TOPLEVEL_FLAG_BOOL_ONE_ARG,
        toplevel_flag_bool_two: TOPLEVEL_FLAG_BOOL_TWO_ARG,
        toplevel_flag_option_string: Some(TOPLEVEL_FLAG_OPTION_STRING_ARG.to_owned()),
        toplevel_flag_vec_single: vec![TOPLEVEL_FLAG_VEC_SINGLE_ARG.to_owned()],
        toplevel_flag_vec_multiple: vec![
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
            TOPLEVEL_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
        ],
        toplevel_flag_bytesize: ByteSize::from_str(TOPLEVEL_FLAG_BYTESIZE_ARG).unwrap(),
        toplevel_flag_enum: ToplevelFlagEnum::ToplevelArgValue,
        toplevel_flag_option_enum: Some(ToplevelFlagEnum::ToplevelArgValue),
        toplevel_positional_string: Some(TOPLEVEL_POSITIONAL_STRING_ARG.to_owned()),

        cmd: Some(SubCommand::SubcommandB(SubcommandBOptions {
            subcommand_b_flag_string: SUBCOMMAND_B_FLAG_STRING_ARG.to_owned(),
            subcommand_b_flag_bool_one: SUBCOMMAND_B_FLAG_BOOL_ONE_ARG,
            subcommand_b_flag_bool_two: SUBCOMMAND_B_FLAG_BOOL_TWO_ARG,
            subcommand_b_flag_option_string: Some(SUBCOMMAND_B_FLAG_OPTION_STRING_ARG.to_owned()),
            subcommand_b_flag_vec_single: vec![SUBCOMMAND_B_FLAG_VEC_SINGLE_ARG.to_owned()],
            subcommand_b_flag_vec_multiple: vec![
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_ONE.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_TWO.to_owned(),
                SUBCOMMAND_B_FLAG_VEC_MULTIPLE_ARG_THREE.to_owned(),
            ],
            subcommand_b_flag_bytesize: ByteSize::from_str(SUBCOMMAND_B_FLAG_BYTESIZE_ARG).unwrap(),
            subcommand_b_flag_enum: SubcommandBFlagEnum::SubcommandBArgValue,
            subcommand_b_flag_option_enum: Some(SubcommandBFlagEnum::SubcommandBArgValue),
            subcommand_b_positional_string: Some(SUBCOMMAND_B_POSITIONAL_STRING_ARG.to_owned()),
        })),
    };

    assert_eq!(expected_opts, opts);
    Ok(())
}
