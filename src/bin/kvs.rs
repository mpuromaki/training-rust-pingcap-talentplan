use anyhow::{self, Result};
use docopt::Docopt;
use kvs::KvStore;
use serde::Deserialize;
use std::path;
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

fn set(cwd: path::PathBuf, key: String, value: String) -> Result<()> {
    let get_store = KvStore::open(&cwd.as_path());
    match get_store {
        Ok(mut store) => {
            store
                .set(key, value)
                .expect("Could not store key and value.");
        }
        Err(e) => anyhow::bail!("Could not open datastore. {:?}", e),
    }
    Ok(())
}

fn get(cwd: path::PathBuf, key: String) -> Result<()> {
    let get_store = KvStore::open(&cwd.as_path());
    match get_store {
        Ok(mut store) => {
            let value = store.get(key).expect("Could not get value");
            match value {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }
        }
        Err(e) => anyhow::bail!("Could not open datastore. {:?}", e),
    }
    Ok(())
}

fn rm(cwd: path::PathBuf, key: String) -> Result<()> {
    let get_store = KvStore::open(&cwd.as_path());
    match get_store {
        Ok(mut store) => {
            let result = store.remove(key);
            match result {
                Ok(_) => {}
                Err(_) => anyhow::bail!("Key not found"),
            }
        }
        Err(e) => anyhow::bail!("Could not open datastore. {:?}", e),
    }
    Ok(())
}

fn main() -> Result<()> {
    // Handle command line arguments with docopt
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    //#[rustfmt::skip]
    match args {
        Args {
            flag_version: true, ..
        } => {
            println!("kvs.exe {}", env!("CARGO_PKG_VERSION"));
            process::exit(1)
        }
        Args { cmd_set: true, .. } => {
            let returnval = set(
                path::PathBuf::from("./"),
                args.arg_key.unwrap(),
                args.arg_value.unwrap(),
            );
            match returnval {
                Ok(_) => process::exit(0),
                Err(err) => {
                    println!("{}", err);
                    process::exit(1)
                }
            }
        }
        Args { cmd_get: true, .. } => {
            let returnval = get(path::PathBuf::from("./"), args.arg_key.unwrap());
            match returnval {
                Ok(_) => process::exit(0),
                Err(err) => {
                    println!("{}", err);
                    process::exit(1)
                }
            }
        }
        Args { cmd_rm: true, .. } => {
            let returnval = rm(path::PathBuf::from("./"), args.arg_key.unwrap());
            match returnval {
                Ok(_) => process::exit(0),
                Err(err) => {
                    println!("{}", err);
                    process::exit(1)
                }
            }
        }
        _ => process::exit(99),
    }
}
