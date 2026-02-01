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
use crate::api::auth::login::LoginAuth;
use crate::api::auth::signup::SignupAuth;
use crate::api::auth::Authentication;
use crate::database::Database;
use crate::entity::accounts::{Column, Entity, Model};
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use base64::engine::general_purpose;
use base64::{alphabet, engine, Engine};
use log::{info, warn};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use uuid::Uuid;

use axum::body::Body;
use axum::http::StatusCode;

pub(crate) async fn auth_api(db: Database) -> Router {
	Router::new()
		.route("/signup/{uuid}/{username}/{password}", get(signup))
		.route("/login/{username}/{password}", get(login))
		.with_state(Arc::new(db))
}

async fn signup(
	axum::extract::State(db): axum::extract::State<Arc<Database>>,
	Path((uuid_b64, username_b64, password_b64)): Path<(String, String, String)>,
) -> Json<String> {
	let username_bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
		.decode(username_b64)
		.unwrap();

	let username = String::from_utf8(username_bytes).unwrap();

	let password = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
		.decode(password_b64)
		.unwrap();

	let uuid_bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
		.decode(uuid_b64)
		.unwrap();

	let uuid_str = String::from_utf8(uuid_bytes).unwrap();
	let uuid = Uuid::parse_str(&uuid_str).expect("Bad uuid.");

	let signup = SignupAuth {
		uuid,
		password,
		nickname: username,
		db,
	};

	let status = signup.await_signup(signup.clone()).await.unwrap();

	Json(status.to_string())
}

async fn login(
	axum::extract::State(db): axum::extract::State<Arc<Database>>,
	Path((uuid_b64, password_b64)): Path<(String, String)>,
) -> impl IntoResponse {
	let password_bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
		.decode(password_b64)
		.unwrap();

	let uuid_bytes = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
		.decode(uuid_b64)
		.unwrap();

	let uuid_str = String::from_utf8(uuid_bytes).unwrap();
	let uuid = Uuid::parse_str(&uuid_str).expect("Bad uuid.");

	let login = LoginAuth {
		uuid,
		password: password_bytes,
		db: db.clone(),
	};

	info!("Authenticating for {}", login.uuid);

	let account = match Entity::find()
		.filter(Column::Uid.eq(login.uuid))
		.one(db.conn())
		.await
	{
		Ok(Some(acc)) => acc,
		Ok(None) => {
			warn!("Account not found for UUID: {}", login.uuid);
			return Response::builder()
				.status(StatusCode::UNAUTHORIZED)
				.body(Body::from("Authentication failed."))
				.unwrap_or_else(|_| Response::new(Body::from("Authentication failed.")));
		}
		Err(e) => {
			warn!("Database error during account lookup: {}", e);
			return Response::builder()
				.status(StatusCode::INTERNAL_SERVER_ERROR)
				.body(Body::from("Internal server error."))
				.unwrap_or_else(|_| Response::new(Body::from("Internal server error.")));
		}
	};

	let password_str = match String::from_utf8(login.password.clone()) {
		Ok(s) => s,
		Err(_) => {
			warn!("Invalid UTF-8 in password for UUID: {}", login.uuid);
			return Response::builder()
				.status(StatusCode::BAD_REQUEST)
				.body(Body::from("Invalid password format."))
				.unwrap_or_else(|_| Response::new(Body::from("Invalid password format.")));
		}
	};

	if account.password.eq(&password_str) {
		info!("Authorized.");

		return Response::builder()
			.status(StatusCode::ACCEPTED)
			.body(Body::from("Logged in!"))
			.unwrap_or_else(|_| Response::new(Body::from("Logged in!")));
	}

	warn!("Bad credentials.");

	Response::builder()
		.status(StatusCode::UNAUTHORIZED)
		.body(Body::from("Authentication failed."))
		.unwrap_or_else(|_| Response::new(Body::from("Authentication failed.")))
}
