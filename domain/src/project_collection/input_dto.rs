use crate::enums::GeometryType;

pub struct ProjectCollectionInputDto {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub geometry_type: GeometryType,
}

pub fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
