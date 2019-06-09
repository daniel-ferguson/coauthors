use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "git-coauthors", about = "A git subcommand for pairing")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "add", about = "Add to list of available coauthors")]
    Add {
        alias: String,
        name: String,
        email: String,
    },
    #[structopt(name = "ls", about = "Print available and active coauthors")]
    Ls,
    #[structopt(
        name = "print",
        about = "Format active coauthors for adding to a commit message"
    )]
    Print,
    #[structopt(name = "reset", about = "Remove active coauthors")]
    Reset,
    #[structopt(name = "set", about = "Set active coauthors")]
    Set {
        #[structopt(name = "aliases")]
        aliases: Vec<String>,
    },
}
