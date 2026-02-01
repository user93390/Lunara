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
use crate::api::auth::Authentication;
use crate::api::auth::login::LoginAuth;
use crate::{database::Database, entity::accounts::ActiveModel};
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, Set};
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct SignupAuth {
	pub(crate) uuid: Uuid,
	pub(crate) nickname: String,
	pub(crate) password: Vec<u8>,
	pub(crate) db: Arc<Database>,
}

impl Authentication for SignupAuth {
	async fn await_login(
		&self, _auth: LoginAuth,
	) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		Ok(StatusCode::NOT_IMPLEMENTED)
	}
	async fn await_signup(
		&self, auth: SignupAuth,
	) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		let uuid = auth.uuid;
		let username = auth.nickname;
		let password = String::from_utf8(auth.password)?;

		let new_account = ActiveModel {
			uid: Set(uuid),
			username: Set(username),
			password: Set(password),
		};

		new_account
			.insert(self.db.conn())
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
			.unwrap();

		Ok(StatusCode::CREATED)
	}
}
