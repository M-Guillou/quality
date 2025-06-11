use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use askama::Template;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Client {
    pub id: i32,
    pub nom: String,
    pub prenom: String,
    pub date_naissance: NaiveDate,
    pub adresse: String,
    pub code_postal: String,
    pub ville: String,
}

#[derive(Debug, Deserialize)]
pub struct NewClient {
    pub nom: String,
    pub prenom: String,
    pub date_naissance: NaiveDate,
    pub adresse: String,
    pub code_postal: String,
    pub ville: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct ClientListTemplate {
    pub clients: Vec<Client>,
}

#[derive(Template)]
#[template(path = "form.html")]
pub struct ClientFormTemplate;
