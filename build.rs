#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand, Shell, AppSettings};

fn main() {
    let yaml = load_yaml!("src/mm3d-cli.yml");
    let mut app = App::from_yaml(yaml)
    	.global_setting(AppSettings::HidePossibleValuesInHelp)
        .setting(AppSettings::VersionlessSubcommands)
    	.global_setting(AppSettings::DisableVersion);

    app.gen_completions("mm3d",          // We need to specify the bin name manually
                        Shell::Bash,      // Then say which shell to build completions for
                        env!("OUT_DIR")); // Then say where write the completions to

    app.gen_completions("mm3d",          // We need to specify the bin name manually
                        Shell::Fish,      // Then say which shell to build completions for
                        env!("OUT_DIR")); // Then say where write the completions to

    app.gen_completions("mm3d",          // We need to specify the bin name manually
                        Shell::Zsh,      // Then say which shell to build completions for
                        env!("OUT_DIR")); // Then say where write the completions to

}