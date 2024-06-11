use actix_web::{web, HttpResponse, Responder};
use scylla::transport::errors::{BadQuery, QueryError};
use serde::Deserialize;
use scylla::Session;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::user::{User, UserResponse};
use crate::responses::user_responses::{SingleUserReponse, UserListResponse};
use crate::responses::generic_responses::GenericResponse;
use crate::utils::enums::Status;


#[derive(Deserialize)]
pub struct UserRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct PasswordRequest {
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    email: Option<String>,

}

async fn fetch_users(session: Arc<Arc<Mutex<Session>>>) -> Result<Vec<UserResponse>, QueryError> {
    let query = format!("SELECT id, email FROM rustdb.users");

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
    let query = format!("SELECT id, email, password FROM rustdb.users WHERE id = '{}'", id);

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
    body: web::Json<UserRequest>
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
    if users.iter().any(|u| u.email == body.email) {
        let response = GenericResponse::new(
            Status::Fail,
            format!("Account with email: {} already exists", body.email)
        );
        return HttpResponse::Conflict().json(response);
    }
    let hashed_password = hash(&body.password, DEFAULT_COST).unwrap();
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        email: body.email.clone(),
        password: hashed_password,
    };

    let query = format!(
        "INSERT INTO rustdb.users (id, email, password) VALUES ('{}', '{}', '{}')",
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

pub async fn login(body: web::Json<UserRequest>, session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let session = session.lock().await;
    let query = format!("SELECT * FROM rustdb.users WHERE email = '{}';", body.email);
    let fetch_user = session.query(query, &[]).await;
    match fetch_user {
        Ok(user) => {
            let wrong_response = GenericResponse::new(
                Status::Fail,
                "Wrong credentials".to_string()
                );
            let rows = user.rows.unwrap_or_default();
            if rows.is_empty() {
                return HttpResponse::Unauthorized().json(wrong_response)
            }
            let id = rows[0].columns[0].as_ref().unwrap().as_text().unwrap();
            let email = rows[0].columns[1].as_ref().unwrap().as_text().unwrap();
            let password = rows[0].columns[2].as_ref().unwrap().as_text().unwrap();
            let verify = verify(&body.password, &password);
            match verify {
                Ok(result) => {
                    if result == false {
                        HttpResponse::Unauthorized().json(wrong_response)
                    } else {
                        let response = SingleUserReponse::new(
                            Status::Success,
                            UserResponse { id: id.to_owned(), email: email.to_owned() }
                        );
                        HttpResponse::Ok().json(response)
                    }
                },
                Err(e) => {
                    let response = GenericResponse::new(
                        Status::Fail,
                        e.to_string()
                    );
                    HttpResponse::InternalServerError().json(response)
                }
            }
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

pub async fn reset_password(path: web::Path<String>, body: web::Json<PasswordRequest>, session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let id = path.into_inner();
    let session = session.lock().await;
    let hashed_password = hash(&body.password, DEFAULT_COST).unwrap();
    let query = format!("UPDATE rustdb.users SET password = '{}' WHERE id = '{}' IF EXISTS;", hashed_password, id);
    let query_result = session.query(query, &[]).await;
    match query_result {
        Ok(result) => {
            if result.rows.unwrap()[0].columns[0].as_ref().unwrap().as_boolean().unwrap() == true {
                let response = GenericResponse::new(
                    Status::Success,
                    "Password has been updated".to_string()
                );
                HttpResponse::Ok().json(response)
            } else {
                let response = GenericResponse::new(
                    Status::Fail,
                    "User not found".to_string()
                );
                HttpResponse::BadRequest().json(response)
            }
        },
        Err(e) => {
            HttpResponse::BadRequest().json(e.to_string())
        }
    }
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

pub async fn update_user(path: web::Path<String>, body: web::Json<UpdateUserRequest>, session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let id = path.into_inner();
    let session_clone = Arc::clone(&session);
    let fetch_response = fetch_user(session_clone, id.to_owned()).await;
    match fetch_response {
        Ok(user) => {
            let email = body.email.as_ref().unwrap_or(&user.email);
            let session = session.lock().await;
            let query = format!("UPDATE rustdb.users SET email = '{}' WHERE id = '{}';", email, id);
            let update = session.query(query, &[]).await;
            match update {
                Ok(_) => {
                    let response = SingleUserReponse::new(
                        Status::Success,
                        UserResponse{ id: user.id, email: email.to_owned() }
                    );
                    return HttpResponse::Ok().json(response)
                },
                Err(e) => {
                    let response = GenericResponse::new(
                        Status::Fail,
                        e.to_string()
                    );
                    return HttpResponse::InternalServerError().json(response)
                }
            }
        },
        Err(e) => {
            let response = GenericResponse::new(
                Status::Fail,
                e.to_string()
            );
            if let QueryError::BadQuery(_) = e {
                return HttpResponse::NotFound().json(response);
            }
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn delete_user(path: web::Path<String>, session: web::Data<Arc<Mutex<Session>>>) -> impl Responder {
    let id = path.into_inner();
    let session = session.lock().await;

    let query = format!("DELETE FROM rustdb.users WHERE id = '{}';", id);

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
