pub fn assert_ok(response: &reqwest::Response) {
    assert_eq!(response.status().as_u16(), 200)
}
