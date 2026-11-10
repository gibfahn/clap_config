//! Tests that we can merge two configs together, and layer them under the CLI args.

use clap::CommandFactory;
use clap::Parser;
use clap_config::ClapConfig;
use color_eyre::Result;
use pretty_assertions::assert_eq;

const FROM_DEFAULT: &str = "from-default";
const FROM_ARG: &str = "from-arg";
const FROM_PREFERRED: &str = "from-preferred";
const FROM_FALLBACK: &str = "from-fallback";

#[derive(ClapConfig, Parser, Debug, PartialEq)]
pub struct Opts {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag_a: String,

    #[clap(long, default_value = FROM_DEFAULT)]
    flag_b: String,

    #[clap(long, default_value = FROM_DEFAULT)]
    flag_c: String,

    #[clap(long)]
    list: Vec<String>,

    #[clap(subcommand)]
    cmd: Option<SubCommand>,
}

#[derive(ClapConfig, Parser, Debug, Clone, PartialEq)]
pub enum SubCommand {
    SubcommandA(SubcommandAOptions),
    SubcommandB(SubcommandBOptions),
    SubcommandC,
}

#[derive(ClapConfig, Parser, Debug, Clone, PartialEq)]
pub struct SubcommandAOptions {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag_x: String,

    #[clap(long, default_value = FROM_DEFAULT)]
    flag_y: String,
}

#[derive(ClapConfig, Parser, Debug, Clone, PartialEq)]
pub struct SubcommandBOptions {
    #[clap(long, default_value = FROM_DEFAULT)]
    flag_z: String,
}

/// Preferred config sets some values.
fn preferred() -> Result<OptsConfig> {
    Ok(serde_yaml::from_str(&format!(
        "
flag_a: {FROM_PREFERRED}
flag_b: {FROM_PREFERRED}
list: [{FROM_PREFERRED}]
subcommand_a:
  flag_x: {FROM_PREFERRED}
"
    ))?)
}

/// Fallback config sets an overlapping set of values.
fn fallback() -> Result<OptsConfig> {
    Ok(serde_yaml::from_str(&format!(
        "
flag_b: {FROM_FALLBACK}
flag_c: {FROM_FALLBACK}
list: [{FROM_FALLBACK}, {FROM_FALLBACK}]
subcommand_a:
  flag_x: {FROM_FALLBACK}
  flag_y: {FROM_FALLBACK}
subcommand_b:
  flag_z: {FROM_FALLBACK}
"
    ))?)
}

/// Values in the preferred config win, missing values come from the fallback, and subcommand
/// configs are merged recursively.
#[test]
fn merge_configs() -> Result<()> {
    let merged = preferred()?.merge(fallback()?);

    assert_eq!(Some(FROM_PREFERRED), merged.flag_a.as_deref());
    assert_eq!(Some(FROM_PREFERRED), merged.flag_b.as_deref());
    assert_eq!(Some(FROM_FALLBACK), merged.flag_c.as_deref());
    // Lists are replaced, not combined.
    assert_eq!(Some(vec![FROM_PREFERRED.to_owned()]), merged.list);

    let cmd = merged.cmd.unwrap_or_default();
    let subcommand_a = cmd.subcommand_a.unwrap_or_default();
    assert_eq!(Some(FROM_PREFERRED), subcommand_a.flag_x.as_deref());
    assert_eq!(Some(FROM_FALLBACK), subcommand_a.flag_y.as_deref());
    let subcommand_b = cmd.subcommand_b.unwrap_or_default();
    assert_eq!(Some(FROM_FALLBACK), subcommand_b.flag_z.as_deref());
    Ok(())
}

/// Merging with an empty config is a no-op in either direction.
#[test]
fn merge_empty() -> Result<()> {
    let preferred_json = serde_json::to_value(preferred()?)?;
    assert_eq!(
        preferred_json,
        serde_json::to_value(preferred()?.merge(OptsConfig::default()))?
    );
    assert_eq!(
        preferred_json,
        serde_json::to_value(OptsConfig::default().merge(preferred()?))?
    );
    Ok(())
}

/// CLI args still win over the merged config, which wins over clap defaults.
#[test]
fn merged_config_under_args() -> Result<()> {
    let matches = <Opts as CommandFactory>::command().get_matches_from([
        "myapp",
        "--flag-a",
        FROM_ARG,
        "subcommand-a",
        "--flag-x",
        FROM_ARG,
    ]);
    let opts = Opts::from_merged(matches, Some(preferred()?.merge(fallback()?)));

    let expected = Opts {
        flag_a: FROM_ARG.to_owned(),
        flag_b: FROM_PREFERRED.to_owned(),
        flag_c: FROM_FALLBACK.to_owned(),
        list: vec![FROM_PREFERRED.to_owned()],
        cmd: Some(SubCommand::SubcommandA(SubcommandAOptions {
            flag_x: FROM_ARG.to_owned(),
            flag_y: FROM_FALLBACK.to_owned(),
        })),
    };
    assert_eq!(expected, opts);

    let matches = <Opts as CommandFactory>::command().get_matches_from(["myapp", "subcommand-c"]);
    let opts = Opts::from_merged(matches, Some(preferred()?.merge(fallback()?)));
    assert_eq!(Some(SubCommand::SubcommandC), opts.cmd);
    assert_eq!(FROM_FALLBACK, opts.flag_c);
    Ok(())
}
