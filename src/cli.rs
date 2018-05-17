use clap::{App, SubCommand};

pub fn app<'a, 'b>() -> App<'a, 'b> {
    let ls = SubCommand::with_name("ls").about("Prints available and active co-authors");
    let print = SubCommand::with_name("print")
        .about("Format active co-authors for adding to a commit message");

    App::new("coauthors")
        .version("0.1")
        .author("Daniel Ferguson <danielferguson@me.com>")
        .about("A git wrapper for pairing")
        .subcommand(ls)
        .subcommand(print)
}
