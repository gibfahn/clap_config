//! Tests that the macro produces usable output for a very simple clap derived struct with
//! a subcommand.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::eyre::bail;
use color_eyre::Result;

const FROM_DEFAULT: &str = "from-default";
const FROM_ARG: &str = "from-arg";
const FROM_CONFIG: &str = "from-config";

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
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
    #[clap(long, default_value = FROM_DEFAULT)]
    flag_a: String,
}

#[derive(ClapConfig, Debug, Parser, Clone, PartialEq)]
pub struct SubcommandBOptions {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag_b: String,
}

#[test]
fn main() -> Result<()> {
    let no_subcommand_args = ["myapp"];
    let unset_args = ["myapp", "subcommand-a"];
    let set_args = ["myapp", "subcommand-a", "--flag-a", FROM_ARG];

    let unset_config = "";
    let set_config = &format!(
        "
subcommand_a:
    flag_a: {FROM_CONFIG}
"
    );

    // No subcommand passed, `opts.cmd` should be unset.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(no_subcommand_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(None, opts.cmd);
    }

    // Not set anywhere, use default value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(unset_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        let flag_a = match opts.cmd.unwrap() {
            SubCommand::SubcommandA(SubcommandAOptions { flag_a }) => flag_a,
            subcmd => bail!("Expected subcommand A, found: {subcmd:?}"),
        };
        assert_eq!(FROM_DEFAULT, flag_a);
    }

    // Set in args, use that value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(set_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        let flag_a = match opts.cmd.unwrap() {
            SubCommand::SubcommandA(SubcommandAOptions { flag_a }) => flag_a,
            subcmd => bail!("Expected subcommand A, found: {subcmd:?}"),
        };
        assert_eq!(FROM_ARG, flag_a);
    }

    // Set in config, use that value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(unset_args);
        let config: OptsConfig = serde_yaml::from_str(set_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        let flag_a = match opts.cmd.unwrap() {
            SubCommand::SubcommandA(SubcommandAOptions { flag_a }) => flag_a,
            subcmd => bail!("Expected subcommand A, found: {subcmd:?}"),
        };
        assert_eq!(FROM_CONFIG, flag_a);
    }

    // Set in both, use args value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(set_args);
        let config: OptsConfig = serde_yaml::from_str(set_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        let flag_a = match opts.cmd.unwrap() {
            SubCommand::SubcommandA(SubcommandAOptions { flag_a }) => flag_a,
            subcmd => bail!("Expected subcommand A, found: {subcmd:?}"),
        };
        assert_eq!(FROM_ARG, flag_a);
    }

    Ok(())
}
