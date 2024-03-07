//! Tests that the macro produces usable output for a very simple clap derived struct.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;

const FROM_DEFAULT: &str = "from-default";
const FROM_ARG: &str = "from-arg";
const FROM_CONFIG: &str = "from-config";

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag: String,
}

#[test]
fn main() -> Result<()> {
    let unset_args = ["myapp"];
    let set_args = ["myapp", "--flag", FROM_ARG];

    let unset_config = "";
    let set_config = &format!("flag: {FROM_CONFIG}");

    // Not set anywhere, use default value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(unset_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(FROM_DEFAULT, opts.flag);
    }

    // Set in args, use that value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(set_args);
        let config: OptsConfig = serde_yaml::from_str(unset_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(FROM_ARG, opts.flag);
    }

    // Set in config, use that value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(unset_args);
        let config: OptsConfig = serde_yaml::from_str(set_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(FROM_CONFIG, opts.flag);
    }

    // Set in both, use args value.
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(set_args);
        let config: OptsConfig = serde_yaml::from_str(set_config)?;
        let opts = Opts::from_merged(matches, Some(config));
        assert_eq!(FROM_ARG, opts.flag);
    }

    Ok(())
}
