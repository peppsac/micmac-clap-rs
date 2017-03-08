#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
	let yaml = load_yaml!("mm3d-cli.yml");
    let m = App::from_yaml(yaml);

    // normal logic continues...
}