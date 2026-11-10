//! Tests that `#[clap(skip)]` fields can only be set from the config file.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

const FROM_DEFAULT: &str = "from-default";
const FROM_SKIP_DEFAULT: &str = "from-skip-default";
const FROM_CONFIG: &str = "from-config";

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag: String,

    /// Config-only field with a `Default` value.
    #[clap(skip)]
    config_only: String,

    /// Config-only field with a default from the `skip` attribute.
    #[arg(skip = FROM_SKIP_DEFAULT.to_owned())]
    config_only_skip_default: String,

    /// Config-only optional field.
    #[clap(skip)]
    config_only_optional: Option<String>,

    /// Config-only field that contains configs of the same type.
    #[clap(skip)]
    nested: BTreeMap<String, OptsConfig>,

    /// Skipped by both clap and `clap_config`, so always the default.
    #[clap(skip)]
    #[clap_config(skip)]
    fully_skipped: Vec<String>,
}

/// Not set in config, use defaults.
#[test]
fn unset() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp"]);
    let config: OptsConfig = serde_yaml::from_str("")?;
    let opts = Opts::from_merged(matches, Some(config));

    assert_eq!(FROM_DEFAULT, opts.flag);
    assert_eq!(String::new(), opts.config_only);
    assert_eq!(FROM_SKIP_DEFAULT, opts.config_only_skip_default);
    assert_eq!(None, opts.config_only_optional);
    assert!(opts.nested.is_empty());
    assert_eq!(Vec::<String>::new(), opts.fully_skipped);
    Ok(())
}

/// No config at all, use defaults.
#[test]
fn no_config() {
    let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp"]);
    let opts = Opts::from_merged(matches, None);
    assert_eq!(String::new(), opts.config_only);
    assert_eq!(FROM_SKIP_DEFAULT, opts.config_only_skip_default);
}

/// Set in config, use config.
#[test]
fn set_config() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp"]);
    let config: OptsConfig = serde_yaml::from_str(&format!(
        "
config_only: {FROM_CONFIG}
config_only_skip_default: {FROM_CONFIG}
config_only_optional: {FROM_CONFIG}
nested:
  some_key:
    flag: {FROM_CONFIG}
"
    ))?;
    let opts = Opts::from_merged(matches, Some(config));

    assert_eq!(FROM_DEFAULT, opts.flag);
    assert_eq!(FROM_CONFIG, opts.config_only);
    assert_eq!(FROM_CONFIG, opts.config_only_skip_default);
    assert_eq!(Some(FROM_CONFIG.to_owned()), opts.config_only_optional);
    assert_eq!(
        Some(FROM_CONFIG),
        opts.nested.get("some_key").and_then(|c| c.flag.as_deref())
    );
    Ok(())
}

/// Fields skipped by both clap and `clap_config` aren't in the config struct.
#[test]
fn fully_skipped_not_in_config() -> Result<()> {
    let config: OptsConfig = serde_yaml::from_str("fully_skipped: [a]")?;
    let value = serde_json::to_value(&config)?;
    assert_eq!(serde_json::json!({}), value);
    Ok(())
}
