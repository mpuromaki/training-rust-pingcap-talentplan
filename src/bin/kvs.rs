use docopt::Docopt;
use serde::Deserialize;
use std::process;

const USAGE: &'static str = "
Key-Value store command line interface.

Usage:
    kvs.exe (-h | --help)
    kvs.exe (-v | --version)
    kvs.exe set <key> <value>
    kvs.exe get <key>
    kvs.exe rm <key>

Options:
    -h --help        Show this screen.
    -v --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_set: bool,
    cmd_get: bool,
    cmd_rm: bool,
    arg_key: Option<String>,
    arg_value: Option<String>,
    flag_version: bool,
}

fn unimpl_exit() {
    eprintln!("unimplemented");
    process::exit(1);
}

fn set() {
    unimpl_exit();
}

fn get() {
    unimpl_exit();
}

fn rm() {
    unimpl_exit();
}

fn main() {
    // Handle command line arguments with docopt
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    #[rustfmt::skip]
    match args {
        Args {flag_version: true, ..} => println!("kvs.exe {}", env!("CARGO_PKG_VERSION")),
        Args {cmd_set: true, ..} => set(),
        Args {cmd_get: true, ..} => get(),
        Args {cmd_rm: true, ..} => rm(),
        _ => process::exit(99),
    };
}
