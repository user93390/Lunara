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
use log::warn;
use sea_orm::EntityTrait;
use sea_orm::{ColumnTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn try_login(
	State(db): State<Arc<Database>>,
	Path((username, password)): Path<(String, String)>,
) -> Option<Uuid> {

	let account = Entity::find()
		.filter(Column::Username.eq(&username))
		.one(db.conn())
		.await
		.ok()??;

	if account.password != password {
		warn!("Invalid password for user: {}", username);
		return None;
	}

	Some(account.uid)
}
