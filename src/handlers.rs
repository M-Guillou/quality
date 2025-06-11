use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{Client, ClientListTemplate, ClientFormTemplate, NewClient};
use askama::Template;

#[get("/")]
pub async fn home(db: web::Data<PgPool>) -> impl Responder {
    let clients = sqlx::query_as::<_, Client>("SELECT * FROM clients ORDER BY id DESC")
        .fetch_all(db.get_ref())
        .await
        .unwrap_or_default();

    let template = ClientListTemplate { clients };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

#[get("/ajouter")]
pub async fn show_add_form() -> impl Responder {
    let template = ClientFormTemplate;
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

#[post("/ajouter")]
pub async fn add_client(
    db: web::Data<PgPool>,
    form: web::Form<NewClient>,
) -> impl Responder {
    let c = form.into_inner();

    println!(
        "INSERT INTO client.clients (nom, prenom, date_naissance, adresse, code_postal, ville) VALUES ('{}', '{}', '{}', '{}', '{}', '{}')",
        c.nom, c.prenom, c.date_naissance, c.adresse, c.code_postal, c.ville
    );

    let result = sqlx::query(
        r#"INSERT INTO "client".clients (nom, prenom, date_naissance, adresse, code_postal, ville)
        VALUES ($1, $2, $3, $4, $5, $6)"#,
    )
    .bind(c.nom)
    .bind(c.prenom)
    .bind(c.date_naissance)
    .bind(c.adresse)
    .bind(c.code_postal)
    .bind(c.ville)
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => println!("Client inséré avec succès"),
        Err(e) => {
            eprintln!("Erreur d'insertion : {:?}", e);
            return HttpResponse::InternalServerError().body("Erreur d'insertion");
        }
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}
