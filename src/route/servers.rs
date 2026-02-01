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
use crate::minecraft::server::Server;
use serde::Deserialize;

use axum::{Json, Router, http::StatusCode, routing::get};

use log::{info, warn};

use std::{error::Error, path::Path};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Deserialize)]
struct Cache {
	servers: Vec<Server>,
}

async fn try_cache() -> Result<Vec<Server>, Box<dyn Error + Sync + Send>> {
	let cached: &Path = Path::new("cache/servers.toml");

	if !cached.exists() {
		warn!("No cache file found.");
		return Ok(vec![]);
	}

	info!("Found cache file");

	let mut contents: String = String::new();

	let mut file: File = File::open(cached).await?;
	file.read_to_string(&mut contents).await?;

	let cache: Cache = toml::from_str(&contents)?;

	Ok(cache.servers)
}

#[axum::debug_handler]
async fn servers() -> Result<Json<Vec<Server>>, (StatusCode, String)> {
	let servers = try_cache()
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	Ok(Json(servers))
}

pub(crate) async fn server_api() -> Router {
	Router::new().route("/servers", get(servers))
}
