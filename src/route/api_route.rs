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
use crate::mc::server::ServerBrand;

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
