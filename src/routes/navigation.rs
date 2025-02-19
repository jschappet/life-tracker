use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/manual_hello")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
async fn redirect_to_index() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/s/"))
        .finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
       .service(echo)
       .service(manual_hello)
       .service(redirect_to_index);
}
