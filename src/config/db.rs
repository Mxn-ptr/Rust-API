use actix_web::web::Data;
use scylla::{Session, SessionBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type DbSession = Data<Arc<Mutex<Session>>>;

pub async fn create_session() -> DbSession {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    let session = SessionBuilder::new()
    .known_node(uri)
    .build()
    .await
    .expect("Failed to connected to ScyllaDB");
    println!("Connecting to database");
    
    Data::new(Arc::new(Mutex::new(session)))
}
