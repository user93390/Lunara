/*
 Copyright 2026 seasnail1
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 
 */
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::routing::get;
use axum::Router;
use reqwest::Client;
use serde::Deserialize;
use uuid::Uuid;
use crate::http::plugin_fetch::{fetch_trending_plugins, TrendingPlugin};
use crate::mc::server::{MinecraftServer, ServerBrand};

#[derive(Deserialize)]
struct TrendingQuery {
    trending: Option<usize>,
}

pub(crate) fn mc_route() -> Router {
    Router::new()
        .route("/plugin/trending", get(trending_plugins))
}

#[axum::debug_handler]
async fn trending_plugins(Query(query): Query<TrendingQuery>) -> Result<Json<Vec<TrendingPlugin>>, (StatusCode, String)> {
    let page = query.trending.unwrap_or(1);
    let plugins = fetch_trending_plugins(&Client::new(), page)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(plugins))
}

#[axum::debug_handler]
async fn create_server(Path(brand): Path<ServerBrand>, State(name): State<String>) {
    // create server on I/O

    let new_server = MinecraftServer::new();
}
#[axum::debug_handler]
async fn delete_server(Path(server_id): Path<Uuid>) {
    // find & delete on I/O

    todo!()
}