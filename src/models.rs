use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct Client {
    pub client_id: String,
    pub name: String,
    pub first_name: String,
    pub birth_date: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::clients)]
pub struct NewClient {
    pub client_id: String,
    pub name: String,
    pub first_name: String,
    pub birth_date: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub name: String,
    pub first_name: String,
}

impl NewClient {
    pub fn new(
        name: String,
        first_name: String,
        birth_date: String,
        address: String,
        postal_code: String,
        city: String,
    ) -> Self {
        NewClient {
            client_id: Uuid::new_v4().to_string(),
            name,
            first_name,
            birth_date,
            address,
            postal_code,
            city,
        }
    }
}