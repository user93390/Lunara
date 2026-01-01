/*
 * Copyright 2025 seasnail1
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate log;
mod account;
mod api;
mod database;
pub(crate) mod routes;

use crate::database::{DB_HOST, DB_NAME, DB_PASSWORD, DB_PORT, DB_USER};
use crate::routes::{api_route, auth_route};
use axum::Router;
use dotenv::dotenv;
use log::{info, LevelFilter};
use std::env;
use std::error::Error;
use axum::routing::get;
use tokio::net::TcpListener;

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    env_logger::builder()
        .format_timestamp_secs()
        .format_level(true)
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Running Lunara.");

    dotenv().ok();

    // Load environment variables.
    info!("Loading environment variables");

    *DB_PASSWORD.lock().await = env::var("PASSWORD")?;
    *DB_HOST.lock().await = env::var("HOST")?;
    *DB_PORT.lock().await = env::var("PORT")?;
    *DB_NAME.lock().await = env::var("NAME")?;
    *DB_USER.lock().await = env::var("USER")?;

    info!("Done loading variables!");
    
    let database = database::database().await;

    info!("Configuring routes");
    
    let api_route = api_route::user_api(database).await;
    let auth_route = auth_route::auth_api().await;

    let app = Router::new()
        .nest("/api", api_route)
        .nest("/auth/v1", auth_route)
        .route("/", get(|| async { "Lunara is running!" }));

    let string_addr = format!("{}:{}", SERVER_ADDR, SERVER_PORT);

    info!("Done! Now serving {}", string_addr);

    let listener = TcpListener::bind(string_addr).await?;
    
    axum::serve(listener, app).await?;
    Ok(())
}
