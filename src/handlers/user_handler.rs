use actix_web::{web, HttpResponse, Responder};
use scylla::transport::errors::QueryError;
use serde::Deserialize;
use scylla::Session;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::user::{User, UserResponse};
use crate::responses::user_responses::{GenericResponse, SingleUserReponse, UserData, UserListResponse};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
}

pub async fn create_user(
    session: web::Data<Arc<Mutex<Session>>>,
    user: web::Json<CreateUserRequest>
) -> impl Responder {
    let session_clone = Arc::clone(&session);
    let fetch_response = fetch_users(session_clone).await;

    if let Err(e) = fetch_response {
        let response = GenericResponse {
            status: "fail".to_string(),
            message: e.to_string()
        };
        return HttpResponse::InternalServerError().json(response);
    }
    let users = fetch_response.unwrap();
    if users.iter().any(|u| u.email == user.email) {
        let response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Account with email: {} already exists", user.email)
        };
        return HttpResponse::Conflict().json(response);
    }
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
        data: UserData { user: user.to_user_reponse() }
    };
    HttpResponse::Created().json(response)
}


pub async fn get_users(session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let session_clone = Arc::clone(&session);
    let fetch_response = fetch_users(session_clone).await;
    match fetch_response {
        Ok(users) => {
            let response = UserListResponse {
                status: "sucess".to_string(),
                count: users.len(),
                users: users,
            };
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response = GenericResponse {
                status: "fail".to_string(),
                message: e.to_string()
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}


async fn fetch_users(session: Arc<Arc<Mutex<Session>>>) -> Result<Vec<UserResponse>, QueryError> {
    let query = format!("SELECT id, email FROM tutorial.users");

    let session = session.lock().await;
    let result = session.query(query, &[]).await;
    let mut users: Vec<UserResponse> = vec![];

    match result {
        Ok(response) => {
            let rows = response.rows.unwrap_or_default();
            for row in rows {
                let id = row.columns[0].as_ref().unwrap().as_text().unwrap();
                let email = row.columns[1].as_ref().unwrap().as_text().unwrap();
                users.push(UserResponse { id: id.to_owned(), email: email.to_owned()});
            }
            Ok(users)
        },
        Err(e) => {
            Err(e)
        }
    }
}
