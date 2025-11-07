use reqwest::Response;
use serde::de::DeserializeOwned;

pub fn assert_ok(response: &reqwest::Response) {
    assert_eq!(response.status().as_u16(), 200)
}

pub async fn handle_json_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, anyhow::Error> {
    let status = response.status().as_u16();

    if response.status().is_success() {
        let json: T = response.json().await.expect(&format!(
            "Failed to deserialise successful {status} response"
        ));
        return Ok(json);
    }

    let error = response
        .text()
        .await
        .unwrap_or("no repsonse body".to_string());

    Err(anyhow::anyhow!(
        "Unsuccessful response status: {status}\nbody:\n{:#}",
        error,
    ))
}
