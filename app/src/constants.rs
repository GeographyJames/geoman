pub const ENVIRONMENT_VARIABLE_PREFIX: &str = "GEOMAN";
pub const CONFIGURATION_DIRECTORY: &str = "config";
// Todo! the version should be bumped to 3.1 but this breaks OGC test suit
pub const OPEN_API_JSON: &str = "application/vnd.oai.openapi+json;version=3.0";
pub const USER_AUTH_ID_COLUMN: &str = "clerk_id";
pub const GIS_DATA_SCHEMA: &str = "gis_data";
pub const SITE_BOUNDARIES_COLLECTION_ID: i32 = 0;
pub const TURBINE_LAYOUTS_COLLECTION_ID: i32 = -1;

pub mod db_constraints {
    pub const PROJECT_NAME_UNIQUE: &str = "projects_name_key";
    pub const PROJECT_SLUG_UNIQUE: &str = "projects_slug_key";
    pub const PROJECT_COLLECTIONS_TITLE_UNIQUE: &str = "collections_title_global_unique";
    pub const PROJECT_CRS_ID_FKEY: &str = "projects_crs_srid_fkey";
    pub const PROJECT_COLLECTION_SLUG_UNIQUE: &str = "collections_slug_global_unique";
    pub const TURBINE_PROXIMITY_CHECK: &str = "duplicate_turbine_exclusion";
}
