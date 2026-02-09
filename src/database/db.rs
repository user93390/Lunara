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
use log::{info, warn};
use sea_orm::{Database as SeaDatabase, DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct Database {
	conn: DatabaseConnection,
}

impl Database {
	pub async fn connect(conn_str: &str) -> Result<Self, DbErr> {
		let db_conn: Result<DatabaseConnection, DbErr> = SeaDatabase::connect(conn_str).await;

		match db_conn {
			Ok(_) => {
				info!("Address fond!")
			}
			Err(_) => {
				warn!("Database hasn't started yet or doesn't exist.")
			}
		}

		Ok(Self { conn: db_conn? })
	}

	pub fn conn(&self) -> &DatabaseConnection {
		&self.conn
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn connect_fails_with_invalid_connection_string() {
		let result = Database::connect("invalid://not-a-real-db").await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn connect_fails_with_empty_string() {
		let result = Database::connect("").await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn connect_fails_with_unreachable_host() {
		let result = Database::connect("postgres://user:pass@127.0.0.1:59999/nonexistent").await;
		assert!(result.is_err());
	}
}
