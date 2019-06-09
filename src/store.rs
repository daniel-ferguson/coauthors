use git2;

use std::error::Error;
#[cfg(test)]
use std::path::Path;

use crate::author::Author;

type StoreResult<T> = Result<T, Box<dyn Error>>;

pub trait Store {
    fn add(&mut self, author: &Author) -> StoreResult<()>;
    fn active(&self) -> StoreResult<Vec<Author>>;
    fn authors(&self) -> StoreResult<Vec<Author>>;
    fn clear(&mut self) -> StoreResult<()>;
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

    #[cfg(test)]
    pub fn with_config_path(path: &Path) -> StoreResult<Self> {
        let config = git2::Config::open(path)?;
        Ok(GitConfig { config })
    }
}

impl Store for GitConfig {
    fn add(&mut self, author: &Author) -> StoreResult<()> {
        self.config.set_multivar(
            "pear.author",
            "^$",
            &format!("{} | {} | {}", author.alias, author.name, author.email),
        )?;

        Ok(())
    }

    fn authors(&self) -> StoreResult<Vec<Author>> {
        let mut out = Vec::new();

        for entry in &self.config.entries(Some("pear.author"))? {
            let entry = entry?;
            if let Some(value) = entry.value() {
                let author: Author = value.parse()?;
                out.push(author);
            }
        }
        Ok(out)
    }

    fn active(&self) -> StoreResult<Vec<Author>> {
        let mut out = Vec::new();

        for entry in &self.config.entries(Some("pear.active"))? {
            let entry = entry?;
            if let Some(value) = entry.value() {
                let author: Author = value.parse()?;
                out.push(author);
            }
        }
        Ok(out)
    }

    fn clear(&mut self) -> StoreResult<()> {
        self.config.remove_multivar("pear.active", ".*")?;
        Ok(())
    }

    fn set(&mut self, authors: &[Author]) -> StoreResult<()> {
        if authors.is_empty() {
            return Ok(());
        }

        if !self.active()?.is_empty() {
            self.config.remove_multivar("pear.active", ".*")?;
        }

        for author in authors {
            self.config.set_multivar(
                "pear.active",
                "^$",
                &format!("{} | {} | {}", author.alias, author.name, author.email),
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use std::fs;
    use std::io::Write;

    use super::Store;
    use super::*;

    #[test]
    fn authors() {
        let mut file = NamedTempFile::new().unwrap();
        let store = GitConfig::with_config_path(file.path()).unwrap();

        write!(
            file,
            r#"[pear]
          author = gd | Good Dog | good_dog@gmail.com
          author = ic | Ice Cream | cool_cream@hotmail.com
        "#
        ).unwrap();

        assert_eq!(
            store.authors().unwrap(),
            vec![
                Author {
                    alias: "gd".into(),
                    name: "Good Dog".into(),
                    email: "good_dog@gmail.com".into()
                },
                Author {
                    alias: "ic".into(),
                    name: "Ice Cream".into(),
                    email: "cool_cream@hotmail.com".into()
                }
            ]
        );
    }

    #[test]
    fn active() {
        let mut file = NamedTempFile::new().unwrap();
        let store = GitConfig::with_config_path(file.path()).unwrap();

        write!(
            file,
            r#"[pear]
          author = gd | Good Dog | good_dog@gmail.com
          active = gd | Good Dog | good_dog@gmail.com
        "#
        ).unwrap();

        assert_eq!(
            store.active().unwrap(),
            vec![Author {
                alias: "gd".into(),
                name: "Good Dog".into(),
                email: "good_dog@gmail.com".into()
            }]
        );
    }

    #[test]
    fn clear() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"
[pear]
  author = gd | Good Dog | good_dog@gmail.com
  active = gd | Good Dog | good_dog@gmail.com
        "#
        ).unwrap();

        let mut store = GitConfig::with_config_path(file.path()).unwrap();

        store.clear().unwrap();

        let file_contents = fs::read_to_string(file.path()).unwrap();

        assert!(!(file_contents).contains("active"));
    }

    #[test]
    fn add() {
        let file = NamedTempFile::new().unwrap();
        let mut store = GitConfig::with_config_path(file.path()).unwrap();

        store
            .add(&Author {
                alias: "gd".into(),
                name: "Good Dog".into(),
                email: "good_dog@gmail.com".into(),
            }).unwrap();

        let file_contents = fs::read_to_string(file.path()).unwrap();

        assert!(file_contents.contains("[pear]"));
        assert!(file_contents.contains("author = gd | Good Dog | good_dog@gmail.com"));
    }

    #[test]
    fn set() {
        let mut file = NamedTempFile::new().unwrap();

        write!(
            file,
            r#"
[pear]
  author = gd | Good Dog | good_dog@gmail.com
        "#
        ).unwrap();

        let mut store = GitConfig::with_config_path(file.path()).unwrap();

        store
            .set(&[Author {
                alias: "gd".into(),
                name: "Good Dog".into(),
                email: "good_dog@gmail.com".into(),
            }]).unwrap();

        let file_contents = fs::read_to_string(file.path()).unwrap();

        assert!(file_contents.contains("active = gd | Good Dog | good_dog@gmail.com"));
    }
}
