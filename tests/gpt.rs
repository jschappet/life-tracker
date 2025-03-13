#[cfg(test)]
mod tests {
    use life_tracker::gpt::{request_gpt, send_gpt_request, GptMessage, GptRequestParams, RequestError};

    
    use std::env;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    fn get_api_key() -> String {
        dotenvy::dotenv().ok();

        let api_key: String = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        api_key
    }

    #[tokio::test]
    async fn test_send_gpt_request() {

        let _ = env_logger::builder().is_test(true).try_init(); // Add this line

        let messages = vec![GptMessage {
            role: "user".to_string(),
            content: "Hello, GPT!".to_string(),
        }];
        let cancel_request: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let params = GptRequestParams {
            url: "https://api.openai.com/v1/engines/davinci-codex/completions",
            api_key: &get_api_key(),
            max_tokens: 100,
            model: "davinci-codex",
            temperature: 0.7,
            messages: &messages,
            cancel_request: &cancel_request,
        };

        // Mock the HTTP request and response
        let response = send_gpt_request(&params).await;
        //log::debug!("{:?}", response.clone().unwrap().lines().into_iter());
        assert!(response.is_err()); // Since we are using a test API key, it should fail
    }

    #[tokio::test]

    async fn test_request_gpt() {

        let _ = env_logger::builder().is_test(true).try_init(); // Add this line

        let mut messages: Vec<GptMessage> = Vec::new();

        messages.push(GptMessage {
            role: "user".to_string(),
            content: "Hello, GPT".to_string(),
        });
       
        let cancel_request: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));


        let assistant_message_result = request_gpt(GptRequestParams {
            url: "https://api.openai.com/v1/engines/davinci-codex/completions",
            api_key: &get_api_key(),
            messages: &messages,
            max_tokens: 100,
            model: "davinci-codex",
            temperature: 0.7,
            cancel_request: &cancel_request,
        })
        .await;
        match assistant_message_result {
            Ok(assistant_message) => {
                messages.push(assistant_message.clone());
                
            }
            Err(RequestError::Cancelled) => {
                cancel_request.store(false, Ordering::SeqCst);
                //  break;
            }
            Err(RequestError::Other(err)) => {
                println!("Error: {:?}", err);
                cancel_request.store(false, Ordering::SeqCst);
            }
        }
        log::debug!("messages: {:?}" , messages);
    }

    
}
