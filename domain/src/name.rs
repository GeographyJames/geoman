use unicode_segmentation::UnicodeSegmentation;
pub struct NameInputDTO(String);

impl std::fmt::Display for NameInputDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NameInputDTO {
    pub fn parse(s: String) -> Result<NameInputDTO, String> {
        validate_name(&s)?;
        if s.trim().parse::<i64>().is_ok() {
            return Err("name cannot be an integer".into());
        };
        Ok(Self(s))
    }
}

impl AsRef<str> for NameInputDTO {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub fn validate_name(s: &str) -> Result<(), String> {
    if s.trim().is_empty() {
        return Err("name cannot be empty".to_string());
    }
    let max_chars = 256;
    if s.graphemes(true).count() > max_chars {
        return Err(format!(
            "name is greater than max of {max_chars} characters"
        ));
    }
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    if let Some(char) = s.chars().find(|char| forbidden_characters.contains(char)) {
        return Err(format!("name contains forbidden character: '{char}'"));
    }
    Ok(())
}
