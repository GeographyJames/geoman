use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct NameInputDTO(String);

impl std::fmt::Display for NameInputDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NameInputDTO {
    pub fn parse(s: String) -> Result<NameInputDTO, String> {
        if s.trim().is_empty() {
            return Err("cannot be empty".to_string());
        }
        if !s.chars().any(|c| c.is_alphabetic()) {
            return Err("must contain at least one letter".to_string());
        }
        let max_chars = 256;
        if s.graphemes(true).count() > max_chars {
            return Err(format!(
                "name is greater than max of {max_chars} characters"
            ));
        }
        Ok(Self(s))
    }
}

impl AsRef<str> for NameInputDTO {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
