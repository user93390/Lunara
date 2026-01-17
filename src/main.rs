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
mod routes;

use crate::{
    database::Database,
    routes::{api_route, auth_route},
};
use axum::Router;
use log::{info, LevelFilter};
use std::error::Error;
use std::rc::Rc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    color_eyre::install()?;

    env_logger::builder()
        .format_timestamp_secs()
        .format_level(true)
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Running Lunara.");

    info!("Configuring routes");

    let database_rc = Rc::new(database().await?);

    let auth_route: Router<_> = auth_route::auth_api((*database_rc).clone()).await;
    let api_route: Router<_> = api_route::user_api((*database_rc).clone()).await;

    let flutter_dir = ServeDir::new("flutter/build/web");

    let app: Router<_> = Router::new()
        .nest("/auth/v1", auth_route)
        .nest("/api", api_route)
        .fallback_service(flutter_dir);

    let string_addr = format!("{}:{}", SERVER_ADDR, SERVER_PORT);

    let alt_addr = format!("localhost:{}", SERVER_PORT);

    info!("Done! Now serving {} \n Alternatively: {}", string_addr, alt_addr);

    let listener = TcpListener::bind(string_addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn database() -> Result<Database, Box<dyn Error + Send + Sync>> {
    Ok(database::database().await?)
}
