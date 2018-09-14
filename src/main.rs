extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate tempfile;

mod author;
mod cli;
mod store;

use std::error::Error;

use author::Author;
use store::Store;

fn add(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let author = Author {
        alias: args.value_of("ALIAS").unwrap().into(),
        name: args.value_of("NAME").unwrap().into(),
        email: args.value_of("EMAIL").unwrap().into(),
    };

    let mut store = store::GitConfig::new()?;

    store.add(&author)?;

    Ok(())
}

fn ls() -> Result<(), Box<Error>> {
    let store = store::GitConfig::new()?;

    println!("Available authors:\n");
    for author in store.authors()? {
        println!("* {}", author);
    }

    println!("\n\nActive authors:\n");
    for author in store.active()? {
        println!("* {}", author);
    }
    Ok(())
}

fn print() -> Result<(), Box<Error>> {
    let store = store::GitConfig::new()?;

    for author in store.active()? {
        println!("Co-authored-by: {} <{}>", author.name, author.email);
    }

    Ok(())
}

fn reset() -> Result<(), Box<Error>> {
    store::GitConfig::new()?.clear()
}

fn set(args: &clap::ArgMatches) -> Result<(), Box<Error>> {
    let mut store = store::GitConfig::new()?;

    let aliases: Vec<&str> = args.values_of("ALIASES").unwrap().collect();
    let authors: Vec<Author> = store
        .authors()?
        .into_iter()
        .filter(|a| aliases.contains(&a.alias.as_ref()))
        .collect();

    store.set(&authors)?;

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
