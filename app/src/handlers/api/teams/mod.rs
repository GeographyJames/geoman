mod delete;
pub use delete::delete_team;
mod get;
pub use get::get_teams;
mod patch;
pub use patch::{TeamUpdatePayload, patch_team};
mod post;
pub use post::{TeamInputPayload, post_team};
