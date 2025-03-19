use actix_web::{ web, HttpResponse, Responder};
use crate::state::AppState;
use serde::Deserialize;
use crate::gpt::{request_gpt, GptRequestParams, GptMessage};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;


#[derive(Deserialize)]
pub struct RequestParams {
    pub messages: Vec<GptMessage>,
}



async fn send_request<'a>(data: web::Data<AppState<'a>>, params: web::Json<RequestParams>) -> impl Responder {
    let cancel_request = Arc::new(AtomicBool::new(false));
    let settings = &data.settings.gpt;
    let gpt_params = GptRequestParams {
        url: settings.url.as_str(),
        api_key: settings.api_key.as_str(),
        max_tokens: settings.max_tokens,
        model: settings.model.as_str(),
        temperature: settings.temperature,
        messages: &params.messages,
        cancel_request: &cancel_request,
    };

    match request_gpt(gpt_params).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/send_request", web::post().to(send_request));
}
