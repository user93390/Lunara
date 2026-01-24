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

use crate::api::auth::login::try_login;
use crate::api::auth::signup::create_account;
use crate::database::Database;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use log::info;
use std::sync::Arc;

pub(crate) async fn auth_api(db: Database) -> Router {
	Router::new()
		.route("/signup/{uuid}/{username}/{password}", get(signup_handler))
		.route("/login/{username}/{password}", get(login_handler))
		.with_state(Arc::new(db))
}

async fn signup_handler(
	State(db): State<Arc<Database>>,
	Path((uuid, username, password)): Path<(String, String, String)>,
) -> Result<StatusCode, StatusCode> {
	let path = Path((
		uuid.parse().map_err(|_| StatusCode::BAD_REQUEST)?,
		username,
		password,
	));

	create_account(State(db), path).await
}

async fn login_handler(
	State(db): State<Arc<Database>>,
	Path((username, password)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {
	let uuid = try_login(State(db), Path((username, password)))
		.await
		.ok_or(StatusCode::BAD_REQUEST)?;

	info!("Logged in as {}!", uuid);

	Ok(StatusCode::OK)
}
