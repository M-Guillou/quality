mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_pool = db::get_pool().await.expect("DB connection failed");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(handlers::home)
            .service(handlers::show_add_form)
            .service(handlers::add_client)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
