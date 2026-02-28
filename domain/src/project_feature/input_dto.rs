use unicode_segmentation::UnicodeSegmentation;

pub struct FeatureInputDTO {
    pub name: FeatureNameInputDTO,
    pub primary: Option<bool>,
    pub geom_wkb: Vec<u8>,
    pub srid: i32,
    pub target_srid: i32,
}

#[derive(Debug)]
pub struct FeatureNameInputDTO(String);

impl FeatureNameInputDTO {
    pub fn parse(s: String) -> Result<FeatureNameInputDTO, String> {
        if s.trim().is_empty() {
            return Err("cannot be empty".to_string());
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

impl AsRef<str> for FeatureNameInputDTO {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
