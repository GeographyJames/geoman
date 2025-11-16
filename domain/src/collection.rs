use serde::Deserialize;

#[derive(Deserialize)]
pub struct Collection {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
}

impl Collection {
    pub fn into_ogc_collection(self, collections_url: &str) -> ogc::Collection {
        let Self {
            title,
            slug,
            description,
            ..
        } = self;
        ogc::Collection::new(slug, title, description, collections_url)
    }
}
