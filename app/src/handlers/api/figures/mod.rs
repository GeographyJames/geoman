mod post;
pub use post::post_figure;
mod payload;
pub use payload::{
    FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload, PgTablePayload,
};
mod get;
pub use get::{get_figure, get_figures};
mod patch;
pub use patch::patch_figure;
mod delete;
pub use delete::delete_figure;
pub mod get_print;
pub use get_print::FigureFormat;
