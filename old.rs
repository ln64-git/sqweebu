pub async fn get_azure_response_old(
    api_key: &str,
    region: &str,
    text_to_speak: &str,
    voice_name: &str,
) -> Result<reqwest::Response, ReqwestError> {
    let endpoint = format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/v1", // Unchanged for v3.1
        region
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Content-Type", "application/ssml+xml")
        .body(format!(
            r#"<speak version='1.0' xmlns='http://www.w3.org/2001/10/synthesis' xml:lang='en-US'>
                <voice name='{}'>{}</voice>
            </speak>"#,
            voice_name, text_to_speak
        ))
        .send()
        .await?;

    Ok(response)
}
