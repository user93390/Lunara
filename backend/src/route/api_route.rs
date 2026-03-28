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

use crate::{
	database::Database,
	entity::accounts::{Column, Entity},
	keyring_service::KeyringService,
	route::auth_route::{SESSION_COOKIE_NAME, validate_jwt},
};
use axum_cookie::CookieManager;

use axum::{
	Json, Router,
	extract::{Path, Request, State},
	http::{HeaderMap, StatusCode, header},
	middleware::{Next, from_fn},
	response::Response,
	routing::get,
};
use axum_cookie::CookieLayer;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::{collections::HashMap, sync::Arc};
use tower::limit::ConcurrencyLimitLayer;
use uuid::Uuid;

use log::warn;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

fn api_router() -> Router<Arc<Database>> {
	Router::new()
		.route("/users", get(users))
		.route("/users/search/{uuid}", get(search_user))
		.route("/users/me", get(current_user))
		.route_layer(from_fn(auth))
}

pub(crate) async fn user_api(database: Database) -> Router {
	api_router()
		.with_state(Arc::new(database))
		.layer(CookieLayer::default())
		.layer(ConcurrencyLimitLayer::new(100))
		.layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().level(Level::INFO)))
}

async fn auth(request: Request, next: Next) -> Result<Response, StatusCode> {
	if authorize(request.headers()).await {
		return Ok(next.run(request).await);
	}

	Err(StatusCode::UNAUTHORIZED)
}

async fn authorize(headers: &HeaderMap) -> bool {
	let cookie_header = headers
		.get(header::COOKIE)
		.and_then(|value| value.to_str().ok());

	let Some(cookie_header) = cookie_header else {
		return false;
	};

	for raw_cookie in cookie_header.split(';') {
		let mut parts = raw_cookie.trim().splitn(2, '=');
		let Some(name) = parts.next() else {
			continue;
		};

		let value = parts.next().unwrap_or_default();

		if name != SESSION_COOKIE_NAME {
			continue;
		}

		let keyring = KeyringService::new("lunara");
		let key = match keyring.get_secret("key").await {
			Ok(key) => key,
			Err(error) => {
				warn!("Unable to load key from keyring: {error}");
				return false;
			}
		};

		return match validate_jwt(key.as_bytes(), value) {
			Ok(_) => true,
			Err(_) => {
				warn!("Invalid JWT token");
				false
			}
		};
	}

	false
}

#[axum::debug_handler]
async fn current_user(manager: CookieManager) -> String {
	let token = manager.get(SESSION_COOKIE_NAME).unwrap();
	let key = KeyringService::new("lunara")
		.get_secret("key")
		.await
		.unwrap()
		.into_bytes();

	match validate_jwt(&key, token.value()) {
		Ok(value) => value.uuid.to_string(),
		Err(err) => err.to_string(),
	}
}

#[axum::debug_handler]
async fn users(
	State(db): State<Arc<Database>>,
) -> Result<Json<HashMap<Uuid, String>>, (StatusCode, String)> {
	let mut users: HashMap<Uuid, String> = HashMap::new();

	let accounts = Entity::find()
		.all(db.conn())
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	for account in accounts {
		users.insert(account.uid, account.username);
	}

	Ok(Json(users))
}

#[axum::debug_handler(state = Arc<Database>)]
async fn search_user(
	Path(uuid): Path<Uuid>,
	State(db): State<Arc<Database>>,
) -> Result<Json<HashMap<Uuid, String>>, (StatusCode, String)> {
	let mut users: HashMap<Uuid, String> = HashMap::new();

	let account = Entity::find()
		.filter(Column::Uid.eq(uuid))
		.one(db.conn())
		.await
		.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

	if let Some(account) = account {
		users.insert(account.uid, account.username);
	}

	Ok(Json(users))
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::{
		body::Body,
		http::{Request, StatusCode, header},
		response::Response,
	};
	use tower::ServiceExt;

	fn authorized_request(method: &str, uri: &str) -> Request<Body> {
		Request::builder()
			.method(method)
			.uri(uri)
			.header(
				header::COOKIE,
				format!("{}={}", SESSION_COOKIE_NAME, "test-token"),
			)
			.body(Body::empty())
			.unwrap()
	}

	#[tokio::test]
	async fn user_api_router_has_users_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

			let response: Response = app
				.oneshot(authorized_request("GET", "/users"))
				.await
				.unwrap();

			assert_ne!(response.status(), StatusCode::NOT_FOUND);
		}
	}

	#[tokio::test]
	async fn user_api_router_has_search_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;
			let test_uuid = Uuid::new_v4();
			let uri = format!("/users/search/{}", test_uuid);

			let response: Response = app.oneshot(authorized_request("GET", &uri)).await.unwrap();

			assert_ne!(response.status(), StatusCode::NOT_FOUND);
		}
	}

	#[tokio::test]
	async fn user_api_returns_404_for_unknown_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

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
	}

	#[tokio::test]
	async fn user_api_users_route_returns_json() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

			let response: Response = app
				.oneshot(authorized_request("GET", "/users"))
				.await
				.unwrap();

			assert_eq!(response.status(), StatusCode::OK);
			assert!(response.headers().contains_key(header::CONTENT_TYPE));
			assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
		}
	}

	#[tokio::test]
	async fn user_api_search_route_returns_json() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;
			let test_uuid = Uuid::new_v4();
			let uri = format!("/users/search/{}", test_uuid);

			let response: Response = app.oneshot(authorized_request("GET", &uri)).await.unwrap();

			assert_eq!(response.status(), StatusCode::OK);
			assert!(response.headers().contains_key(header::CONTENT_TYPE));
			assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
		}
	}

	#[tokio::test]
	async fn user_api_users_route_rejects_post() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

			let response: Response = app
				.oneshot(authorized_request("POST", "/users"))
				.await
				.unwrap();

			assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
		}
	}

	#[tokio::test]
	async fn user_api_search_route_rejects_post() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;
			let test_uuid = Uuid::new_v4();
			let uri = format!("/users/search/{}", test_uuid);

			let response: Response = app.oneshot(authorized_request("POST", &uri)).await.unwrap();

			assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
		}
	}

	#[tokio::test]
	async fn user_api_search_route_handles_invalid_uuid() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

			let response: Response = app
				.oneshot(authorized_request("GET", "/users/search/invalid-uuid"))
				.await
				.unwrap();

			assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		}
	}

	#[tokio::test]
	async fn user_api_requires_auth_cookie() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

			let response: Response = app
				.oneshot(
					Request::builder()
						.uri("/users")
						.body(Body::empty())
						.unwrap(),
				)
				.await
				.unwrap();

			assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
		}
	}

	async fn mock_database() -> Option<Database> {
		Database::connect("postgres://postgres:postgres@localhost:5432/lunara")
			.await
			.ok()
	}
}
