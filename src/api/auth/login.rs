/*
 * Copyright 2026 seasnail1
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

use serde::{Deserialize, Serialize};

use super::AuthApi;
use crate::entity::accounts::{self, Column, Entity};
use axum::{body::Body, http::StatusCode, response::Response};
use log::{info, warn};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(crate) struct LoginStruct {
	pub(crate) uuid: Uuid,
	pub(crate) password: String,
}

impl AuthApi for LoginStruct {}

impl LoginStruct {
	pub fn builder() -> LoginStructBuilder {
		LoginStructBuilder::default()
	}
	pub async fn login(&self) -> Result<(Response, Option<&Self>), Box<dyn Error + Sync + Send>> {
		let database = self.get_database().await?;

		let result = Entity::find()
			.filter(Column::Uid.eq(self.uuid))
			.one(database.conn())
			.await?;

		match result {
			Some(account) => {
				if account.password.eq(&self.password) {
					info!("Authenticated!");

					let tuple = (
						Response::builder()
							.status(StatusCode::ACCEPTED)
							.body(Body::from("Authorized"))
							.unwrap_or_else(|_| Response::new(Body::from("Authorized"))),
						Some(self),
					);

					return Ok(tuple);
				}
			}

			None => {
				warn!("No account found.")
			}
		}
		let tuple = (
			Response::builder()
				.status(StatusCode::BAD_REQUEST)
				.body(Body::from("Unable to login"))
				.unwrap_or_else(|_| Response::new(Body::from("Unable to login"))),
			None,
		);

		Ok(tuple)
	}
}

#[derive(Default)]
pub(crate) struct LoginStructBuilder {
	uuid: Option<Uuid>,
	password: Option<String>,
}

impl LoginStructBuilder {
	pub fn uuid(mut self, uuid: Uuid) -> Self {
		self.uuid = Some(uuid);
		self
	}

	pub fn password(mut self, password: String) -> Self {
		self.password = Some(password);
		self
	}

	pub fn build(self) -> LoginStruct {
		LoginStruct {
			uuid: self.uuid.expect("uuid is required"),
			password: self.password.expect("password is required"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::panic::{AssertUnwindSafe, catch_unwind};

	#[test]
	fn builder_creates_login_with_expected_values() {
		let uuid = Uuid::new_v4();
		let password = String::from("pw-123");

		let login = LoginStruct::builder()
			.uuid(uuid)
			.password(password.clone())
			.build();

		assert_eq!(login.uuid, uuid);
		assert_eq!(login.password, password);
	}

	#[test]
	fn builder_panics_when_uuid_missing() {
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = LoginStruct::builder()
				.password(String::from("password"))
				.build();
		}));

		assert!(result.is_err());
	}

	#[test]
	fn builder_panics_when_password_missing() {
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = LoginStruct::builder().uuid(Uuid::new_v4()).build();
		}));

		assert!(result.is_err());
	}

	#[test]
	fn serde_roundtrip_preserves_fields() {
		let login = LoginStruct::builder()
			.uuid(Uuid::new_v4())
			.password(String::from("my-password"))
			.build();

		let json = serde_json::to_string(&login).expect("serialize login");
		let parsed: LoginStruct = serde_json::from_str(&json).expect("deserialize login");

		assert_eq!(parsed.uuid, login.uuid);
		assert_eq!(parsed.password, login.password);
	}
}
