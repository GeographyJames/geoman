use serde::Serialize;

use crate::name::validate_name;

#[derive(Debug, Serialize)]
pub struct ProjectNameInputDTO(String);

impl std::fmt::Display for ProjectNameInputDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ProjectNameInputDTO {
    pub fn parse(s: String) -> Result<ProjectNameInputDTO, String> {
        validate_name(&s)?;
        if s.trim().parse::<i64>().is_ok() {
            return Err("name cannot be an integer".into());
        };
        Ok(Self(s))
    }
}

impl AsRef<str> for ProjectNameInputDTO {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectNameInputDTO;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(ProjectNameInputDTO::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(ProjectNameInputDTO::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(ProjectNameInputDTO::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(ProjectNameInputDTO::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(ProjectNameInputDTO::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "My New Wind Farm".to_string();
        assert_ok!(ProjectNameInputDTO::parse(name));
    }

    #[test]
    fn names_cannot_be_integers() {
        let name = "1".to_string();
        assert_err!(ProjectNameInputDTO::parse(name));
    }
}
