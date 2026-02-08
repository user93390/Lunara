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
use crate::api::authentication::Authentication;
use crate::api::authentication::signup::SignupAuth;
use crate::database::Database;
use crate::entity::accounts::{Column, Entity};
use axum::http::StatusCode;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

pub struct LoginAuth {
	pub(crate) uuid: Uuid,
	pub(crate) password: Vec<u8>,
	pub(crate) db: Arc<Database>,
}

impl Authentication for LoginAuth {
	async fn await_login(
		&self,
		auth: LoginAuth,
	) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		let auth_pw_str = &String::from_utf8(auth.password)?;

		let account = Entity::find()
			.filter(Column::Uid.eq(auth.uuid))
			.one(self.db.conn())
			.await?
			.unwrap();

		if account.password.eq(auth_pw_str) {
			return Ok(StatusCode::OK);
		}

		Ok(StatusCode::UNAUTHORIZED)
	}

	async fn await_signup(
		&self,
		_auth: SignupAuth,
	) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		Ok(StatusCode::NOT_IMPLEMENTED)
	}
}
