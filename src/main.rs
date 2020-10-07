use std::fs::File;

use anyhow::Error;
use clap::{App, Arg};
use serde_json as json;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> Result<(), Error> {
    let matches = App::new("word-count")
        .author(AUTHORS)
        .about(ABOUT)
        .version(VERSION)
        .subcommand(
            App::new("create")
                .arg(
                    Arg::with_name("size")
                        .short("s")
                        .long("size")
                        .required(true)
                        .value_name("SIZE")
                        .help("The size of the file to be generated."),
                )
                .arg(
                    Arg::with_name("name")
                        .index(1)
                        .required(true)
                        .value_name("NAME")
                        .help("The name of the file to be generated."),
                ),
        )
        .subcommand(
            App::new("count")
                .arg(
                    Arg::with_name("name")
                        .index(1)
                        .required(true)
                        .value_name("FILE")
                        .help("The name of the file to count words."),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .required(true)
                        .value_name("NAME")
                        .help("The file name (without ext) for the output."),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("create", Some(submatches)) => {
            let name = submatches.value_of("name").unwrap();
            let size = submatches.value_of("size").unwrap();

            let bytes = unbytify::unbytify(size)?;
            word_count::create(name, bytes as usize)?;
        }

        ("count", Some(submatches)) => {
            let name = submatches.value_of("name").unwrap();
            let count = word_count::count(name)?;

            let output = submatches.value_of("output").unwrap();
            let output = File::create(format!("{}.json", output))?;

            json::to_writer_pretty(output, &count)?;
        }

        _ => {}
    }

    Ok(())
}
