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

use super::AuthApi;
use crate::entity::accounts::{self, Column, Entity};
use axum::{body::Body, http::StatusCode, response::Response};
use log::{info, warn};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::error::Error;
use uuid::Uuid;

pub(crate) struct LoginStruct {
	pub(crate) uuid: Uuid,
	pub(crate) password: String,
}

impl AuthApi for LoginStruct {}

impl LoginStruct {
	pub async fn login(&self) -> Result<Response, Box<dyn Error + Sync + Send>> {
		let database = self.get_database().await?;

		let result = Entity::find()
			.filter(Column::Uid.eq(self.uuid))
			.one(database.conn())
			.await?;

		match result {
			Some(account) => {
				if account.password.eq(&self.password) {
					info!("Authenticated!");

					return Ok(Response::builder()
						.status(StatusCode::ACCEPTED)
						.body(Body::from("Authorized"))
						.unwrap_or_else(|_| Response::new(Body::from("Authorized"))));
				}
			}

			None => {
				warn!("No account found.")
			}
		}

		Ok(Response::builder()
			.status(StatusCode::BAD_REQUEST)
			.body(Body::from("Unable to login"))
			.unwrap_or_else(|_| Response::new(Body::from("Unable to login"))))
	}
}
