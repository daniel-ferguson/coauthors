#![feature(try_from)]

extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod author;
mod cli;
mod git_config_format;
mod patch_format;

use std::convert::TryFrom;
use std::error::Error;

use author::Author;

fn add(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    use git_config_format::GitConfigFormat;

    let author = Author {
        alias: args.value_of("ALIAS").unwrap().into(),
        name: args.value_of("NAME").unwrap().into(),
        email: args.value_of("EMAIL").unwrap().into(),
    };

    let mut config = git2::Config::open_default()?.open_level(git2::ConfigLevel::Global)?;

    config.set_multivar("pair.user", "^$", &author.format())?;

    Ok(())
}

fn ls() -> Result<(), Box<Error>> {
    use patch_format::PatchFormat;

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
    use patch_format::PatchFormat;

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

fn set(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    use git_config_format::GitConfigFormat;
    let mut config = git2::Config::open_default()?;

    let mut authors = Vec::new();
    {
        let entries = config.entries(Some("pair.user"))?;

        for entry in &entries {
            let entry = entry?;
            if let Some(value) = entry.value() {
                let author = Author::try_from(value)?;
                authors.push(author);
            }
        }
    }

    let aliases: Vec<&str> = args.values_of("ALIASES").unwrap().collect();
    let authors: Vec<Author> = authors
        .into_iter()
        .filter(|a| aliases.contains(&a.alias.as_ref()))
        .collect();

    if authors.len() > 0 {
        // Ignore failures here - a common case is when pair.active hasn't
        // yet been set
        let _ = config.remove_multivar("pair.active", ".*");

        for author in authors {
            config.set_multivar("pair.active", "^$", &author.format())?;
        }
    }

    Ok(())
}

fn main() {
    let matches = cli::app().get_matches();

    let result = match matches.subcommand() {
        ("add", Some(args)) => add(args),
        ("ls", Some(_)) => ls(),
        ("print", Some(_)) => print(),
        ("reset", Some(_)) => reset(),
        ("set", Some(args)) => set(args),
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
