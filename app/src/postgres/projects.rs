use domain::Project;
use futures::Stream;

use crate::postgres::{PoolWrapper, traits::SelectAllStreaiming};

impl SelectAllStreaiming for Project {
    fn select_all_streaming(
        executor: PoolWrapper,
    ) -> impl Stream<Item = Result<Self, sqlx::Error>> + use<> {
        sqlx::query_as!(Project, "SELECT id, name, slug FROM app.projects").fetch(executor)
    }
}
