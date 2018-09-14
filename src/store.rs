use git2;

use std::error::Error;

use author::Author;

type StoreResult<T> = Result<T, Box<Error>>;

pub trait Store {
    fn add(&mut self, author: &Author) -> StoreResult<()>;
    fn active(&self) -> StoreResult<Vec<Author>>;
    fn authors(&self) -> StoreResult<Vec<Author>>;
    fn clear(&self) -> StoreResult<()>;
    fn set(&mut self, authors: &[Author]) -> StoreResult<()>;
}

pub struct GitConfig {
    config: git2::Config,
}

impl GitConfig {
    pub fn new() -> StoreResult<Self> {
        let config = git2::Config::open_default()?;
        Ok(GitConfig { config })
    }
}

impl Store for GitConfig {
    fn add(&mut self, author: &Author) -> StoreResult<()> {
        let mut config = git2::Config::open_default()?.open_level(git2::ConfigLevel::Global)?;

        config.set_multivar(
            "pear.author",
            "^$",
            &format!("{} | {} | {}", author.alias, author.name, author.email),
        )?;
        Ok(())
    }

    fn authors(&self) -> StoreResult<Vec<Author>> {
        let config = git2::Config::open_default()?;

        let mut out = Vec::new();

        for entry in &config.entries(Some("pear.author"))? {
            let entry = entry?;
            if let Some(value) = entry.value() {
                let author: Author = value.parse()?;
                out.push(author);
            }
        }
        Ok(out)
    }

    fn active(&self) -> StoreResult<Vec<Author>> {
        let config = git2::Config::open_default()?;

        let mut out = Vec::new();

        for entry in &config.entries(Some("pear.active"))? {
            let entry = entry?;
            if let Some(value) = entry.value() {
                let author: Author = value.parse()?;
                out.push(author);
            }
        }
        Ok(out)
    }

    fn clear(&self) -> StoreResult<()> {
        let mut config = git2::Config::open_default()?;
        let _ = config.remove_multivar("pear.active", ".*");

        Ok(())
    }

    fn set(&mut self, authors: &[Author]) -> StoreResult<()> {
        let mut config = git2::Config::open_default()?;

        if authors.is_empty() {
            return Ok(());
        }

        if !self.active()?.is_empty() {
            config.remove_multivar("pear.active", ".*")?;
        }

        for author in authors {
            config.set_multivar(
                "pear.active",
                "^$",
                &format!("{} | {} | {}", author.alias, author.name, author.email),
            )?;
        }

        Ok(())
    }
}
