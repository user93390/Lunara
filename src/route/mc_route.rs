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

use crate::http::plugin_fetch::{TrendingPlugin, fetch_plugin_versions, fetch_trending_plugins};
use crate::http::server_creator::ServerCreator;
use crate::mc::plugin::Plugin;
use crate::mc::server::ServerBrand::Vanilla;
use crate::mc::server::{BuildInfo, MinecraftServer, ServerBrand};
use crate::route::route_error::RouteError;
use crate::route::route_error::RouteError::{InternalError, NotFound};
use axum::Json;
use axum::Router;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::get;
use core::str::from_utf8;
use log::{debug, error, info, warn};
use reqwest::Client;
use serde::Deserialize;
use tokio::fs::{File, create_dir_all};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CONF_LOCATION: &str = "servers.json";

#[derive(Deserialize)]
struct TrendingQuery {
	trending: Option<usize>,
}

async fn load_server_by_name(name: &str) -> Result<MinecraftServer, RouteError> {
	let path = std::path::Path::new(CONF_LOCATION);
	let mut file = File::open(path)
		.await
		.map_err(|e| InternalError(format!("failed to open servers.json: {e}")))?;

	let mut contents = vec![];
	file.read_to_end(&mut contents)
		.await
		.map_err(|e| InternalError(format!("failed to read servers.json: {e}")))?;

	let servers: Vec<MinecraftServer> = serde_json::from_slice(&contents)
		.map_err(|e| InternalError(format!("failed to parse servers.json: {e}")))?;

	servers
		.into_iter()
		.find(|s| s.name() == name)
		.ok_or_else(|| NotFound(format!("server '{}' not found", name)))
}

pub(crate) fn mc_route() -> Router {
	Router::new()
		.route("/plugin/trending", get(trending_plugins))
		.route("/server/list", get(servers))
		.route("/server/create/{brand}/{version}", get(create_server))
		.route(
			"/server/create/{brand}/{version}/{name}",
			get(create_server_with_name),
		)
		.route("/server/start/{server}", get(start_server))
		.route(
			"/server/{server}/add/{plugin}/{version}",
			get(add_plugin_to_server),
		)
		.route("/server/{server}/delete", get(delete_server))
		.route("/server/{server}/plugin/list", get(get_plugins_from_server))
		.route("/server/{server}/logs", get(get_log))
		.route("/plugin/versions/{plugin}", get(get_versions))
}

const MAX_VERSIONS: i16 = 25;

#[axum::debug_handler]
async fn get_versions(Path(plugin): Path<String>) -> Result<Json<Vec<String>>, RouteError> {
	let versions = fetch_plugin_versions(&Client::new(), &plugin, MAX_VERSIONS).await;

	match versions {
		Ok(v) => Ok(Json(v)),
		Err(error) => {
			error!("Error fetching versions for {}. {}", plugin, error);
			Ok(Json(vec![]))
		}
	}
}

#[axum::debug_handler]
async fn trending_plugins(
	Query(query): Query<TrendingQuery>,
) -> Result<Json<Option<Vec<TrendingPlugin>>>, RouteError> {
	let page = query.trending.unwrap_or(1);
	let plugins = fetch_trending_plugins(&Client::new(), page).await;

	match plugins {
		Ok(a) => Ok(Json(Some(a))),
		Err(error) => {
			error!("Error occurred while fetching plugins. {}", error);
			Ok(Json(None))
		}
	}
}

