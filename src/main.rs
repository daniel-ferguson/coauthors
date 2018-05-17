#![feature(try_from)]

extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod author;
mod cli;
mod patch_format;

use std::convert::TryFrom;
use std::error::Error;

use author::Author;
use patch_format::PatchFormat;

fn ls() -> Result<(), Box<Error>> {
    let config = git2::Config::open_default()?;

    println!("Available authors:\n");
    for entry in &config.entries(Some("pair.user"))? {
        let entry = entry?;
        if let Some(value) = entry.value() {
            let author = Author::try_from(value)?;

            println!("* {}", author.format());
        }
    }

    println!("\n\nActive authors:\n");
    for entry in &config.entries(Some("pair.active"))? {
        let entry = entry?;
        if let Some(value) = entry.value() {
            let author = Author::try_from(value)?;

            println!("* {}", author.format());
        }
    }
    Ok(())
}

fn print() -> Result<(), Box<Error>> {
    let config = git2::Config::open_default()?;

    for entry in &config.entries(Some("pair.active"))? {
        let entry = entry?;
        if let Some(value) = entry.value() {
            let author = Author::try_from(value)?;

            println!("Co-authored-by: {}", author.format());
        }
    }
    Ok(())
}

fn reset() -> Result<(), Box<Error>> {
    let mut config = git2::Config::open_default()?;

    config.remove_multivar("pair.active", "")?;

    Ok(())
}

fn main() {
    let matches = cli::app().get_matches();

    let result = match matches.subcommand() {
        ("ls", Some(_)) => ls(),
        ("print", Some(_)) => print(),
        ("reset", Some(_)) => reset(),
        _ => {
            println!("{}", matches.usage());
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
