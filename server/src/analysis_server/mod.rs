use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

use crate::games::shut_the_box::{ShutTheBoxAnalyst, State};

#[derive(Deserialize)]
pub struct BestActionRequest {
    dice_value: u8,
    tiles_open: [bool; 9],
}

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

pub async fn find_best_action(info: web::Json<BestActionRequest>, data: web::Data<ShutTheBoxAnalyst>) -> impl Responder {
    println!("called find_best_action {:?} {:?}", info.dice_value, info.tiles_open);
    let state = State::new(info.dice_value, info.tiles_open);
    let best_action_with_value = data.find_best_action(
        &state,
    );
    let (best_action, value) = best_action_with_value.unwrap_or((
        vec![],
        f64::MAX,
    ));

    let action: Option<Vec<u8>> = if best_action.is_empty() {
        None
    } else {
        Some(best_action.iter().map(|t| t.score()).collect())
    };
    
    BestActionResponse {
        action,
    }
}



// #[post("/echo")]
// pub async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// pub async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
