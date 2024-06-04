use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use scylla::Session;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::user::User;
use crate::responses::user_responses::{SingleUserReponse, UserData};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
}

pub async fn check_api() -> impl Responder {
    HttpResponse::Ok().json("API is running")
}

pub async fn create_user(
    session: web::Data<Arc<Mutex<Session>>>,
    user: web::Json<CreateUserRequest>
) -> impl Responder {

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        email: user.email.clone(),
        password: hashed_password,
    };

    let query = format!(
        "INSERT INTO tutorial.users (id, email, password) VALUES ('{}', '{}', '{}')",
        user.id, user.email, user.password
    );

    let session = session.lock().await;
    session.query(query, &[]).await.expect("Failed to insert user");

    let response = SingleUserReponse {
        status: "success".to_string(),
        data: UserData { user }
    };
    HttpResponse::Ok().json(response)
}
