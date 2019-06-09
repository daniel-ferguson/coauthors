mod author;
mod cli;
mod store;

use std::error::Error;

use crate::author::Author;
use crate::store::Store;

fn add(alias: String, name: String, email: String) -> Result<(), Box<dyn Error>> {
    let author = Author { alias, name, email };

    let mut store = store::GitConfig::new()?;

    store.add(&author)?;

    Ok(())
}

fn ls() -> Result<(), Box<dyn Error>> {
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

fn print() -> Result<(), Box<dyn Error>> {
    let store = store::GitConfig::new()?;

    for author in store.active()? {
        println!("Co-authored-by: {} <{}>", author.name, author.email);
    }

    Ok(())
}

fn reset() -> Result<(), Box<dyn Error>> {
    store::GitConfig::new()?.clear()
}

fn set(aliases: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut store = store::GitConfig::new()?;

    let authors: Vec<Author> = store
        .authors()?
        .into_iter()
        .filter(|a| aliases.contains(&a.alias))
        .collect();

    store.set(&authors)?;

    Ok(())
}

fn main() {
    use cli::{Command, Opt};
    use structopt::StructOpt;

    let opt = Opt::from_args();

    let result = match opt.cmd {
        Command::Add { alias, name, email } => add(alias, name, email),
        Command::Ls => ls(),
        Command::Print => print(),
        Command::Reset => reset(),
        Command::Set { aliases } => set(aliases),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