#[axum::debug_handler]
async fn servers() -> Result<Json<String>, (StatusCode, String)> {
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
async fn get_log(
	Query(distance): Query<usize>,
	Path(server_name): Path<String>,
) -> Result<String, RouteError> {
	let mut server = load_server_by_name(&server_name).await?;

	if let Err(error) = server.refresh_log_cache().await {
		error!("Error refreshing log cache: {}", error);
	}

	if let Some(chunk) = server.log_chunks(1024 * distance).next() {
		return Ok(String::from(chunk));
	}

	Ok(String::new())
}

#[axum::debug_handler]
async fn start_server(Path(server_name): Path<String>) -> Result<String, RouteError> {
	let server = load_server_by_name(&server_name).await?;

	match MinecraftServer::turn_on(&server).await {
		Ok(_) => {
			info!("Starting Minecraft server {}", server.name());

			Ok(String::from("Started server..."))
		}
		Err(error) => {
			error!("Error trying to run Minecraft server.");

			Ok(format!("Cause: {}", error))
		}
	}
}

#[axum::debug_handler]
async fn add_plugin_to_server(
	Path((server_name, plugin_name, plugin_version)): Path<(String, String, String)>,
) -> Result<(), RouteError> {
	let server = load_server_by_name(&server_name).await?;

	if server.brand().eq(&Vanilla) {
		warn!("Cannot add plugins to vanilla server");

		return Err(InternalError(String::from(
			"Cannot download plugins on a vanilla server. Try using a paper server?",
		)));
	}

	let plugin = Plugin::new(plugin_name.clone(), plugin_version);

	match MinecraftServer::add_plugin(&server, &plugin).await {
		Ok(_) => {
			info!("Downloading plugin {}", plugin.name())
		}

		Err(error) => {
			error!(
				"Error trying to download plugin {}. {}",
				plugin.name(),
				error
			);

			return Err(InternalError(String::from(
				"Plugin doesn't exist or something else is wrong?",
			)));
		}
	}
	Ok(())
}

#[axum::debug_handler]
async fn get_plugins_from_server(
	Path(server_name): Path<String>,
) -> Result<Json<Option<Vec<Plugin>>>, RouteError> {
	let server = load_server_by_name(&server_name).await?;

	match server.plugins() {
		None => Ok(Json(None)),

		Some(vec) => {
			let result = Some(vec);

			Ok(Json(result.cloned()))
		}
	}
}

#[axum::debug_handler]
async fn create_server_with_name(
	Path((brand, version, name)): Path<(ServerBrand, String, String)>,
) -> Result<(), RouteError> {
	create_server_inner(brand, version, Some(name)).await
}

#[axum::debug_handler]
async fn create_server(
	Path((brand, version)): Path<(ServerBrand, String)>,
) -> Result<(), RouteError> {
	create_server_inner(brand, version, None).await
}

async fn create_server_inner(
	brand: ServerBrand,
	version: String,
	name: Option<String>,
) -> Result<(), RouteError> {
	let mut new_server = MinecraftServer::new();

	new_server
		.with_brand(brand)
		.with_version(BuildInfo { version })
		.build();

	let path_fmt_str: &String = &format!("/server/{}", name.clone().unwrap());
	let path: &str = path_fmt_str.as_str();

	if let Err(e) = create_dir_all(path).await {
		error!("Error creating directory: {}", e)
	}

	let jar_name: String = match new_server.try_download(path).await {
		Ok(file_name) => {
			info!("Downloaded jar!");
			file_name
		}

		Err(error) => {
			error!("Error trying to download_url server jar file.");
			error!("Ensure you did use a valid version");

			debug!("{}", error);

			return Err(InternalError(String::from(
				"Cannot download version. Did you type it correctly?",
			)));
		}
	};

	let server_name = name.unwrap_or(jar_name);
	new_server.with_name(Some(server_name.clone()));

	info!("Creating new server {}...", server_name);

	let path = std::path::Path::new(CONF_LOCATION);
	let mut file = File::open(path)
		.await
		.map_err(|e| InternalError(format!("failed to create servers.json: {e}")))?;

	let json_str = serde_json::to_string(&new_server)
		.map_err(|_| InternalError(String::from("serialization failed")))?;

	file.write_all(json_str.as_bytes())
		.await
		.map_err(|e| InternalError(format!("failed to write file: {e}")))?;

	Ok(())
}

#[axum::debug_handler]
async fn delete_server(Path(server_name): Path<String>) -> Result<(), RouteError> {
	let server = load_server_by_name(&server_name).await?;
	let result = server.delete().await;

	if let Err(error) = result {
		error!("Error deleting server: {}", error)
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::body::Body;
	use axum::http::{Request, StatusCode};
	use axum::response::Response;
	use tower::ServiceExt;

	#[tokio::test]
	async fn mc_route_has_trending_plugins_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/plugin/trending")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_server_list_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/server/list")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_create_server_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/server/create/Paper/testserver/1.20.4")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_returns_404_for_unknown_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/nonexistent")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn trending_plugins_accepts_query_param() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/plugin/trending?trending=2")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_server_start_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/server/start/testserver")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_server_delete_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/server/testserver/delete")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_plugin_list_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/server/testserver/plugin/list")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_server_logs_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/server/testserver/logs?distance=10")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn mc_route_has_plugin_versions_route() {
		let app = mc_route();

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/plugin/versions/testplugin")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}
}
