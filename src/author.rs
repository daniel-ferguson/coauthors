use std::convert::TryFrom;

use regex::Regex;

use patch_format::PatchFormat;

#[derive(Debug, PartialEq)]
pub struct Author {
    pub alias: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl<'a> TryFrom<&'a str> for Author {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Author, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\S+)\s*\|\s*(.+?)\s*\|\s*(\S+)\s*$").unwrap();
        }

        match RE.captures(value) {
            Some(ref captures) if captures.len() == 4 => Ok(Author {
                alias: captures[1].into(),
                name: captures[2].into(),
                email: captures[3].into(),
            }),
            _ => Err(ParseError),
        }
    }
}

impl PatchFormat for Author {
    fn format(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

#[cfg(test)]
mod tests {
    use super::TryFrom;

    use super::Author;
    use super::ParseError;
    use super::PatchFormat;

    #[test]
    fn test_format_for_patch() {
        let author = Author {
            alias: "doggo".into(),
            name: "Really Good Doggo".into(),
            email: "doggo113@email.com".into(),
        };

        assert_eq!(
            &author.format()[..],
            "Really Good Doggo <doggo113@email.com>"
        );
    }

    #[test]
    fn test_parse_alias() {
        let line = "doggo | Really Good Doggo | doggo113@email.co.uk";
        let author = Author::try_from(line).unwrap();
        assert_eq!(author.alias, "doggo");
    }

    #[test]
    fn test_parse_name() {
        let line = "doggo | Really Good Doggo | doggo113@email.co.uk";
        let author = Author::try_from(line).unwrap();
        assert_eq!(author.name, "Really Good Doggo");
    }

    #[test]
    fn test_parse_email() {
        let line = "doggo | Really Good Doggo | doggo113@email.co.uk";
        let author = Author::try_from(line).unwrap();
        assert_eq!(author.email, "doggo113@email.co.uk");
    }

    #[test]
    fn test_parse_unexpected_format() {
        let line = "doggo | ";
        assert_eq!(Author::try_from(line), Err(ParseError));
    }
}
