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

use crate::database::Database;
use crate::entity::accounts::{Column, Entity};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn user_api(database: Database) -> Router {
	Router::new()
		.route("/users", get(users))
		.with_state(Arc::new(database.clone()))
		.route("/users/search/{uuid}", get(search_user))
		.with_state(Arc::new(database))
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
	use axum::body::Body;
	use axum::http::{Request, StatusCode};
	use axum::response::Response;
	use tower::ServiceExt;

	#[tokio::test]
	async fn user_api_router_has_users_route() {
		let db = mock_database().await;

		if let Some(db) = db {
			let app = user_api(db).await;

			let response: Response = app
				.oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
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

			let response: Response = app
				.oneshot(
					Request::builder()
						.uri(&format!("/users/search/{}", test_uuid))
						.body(Body::empty())
						.unwrap(),
				)
				.await
				.unwrap();

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

	async fn mock_database() -> Option<Database> {
		Database::connect("postgres://postgres:postgres@localhost:5432/lunara")
			.await
			.ok()
	}
}
