#![feature(try_from)]

extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod author;
mod patch_format;

use author::Author;

fn main() {
    use patch_format::PatchFormat;
    let author = Author {
        alias: "doggo".into(),
        name: "Good Doggo".into(),
        email: "doggo113@gmail.com".into(),
    };

    println!("{}", author.format());
}
