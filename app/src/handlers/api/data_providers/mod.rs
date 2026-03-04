mod delete;
pub use delete::delete_data_provider;
mod get;
pub use get::get_data_providers;
mod patch;
pub use patch::{DataProviderUpdatePayload, patch_data_provider};
mod post;
pub use post::{DataProviderInputPayload, post_data_provider};
