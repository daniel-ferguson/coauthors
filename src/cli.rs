use clap::{App, Arg, SubCommand};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    let add = SubCommand::with_name("add")
        .about("Add to list of available co-authors")
        .arg(Arg::with_name("ALIAS").required(true).index(1))
        .arg(Arg::with_name("NAME").required(true).index(2))
        .arg(Arg::with_name("EMAIL").required(true).index(3));
    let ls = SubCommand::with_name("ls").about("Prints available and active co-authors");
    let print = SubCommand::with_name("print")
        .about("Format active co-authors for adding to a commit message");
    let reset = SubCommand::with_name("reset").about("Remove active co-authors");

    App::new("coauthors")
        .version("0.1")
        .author("Daniel Ferguson <danielferguson@me.com>")
        .about("A git wrapper for pairing")
        .subcommand(add)
        .subcommand(ls)
        .subcommand(print)
        .subcommand(reset)
}
