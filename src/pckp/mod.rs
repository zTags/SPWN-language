pub mod util;
pub mod add;
pub mod remove;
pub mod restore;

use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
    match args.subcommand().unwrap() {
        ("add", cmd) => {
            add::add(
                cmd.get_many::<String>("LIBRARIES").unwrap().collect()
            );
        },
        ("remove", cmd) => {
            remove::remove(
                cmd.get_many::<String>("LIBRARIES").unwrap().collect()
            );
        },
        ("restore", cmd) => {
            restore::restore();
        },
        (_,_) => unreachable!(),
    }

    return;
}
