# Coauthors

A (ridiculously overengineered) git subcommand for managing co-authored-by
commit trailer for pairing. It may eat your git config

**WARNING - incomplete and subject to regular breaking changes**

*Everything will probably break/change as I get a better understanding of what
a good workflow around applying Co-authored-by trailers looks like.*

## Requirements

* A recent version of rustc and cargo (tested on 1.28.0 stable)
* Ruby (for the prepare-commit-msg script, I'd love to rewrite this in bash)
* git


## Installation

Clone this repo

    git clone git@github.com:daniel-ferguson/coauthors.git

Build

    cargo build --release

Add binary to location in your `PATH`

    cp target/release/git-coauthors ~/local/bin

Check it works

    git coauthors -h

## Configure a template for commit messages

Create a commit message template, containing the string {%authors%} where you'd
like your coauthor list to appear, for example:

    $ cat ~/.git-commit-template
    
    
    {%authors%}

Tell git to use the commit template (globally)

    git config --global commit.template ~/.git-commit-template

Add the prepare-commit-msg hook to your local git repo

    cp git-hooks/prepare-commit-msg ~/projects/your-repo/.git/hooks


## CLI Usage

```
A git wrapper for pairing

USAGE:
    git-coauthors [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Add to list of available co-authors
    help     Prints this message or the help of the given subcommand(s)
    ls       Prints available and active co-authors
    print    Format active co-authors for adding to a commit message
    reset    Remove active co-authors
    set      Set active co-authors
```

[1]: https://help.github.com/articles/creating-a-commit-with-multiple-authors/
