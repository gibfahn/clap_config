//! Tests that we can parse nested subcommand enums, including those with unit variants.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;
use const_format::formatcp;

const FROM_DEFAULT_A: &str = "from-default-a";
const FROM_ARG_A: &str = "from-arg-a";
const FROM_CONFIG_A: &str = "from-config-a";

const FROM_DEFAULT_C: &str = "from-default-c";
const FROM_CONFIG_C: &str = "from-config-c";

const FROM_DEFAULT_X: &str = "from-default-x";
const FROM_ARG_X: &str = "from-arg-x";
const FROM_CONFIG_X: &str = "from-config-x";

#[derive(ClapConfig, Parser, Debug, PartialEq)]
pub struct Opts {
    #[clap(subcommand)]
    pub(crate) cmd: Option<SubCommand>,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub enum SubCommand {
    SubcommandA(SubcommandAOptions),
    SubcommandB,
    SubcommandC(SubcommandCOptions),
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub struct SubcommandAOptions {
    #[clap(long, default_value = FROM_DEFAULT_A)]
    flag_a: String,
    #[clap(subcommand)]
    pub(crate) subcommand_a_cmd: Option<SubCommandACmd>,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub enum SubCommandACmd {
    SubcommandX(SubcommandXOptions),
    SubcommandY,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub struct SubcommandXOptions {
    #[clap(long, default_value = FROM_DEFAULT_X)]
    flag_x: String,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub struct SubcommandCOptions {
    #[clap(long, default_value = FROM_DEFAULT_C)]
    flag_c: String,
    #[clap(subcommand)]
    pub(crate) subcommand_c_cmd: SubCommandCCmd,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub enum SubCommandCCmd {
    SubcommandZ,
}

const NO_ARGS: [&str; 1] = ["myapp"];
const UNSET_ARGS_A: [&str; 2] = ["myapp", "subcommand-a"];
const SET_ARGS_A: [&str; 4] = ["myapp", "subcommand-a", "--flag-a", FROM_ARG_A];
const SET_ARGS_B: [&str; 2] = ["myapp", "subcommand-b"];
const UNSET_ARGS_X: [&str; 3] = ["myapp", "subcommand-a", "subcommand-x"];
const UNSET_ARGS_Y: [&str; 3] = ["myapp", "subcommand-a", "subcommand-y"];
const SET_ARGS_Z: [&str; 3] = ["myapp", "subcommand-c", "subcommand-z"];
const SET_ARGS_X: [&str; 7] = [
    "myapp",
    "subcommand-a",
    "--flag-a",
    FROM_ARG_A,
    "subcommand-x",
    "--flag-x",
    FROM_ARG_X,
];

const UNSET_CONFIG: &str = "";
const SET_CONFIG: &str = formatcp!(
    "
subcommand_a:
    flag_a: {FROM_CONFIG_A}
    subcommand_x:
        flag_x: {FROM_CONFIG_X}
subcommand_c:
    flag_c: {FROM_CONFIG_C}
"
);

#[test]
fn no_subcommand() -> Result<()> {
    // No subcommand passed, `opts.cmd` should be unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(NO_ARGS);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts { cmd: None };
        assert_eq!(expected, opts);
    }

    Ok(())
}

#[test]
fn empty_subcommand() -> Result<()> {
    // Subcommand passed, no config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS_A);
        let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_DEFAULT_A.to_owned(),
                subcommand_a_cmd: None,
            })),
        };
        assert_eq!(expected, opts);
    }

    // No subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS_A);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_CONFIG_A.to_owned(),
                subcommand_a_cmd: None,
            })),
        };
        assert_eq!(expected, opts);
    }

    Ok(())
}

#[test]
fn set_subcommand() -> Result<()> {
    // Subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_A);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_ARG_A.to_owned(),
                subcommand_a_cmd: None,
            })),
        };
        assert_eq!(expected, opts);
    }

    // Nested subcommand passed, config unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS_X);
        let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_DEFAULT_A.to_owned(),
                subcommand_a_cmd: Some(SubCommandACmd::SubcommandX(SubcommandXOptions {
                    flag_x: FROM_DEFAULT_X.to_owned(),
                })),
            })),
        };
        assert_eq!(expected, opts);
    }

    Ok(())
}

#[test]
fn nested_subcommand() -> Result<()> {
    // Nested unset subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS_X);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_CONFIG_A.to_owned(),
                subcommand_a_cmd: Some(SubCommandACmd::SubcommandX(SubcommandXOptions {
                    flag_x: FROM_CONFIG_X.to_owned(),
                })),
            })),
        };
        assert_eq!(expected, opts);
    }

    // Nested set subcommand passed, config unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_X);
        let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_ARG_A.to_owned(),
                subcommand_a_cmd: Some(SubCommandACmd::SubcommandX(SubcommandXOptions {
                    flag_x: FROM_ARG_X.to_owned(),
                })),
            })),
        };
        assert_eq!(expected, opts);
    }

    // Nested set subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_X);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_ARG_A.to_owned(),
                subcommand_a_cmd: Some(SubCommandACmd::SubcommandX(SubcommandXOptions {
                    flag_x: FROM_ARG_X.to_owned(),
                })),
            })),
        };
        assert_eq!(expected, opts);
    }

    Ok(())
}

#[test]
fn unit_subcommand() -> Result<()> {
    // Unit subcommand passed, config unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_B);
        let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandB),
        };
        assert_eq!(expected, opts);
    }

    // Unit subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_B);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandB),
        };
        assert_eq!(expected, opts);
    }

    Ok(())
}

#[test]
fn nested_unit_subcommand() -> Result<()> {
    // Nested unit subcommand passed, config unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS_Y);
        let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_DEFAULT_A.to_owned(),
                subcommand_a_cmd: Some(SubCommandACmd::SubcommandY),
            })),
        };
        assert_eq!(expected, opts);
    }

    // Nested unit subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(UNSET_ARGS_Y);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
                flag_a: FROM_CONFIG_A.to_owned(),
                subcommand_a_cmd: Some(SubCommandACmd::SubcommandY),
            })),
        };
        assert_eq!(expected, opts);
    }

    Ok(())
}

#[test]
fn nested_unit_nonoptional_subcommand() -> Result<()> {
    // Nested unit subcommand passed, config unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_Z);
        let config: OptsConfig = serde_yaml::from_str(UNSET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandC(SubcommandCOptions {
                flag_c: FROM_DEFAULT_C.to_owned(),
                subcommand_c_cmd: SubCommandCCmd::SubcommandZ,
            })),
        };
        assert_eq!(expected, opts);
    }

    // Nested unit subcommand passed, config set.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(SET_ARGS_Z);
        let config: OptsConfig = serde_yaml::from_str(SET_CONFIG)?;
        let opts = Opts::from_merged(matches, Some(config));
        let expected = Opts {
            cmd: Some(SubCommand::SubcommandC(SubcommandCOptions {
                flag_c: FROM_CONFIG_C.to_owned(),
                subcommand_c_cmd: SubCommandCCmd::SubcommandZ,
            })),
        };
        assert_eq!(expected, opts);
    }

    Ok(())
}
