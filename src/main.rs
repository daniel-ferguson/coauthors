#![feature(try_from)]

extern crate clap;
extern crate git2;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod author;
mod patch_format;

fn main() {
    println!("Hello, world!");
}
