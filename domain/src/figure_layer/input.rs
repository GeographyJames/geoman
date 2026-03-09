use crate::domain::{
    dtos::{Id, LayerProperties},
    enums::FigureLayerDatasourceInput,
};

#[derive(Debug)]
pub struct FigureLayerInputDTO {
    pub style_id: Option<Id>,
    pub name: LayerNameInputDTO,
    pub properties: LayerProperties,
    pub source: FigureLayerDatasourceInput,
}

#[derive(Debug)]
pub struct LayerNameInputDTO(String);

impl LayerNameInputDTO {
    pub fn parse(s: String) -> Result<Self, String> {
        let permitted_chars: Vec<char> = ('a'..='z').chain('0'..='9').chain(['-', '_']).collect();
        for char in s.chars() {
            if !permitted_chars.contains(&char) {
                return Err(format!(
                    "layer name contains invalid character: ({}). Layer name must only contain hyphens, underscores and lowercase alphanumeric characters ('a-z',  '0-9', '-' and '_').",
                    char
                ));
            }
        }
        Ok(Self(s))
    }
}

impl AsRef<str> for LayerNameInputDTO {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for LayerNameInputDTO {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}
