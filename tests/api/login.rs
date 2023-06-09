use crate::helpers::spawn_app;
use crate::helpers::assert_is_redirect_to;


#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
// Arrange
    let app = spawn_app().await;
// Act
    let login_body = serde_json::json!({
"username": "random-username",
"password": "random-password"
});
    let response = app.post_login(&login_body).await;
// Assert
    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>Authentication failed</i></p>"));

    let html_page = app.get_login_html().await;
    assert!(!html_page.contains("Authentication failed"));
}
/*
#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
// Arrange
    let app = spawn_app().await;
// Act - Part 1 - Login
    let login_body = serde_json::json!({
"username": &app.test_user.username,
"password": &app.test_user.password
});
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
// Act - Part 2 - Follow the redirect
    let html_page = app.get_admin_dashboard().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));
}
*/