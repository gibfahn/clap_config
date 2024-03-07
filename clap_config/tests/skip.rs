//! Tests that we can skip non-serializable fields.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;
use std::fmt::Display;
use std::str::FromStr;

const FROM_DEFAULT: &str = "from-default";

#[derive(Debug, Clone, PartialEq)]
struct NonSerializable(bool);

impl Display for NonSerializable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for NonSerializable {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let inner: bool = s.parse()?;
        Ok(Self(inner))
    }
}

#[derive(ClapConfig, Parser, Debug, PartialEq)]
pub struct Opts {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag: String,

    #[clap_config(skip)]
    #[clap(long, default_value_t = NonSerializable(false))]
    ignore_me_flag: NonSerializable,
}

#[test]
fn main() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp"]);
    let config: OptsConfig = serde_yaml::from_str("")?;
    let opts = Opts::from_merged(matches, Some(config));

    let expected = Opts {
        flag: FROM_DEFAULT.to_owned(),
        ignore_me_flag: NonSerializable(false),
    };
    assert_eq!(expected, opts);

    Ok(())
}
