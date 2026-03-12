mod post;
pub use post::post_figure;
mod payload;
pub use payload::{
    FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload, PgTablePayload,
};
mod get;
pub use get::{get_figure, get_figures};
mod patch;
pub use patch::{patch_figure, FigureUpdatePayload};
// mod get_print;
// pub use get_print::GetPrintRequestBuilder;
// pub use get_print::{FigureFormat, get_print};
// mod delete;
// pub use delete::delete_figure;
