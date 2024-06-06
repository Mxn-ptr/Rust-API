use actix_web::{web, HttpResponse, Responder};
use scylla::transport::errors::{BadQuery, QueryError};
use serde::Deserialize;
use scylla::Session;
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::user::{User, UserResponse};
use crate::responses::user_responses::{SingleUserReponse, UserListResponse};
use crate::responses::generic_responses::GenericResponse;
use crate::utils::enums::Status;


#[derive(Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
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

async fn fetch_user(session: Arc<Arc<Mutex<Session>>>, id: String) -> Result<User, QueryError> {
    let query = format!("SELECT id, email, password FROM tutorial.users WHERE id = '{}'", id);

    let session = session.lock().await;
    let result = session.query(query, &[]).await;

    match result {
        Ok(response) => {
            let rows = response.rows.unwrap_or_default();
            if rows.is_empty() {
                Err(QueryError::BadQuery(BadQuery::Other(format!("User with id: {} not found", id))))
            } else {
                let id = rows[0].columns[0].as_ref().unwrap().as_text().unwrap();
                let email = rows[0].columns[1].as_ref().unwrap().as_text().unwrap();
                let password = rows[0].columns[2].as_ref().unwrap().as_text().unwrap();
                let user = User { id: id.to_owned(), email: email.to_owned(), password: password.to_owned() };
                Ok(user)
            }
        },
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn create_user(
    session: web::Data<Arc<Mutex<Session>>>,
    user: web::Json<CreateUserRequest>
) -> impl Responder {
    let session_clone = Arc::clone(&session);
    let fetch_response = fetch_users(session_clone).await;

    if let Err(e) = fetch_response {
        let response = GenericResponse::new(
            Status::Success,
            e.to_string()
        );
        return HttpResponse::InternalServerError().json(response);
    }
    let users = fetch_response.unwrap();
    if users.iter().any(|u| u.email == user.email) {
        let response = GenericResponse::new(
            Status::Fail,
            format!("Account with email: {} already exists", user.email)
        );
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

    let response = SingleUserReponse::new(
        Status::Success,
        UserResponse{ id: user.id, email: user.email }
    );
    HttpResponse::Created().json(response)
}


pub async fn get_users(session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let session_clone = Arc::clone(&session);
    let fetch_response = fetch_users(session_clone).await;
    match fetch_response {
        Ok(users) => {
            let response = UserListResponse::new(
                Status::Success,
                users.len(),
                users,
            );
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response = GenericResponse::new(
                Status::Fail,
                e.to_string()
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn get_user(path: web::Path<String>, session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let session_clone = Arc::clone(&session);
    let fetch_response = fetch_user(session_clone, path.into_inner()).await;
    match fetch_response {
        Ok(user) => {
            let response = SingleUserReponse::new(
                Status::Success,
                UserResponse{ id: user.id, email: user.email }
            );
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response = GenericResponse::new(
                Status::Fail,
                e.to_string()
            );
            if let QueryError::BadQuery(_) = e {
                return HttpResponse::NotFound().json(response)
            }
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn delete_user(path: web::Path<String>, session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let id = path.into_inner();
    let session = session.lock().await;

    let query = format!("DELETE FROM tutorial.users WHERE id = '{}';", id);

    let result = session.query(query, &[]).await;
    match result {
        Ok(_) => {
            let response = GenericResponse::new(
                Status::Success, 
            "User has been deleted".to_string()
            );
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            let response = GenericResponse::new(
                Status::Fail, 
                e.to_string()
            );
            HttpResponse::NotFound().json(response)
        }
    }
}
