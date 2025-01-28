//! Tests that we can parse a value skipped in clap, so only provided by `ClapConfig`.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;
use std::collections::HashMap;

const FROM_DEFAULT: &str = "from-default";

#[derive(ClapConfig, Parser, Debug, PartialEq)]
pub struct Opts {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag: String,

    #[clap(skip, value_parser(clap::value_parser!(String)))]
    config_only: HashMap<String, HashMap<String, bool>>,
}

#[test]
fn main() -> Result<()> {
    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp"]);
        let config: OptsConfig = serde_yaml::from_str("")?;
        let opts = Opts::from_merged(matches, Some(config));

        let expected = Opts {
            flag: FROM_DEFAULT.to_owned(),
            config_only: HashMap::new(),
        };
        assert_eq!(expected, opts);
    }

    {
        let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp"]);
        let config: OptsConfig = serde_yaml::from_str(
            "---
config_only:
  key: { 'subkey': true }",
        )?;
        let opts = Opts::from_merged(matches, Some(config));

        let mut expected = Opts {
            flag: FROM_DEFAULT.to_owned(),
            config_only: HashMap::new(),
        };
        expected.config_only.insert(
            "key".to_owned(),
            HashMap::from_iter([("subkey".to_owned(), true)]),
        );
        assert_eq!(expected, opts);
    }

    Ok(())
}
