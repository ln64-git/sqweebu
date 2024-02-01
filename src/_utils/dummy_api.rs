use reqwest::Error as ReqwestError;
use serde_json::Value;

pub async fn get_response() -> Result<Value, ReqwestError> {
    let endpoint = "https://dummy.restapiexample.com/api/v1/employees";
    let client = reqwest::Client::new();
    let response = client.get(endpoint).send().await?; // Change POST to GET
    let body = response.json().await?; // Parse response body as JSON
    Ok(body)
}
