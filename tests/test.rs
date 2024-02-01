#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use response_engine::get_azure_response;
    use response_engine::get_response;
    use std::env;

    #[tokio::test]
    async fn test_get_response() {
        match get_response().await {
            Ok(response) => match serde_json::from_value::<serde_json::Value>(response.clone()) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Failed to parse JSON response: {:?}", err);
                }
            },
            Err(err) => {
                panic!("Failed to get response: {:?}", err);
            }
        }
    }

    use reqwest::header::CONTENT_TYPE;

    #[cfg(test)]
    mod tests {
        use dotenv::dotenv;
        use reqwest::{header::CONTENT_TYPE, Error as ReqwestError};
        use response_engine::get_azure_response;
        use std::env;

        #[tokio::test]
        async fn test_get_azure_response() {
            dotenv().ok();
            let text_to_speak = "Hello, this is a test.";

            let response: Result<reqwest::Response, ReqwestError> =
                get_azure_response(text_to_speak).await;
            handle_response(response);
        }

        fn handle_response(response: Result<reqwest::Response, ReqwestError>) {
            match response {
                Ok(response) => {
                    assert!(response.status().is_success());
                    // Check if the content type is MP3
                    let content_type = response
                        .headers()
                        .get(CONTENT_TYPE)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("");
                    assert_eq!(content_type, "audio/mpeg");
                }
                Err(err) => {
                    panic!("Failed to get Azure response: {:?}", err);
                }
            }
        }
    }
}
