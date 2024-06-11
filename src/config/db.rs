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
    let mut query = "CREATE KEYSPACE IF NOT EXISTS rustdb WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};";
    session.query(query, &[]).await.expect("Failed to create keypace");
    query = "CREATE TABLE IF NOT EXISTS rustdb.users (id text PRIMARY KEY, email text, password text);";
    session.query(query, &[]).await.expect("Failed to create table");
    query = "CREATE INDEX IF NOT EXISTS user_email ON rustdb.users (email);";
    session.query(query, &[]).await.expect("Failed to create index");
    println!("Database initialized");

    
    Data::new(Arc::new(Mutex::new(session)))
}
