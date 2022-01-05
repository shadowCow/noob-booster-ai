use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
struct BestActionResponse {
    action: Option<Vec<u8>>,
}

impl Responder for BestActionResponse {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(
            Ok(
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)
            )
        )
    }
}

pub async fn find_best_action() -> impl Responder {
    BestActionResponse {
        action: None
    }
}



// #[post("/echo")]
// pub async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
