use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

mod schema;
mod models;

use models::{Client, NewClient};

fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn create_client(
    client: web::Json<NewClient>,
    db: web::Data<SqliteConnection>,
) -> impl Responder {
    use schema::clients::dsl::*;

    let new_client = client.into_inner();
    diesel::insert_into(clients)
        .values(&new_client)
        .execute(&mut *db)
        .expect("Error inserting client");

    HttpResponse::Ok().json("Client created")
}

async fn get_client(
    query: web::Query<models::SearchQuery>,
    db: web::Data<SqliteConnection>,
) -> impl Responder {
    use schema::clients::dsl::*;

    let results = clients
        .filter(name.ilike(format!("%{}%", query.name)))
        .filter(first_name.ilike(format!("%{}%", query.first_name)))
        .load::<Client>(&mut *db)
        .expect("Error loading clients");

    HttpResponse::Ok().json(results)
}

async fn update_client(
    id: web::Path<String>,
    client: web::Json<NewClient>,
    db: web::Data<SqliteConnection>,
) -> impl Responder {
    use schema::clients::dsl::*;

    let client_data = client.into_inner();
    diesel::update(clients.filter(client_id.eq(id.to_string())))
        .set((
            name.eq(client_data.name),
            first_name.eq(client_data.first_name),
            birth_date.eq(client_data.birth_date),
            address.eq(client_data.address),
            postal_code.eq(client_data.postal_code),
            city.eq(client_data.city),
        ))
        .execute(&mut *db)
        .expect("Error updating client");

    HttpResponse::Ok().json("Client updated")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = establish_connection();
    diesel::sql_query(
        "CREATE TABLE IF NOT EXISTS clients (
            client_id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            first_name TEXT NOT NULL,
            birth_date TEXT NOT NULL,
            address TEXT NOT NULL,
            postal_code TEXT NOT NULL,
            city TEXT NOT NULL
        )",
    )
    .execute(&mut db)
    .expect("Error creating table");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(establish_connection()))
            .route("/clients", web::post().to(create_client))
            .route("/clients", web::get().to(get_client))
            .route("/clients/{id}", web::put().to(update_client))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}