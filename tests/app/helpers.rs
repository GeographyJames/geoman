use reqwest::Response;
use serde::de::DeserializeOwned;

pub fn assert_ok(response: &reqwest::Response) {
    assert_eq!(response.status().as_u16(), 200)
}

pub async fn handle_json_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, anyhow::Error> {
    if response.status().is_success() {
        let token: T = response
            .json()
            .await
            .expect("failed to deserialise successful response");
        return Ok(token);
    }
    let status = response.status().as_u16();
    let error: serde_json::Value = response.json().await.unwrap_or(serde_json::json!(
        "failed to deserialise unsuccessful response"
    ));
    Err(anyhow::anyhow!(
        "response status: {status}\nError:\n{:#}",
        error,
    ))
}
