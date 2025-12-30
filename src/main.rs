mod api;
mod database;

use crate::api::route;
use crate::database::Database;
use axum::Router;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = database().await;
    let api_route = route::api_users(database);

    let app = Router::new().nest("/api", api_route.await);

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    println!("Closing server");

    Ok(())
}

async fn database() -> Database {
    dotenv().ok();

    let db_host = env::var("HOST").expect("Bad env type");
    let db_port = env::var("PORT").expect("Bad env type");
    let db_name = env::var("NAME").expect("Bad env type");
    let db_user = env::var("USER").expect("Bad env type");
    let db_password = env::var("PASSWORD").expect("Bad env type");

    let connection_string = match db_password {
        pwd if !pwd.is_empty() => format!(
            "host={} port={} dbname={} user={} password={}",
            db_host, db_port, db_name, db_user, pwd
        ),
        _ => format!(
            "host={} port={} dbname={} user={}",
            db_host, db_port, db_name, db_user
        ),
    };

    Database::connect(&connection_string).await.unwrap()
}
