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

use crate::http::plugin_fetch::{TrendingPlugin, fetch_trending_plugins};
use crate::mc::server::{BuildInfo, MinecraftServer, ServerBrand};
use axum::Json;
use axum::Router;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::get;
use core::str::from_utf8;
use log::info;
use reqwest::Client;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CONF_LOCATION: &str = "servers.json";

#[derive(Deserialize)]
struct TrendingQuery {
	trending: Option<usize>,
}

pub(crate) fn mc_route() -> Router {
	Router::new()
		.route("/plugin/trending", get(trending_plugins))
		.route("/server/list", get(servers))
		.route(
			"/server/create/{brand}/{name}/{version}",
			get(create_server),
		)
}

#[axum::debug_handler]
async fn trending_plugins(
	Query(query): Query<TrendingQuery>,
) -> Result<Json<Vec<TrendingPlugin>>, (StatusCode, String)> {
	let page = query.trending.unwrap_or(1);
	let plugins = fetch_trending_plugins(&Client::new(), page)
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	Ok(Json(plugins))
}

#[axum::debug_handler]
async fn servers() -> Result<Json<String>, (StatusCode, String)> {
	info!("Listing servers...");

	let path: &std::path::Path = std::path::Path::new(CONF_LOCATION);
	let mut file: File = File::open(path)
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	let mut contents: Vec<u8> = vec![];
	file.read_to_end(&mut contents).await.unwrap();

	let cont: &str = from_utf8(&contents).unwrap();

	Ok(Json(cont.parse().unwrap()))
}

#[axum::debug_handler]
async fn create_server(
	Path((brand, name, version)): Path<(ServerBrand, String, String)>,
) -> Result<(), (StatusCode, String)> {
	info!("Creating new server {}...", name);

	let mut new_server = MinecraftServer::new();

	new_server
		.with_name(Some(name))
		.with_brand(brand)
		.with_version(BuildInfo { version })
		.build();

	let path: &std::path::Path = std::path::Path::new(CONF_LOCATION);

	let mut file = File::create(path)
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	let json_str = serde_json::to_string_pretty(&new_server)
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	file.write_all(json_str.as_bytes())
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	Ok(())
}

#[axum::debug_handler]
async fn delete_server(Path(name): Path<String>) {
	// find & delete on I/O

	todo!()
}
