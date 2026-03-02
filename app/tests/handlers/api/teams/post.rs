use domain::TeamId;

use crate::common::{AppBuilder, Auth};

#[tokio::test]
async fn post_team_works() {
    let app = AppBuilder::new().build().await;
    let auth = Auth::MockUserCredentials(app.generate_user(true, TeamId(0)).await);

    let _team_id = app.generate_team_id(Some(&auth)).await;
}
