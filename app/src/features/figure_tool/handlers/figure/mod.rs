mod post;
pub use post::post_figure;
mod payload;
pub use payload::{
    FigureLayerDatasourcePayload, FigureLayerPayload, FigurePayload, PgTablePayload,
};
mod get;
pub use get::{get_figure, get_figures};
mod patch;
pub use patch::{FigureUpdatePayload, patch_figure};
mod print;
pub use print::{GetPrintRequestBuilder, get_print};
