use crate::command_prelude::*;
use cargo::ops;

pub fn cli() -> Command {
    subcommand("logout")
        .about("Remove an API token from the registry locally")
        .arg_registry("Registry to use")
        .arg_quiet()
        .after_help(color_print::cstr!(
            "Run `<cyan,bold>cargo help logout</>` for more detailed information.\n"
        ))
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let registry = args.registry(config)?;
    ops::registry_logout(config, registry.as_deref())?;
    Ok(())
}
