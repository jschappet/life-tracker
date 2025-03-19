use chrono::Utc;
use reqwest::{self};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::models::{NewTask, Task};
use crate::settings::Gpt;

#[derive(Debug)]
pub enum RequestError {
    Cancelled,
    Other(Box<dyn std::error::Error>),
}
#[derive( Debug)]
pub struct GptRequestParams<'a> {
    pub url: &'a str,
    pub api_key: &'a str,
    pub max_tokens: u32,
    pub model: &'a str,
    pub temperature: f32,
    pub messages: &'a Vec<GptMessage>,
    pub cancel_request: &'a Arc<AtomicBool>,
}

#[derive(Debug)]
struct GptError {
    message: String,
}

impl Display for GptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GptError: {}", self.message)
    }
}

impl Error for GptError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GptRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    messages: Vec<GptMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GptMessage {
    pub role: String,
    pub content: String,
}


pub async fn request_gpt(params: GptRequestParams<'_>) -> Result<GptMessage, RequestError> {
    //let pb = show_progressbar();
    let response_result = send_gpt_request(&params).await;

    //pb.finish_and_clear();

    if params.cancel_request.load(Ordering::SeqCst) {
        log::error!("Request canceled by user ");
        // dbg!(response_result);
        return Err(RequestError::Cancelled);
    }

    match response_result {
        Ok(response_content) => {
            let assistant_message = GptMessage {
                role: "assistant".to_string(),
                content: response_content.to_string(),
            };
            log::debug!("ChatGPT:{}", response_content);
            Ok(assistant_message)
        }
        Err(err) => {
            //println!("Error: {:?}", err);
            Err(RequestError::Other(err))
        }
    }
}


pub async fn send_gpt_request(params: &GptRequestParams<'_>) -> Result<String, Box<dyn Error>> {


    
    let request = GptRequest {
        model: params.model.to_string(),
        max_tokens: params.max_tokens,
        temperature: params.temperature,
        messages: params.messages.clone(),
    };

    log::debug!("Send Gpt Request: {:?}", request.clone());
    let client = reqwest::Client::new();
    // Adding some debugging information
    log::debug!("Sending request to URL: {}", params.url);
    log::debug!("Using API Key: {}", params.api_key);

    let response = client
        .post(params.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", params.api_key))
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            log::debug!("Response status: {}", resp.status());
            if resp.status().is_success() {
                let json_response = resp.json::<serde_json::Value>().await;
                match json_response {
                    Ok(json) => {
                        log::debug!("Response JSON: {:?}", json);
                        if params.cancel_request.load(Ordering::SeqCst) {
                            println!("Request cancelled by user 2 ...");
                            return Err(Box::new(GptError {
                                message: "Request cancelled by user...".to_string(),
                            }));
                        }

                        let choices = json
                            .get("choices")
                            .and_then(|v| v.as_array())
                            .ok_or_else(|| GptError {
                                message: "Response doesn't contain 'choices' field".to_string(),
                            })?;

                        let message = choices
                            .get(0)
                            .and_then(|v| v.get("message"))
                            .ok_or_else(|| GptError {
                                message: "Response doesn't contain 'message' field".to_string(),
                            })?;

                        let response_content = message
                            .get("content")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_owned();

                        Ok(response_content)
                    }
                    Err(e) => {
                        log::error!("Failed to parse JSON response: {:?}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                log::error!("Request failed with status: {}", resp.status());
                Err(Box::new(GptError {
                    message: format!("Request failed with status: {}", resp.status()),
                }))
            }
        }
        Err(e) => {
            log::error!("Failed to send request: {:?}", e);
            Err(Box::new(e))
        }
    }
}


fn create_sub_tasks(input: String, task: Task) -> Vec<NewTask> {
    
    let input = input.replace("```json", "to").replace("```", "");
    log::debug!("{}", input);
    let json_value: serde_json::Value = serde_json::from_str(&input).unwrap();
    let sub_tasks = json_value["subTasks"].as_array().unwrap();

    let mut tasks = Vec::new();
    for sub_task in sub_tasks {
        let title = sub_task["title"].as_str().unwrap().to_string();
        let description = sub_task["description"].as_str().unwrap().to_string();
        tasks.push(NewTask {
            title,
            description: Some(description),
            due_date: None,
            user_id: task.id,
            status: task.status.clone(),
            project_id: task.project_id,
            start_time: Some(Utc::now().naive_utc()),
            end_time: None,
            parent_task_id: Some(task.id),
        });
    }
    tasks
}

pub async fn get_sub_tasks(task: Task, settings: Gpt) -> Result<Vec<NewTask>, Box<dyn Error>> {
    let formatted_message = format!(
        r#"For the following task I would like you to create 5 new tasks, 
        each task should have a title and description. Title Length < 250c. 
        The results should be formatted as JSON, without extra formating. 
            {{subTaks: [ {{"title": title , "description": description }} ] }}
        The task Description is: {}"#,
        task.title
    );

    let request = GptRequest {
        model: settings.model.to_string(),
        max_tokens: settings.max_tokens,
        temperature: settings.temperature,
        messages: vec![GptMessage {
            role: "user".to_string(),
            content: formatted_message,
        }],
    };

    log::debug!("Send Gpt Request: {:?}", request.clone());
    let client = reqwest::Client::new();
    log::debug!("Sending request to URL: {}", settings.url);
    log::debug!("Using API Key: {}", settings.api_key);

    let response = client
        .post(settings.url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", settings.api_key))
        .json(&request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            log::debug!("Response status: {}", resp.status());
            if resp.status().is_success() {
                let json_response = resp.json::<serde_json::Value>().await;
                match json_response {
                    Ok(json) => {
                        log::debug!("Response JSON: {:?}", json);

                        let choices = json
                            .get("choices")
                            .and_then(|v| v.as_array())
                            .ok_or_else(|| GptError {
                                message: "Response doesn't contain 'choices' field".to_string(),
                            })?;

                        let message = choices
                            .get(0)
                            .and_then(|v| v.get("message"))
                            .ok_or_else(|| GptError {
                                message: "Response doesn't contain 'message' field".to_string(),
                            })?;

                        let response_content = message
                            .get("content")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_owned();

                        Ok(create_sub_tasks(response_content, task))
                    }
                    Err(e) => {
                        log::error!("Failed to parse JSON response: {:?}", e);
                        Err(Box::new(e))
                    }
                }
            } else {
                log::error!("Request failed with status: {}", resp.status());
                Err(Box::new(GptError {
                    message: format!("Request failed with status: {}", resp.status()),
                }))
            }
        }
        Err(e) => {
            log::error!("Failed to send request: {:?}", e);
            Err(Box::new(e))
        }
    }
}
