pub const ENVIRONMENT_VARIABLE_PREFIX: &str = "GEOMAN";
pub const CONFIGURATION_DIRECTORY: &str = "config";
// Todo! the version should be bumped to 3.1 but this breaks OGC test suit
pub const OPEN_API_JSON: &str = "application/vnd.oai.openapi+json;version=3.0";
pub const USER_AUTH_ID_COLUMN: &str = "clerk_id";
pub const GIS_DATA_SCHEMA: &str = "gis_data";
pub const SITE_BOUNDARIES_COLLECTION_NAME: &str = "site boundaries";

pub mod db_constraints {
    pub const PROJECT_NAME_UNIQUE: &str = "projects_name_key";
    pub const PROJECT_SLUG_UNIQUE: &str = "projects_slug_key";
    pub const PROJECT_COLLECTIONS_TITLE_UNIQUE: &str = "project_collections_title_key";
}
