use crate::common::TestApp;

#[tokio::test]
async fn post_figure_works() {
    let (app, user, project_id) = TestApp::with_project().await;

    let _figure_id = app.generate_figure_id(Some(&user), project_id).await;
}
