pub trait IntoOGCFeature {
    fn into_ogc_feature(self, collection_url: String) -> ogc::types::Feature
    where
        Self: Sized;
}
