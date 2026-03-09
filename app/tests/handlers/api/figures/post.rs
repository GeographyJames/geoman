use crate::app::TestApp;

#[tokio::test]
async fn post_figure_works() {
    let app = TestApp::spawn_and_login().await;
    let project_id = app.generate_project_id().await;
    println!("here");
    let _figure_id = app.generate_figure_id(project_id).await;
}
