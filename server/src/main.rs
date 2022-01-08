use actix_web::{web, App, HttpServer};

use rust_game_ai::analysis_server::{
    find_best_action,
};
use rust_game_ai::games::shut_the_box::{ShutTheBoxAnalyst};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8383;
    let address = format!("127.0.0.1:{:?}", port);

    HttpServer::new(|| {
        println!("creating analyst...");
        let analyst = ShutTheBoxAnalyst::new();
        println!("analyst ready!");

        println!("starting worker...");
        App::new()
            .service(
                web::scope("/shut-the-box")
                    .data(analyst)
                    .route("/find-best-action", web::post().to(find_best_action))
            )
    })
    .workers(2)
    .bind(address.to_owned())?
    .run()
    .await
}
