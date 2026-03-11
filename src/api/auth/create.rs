use super::AuthApi;
use crate::Database;
use crate::entity::accounts::ActiveModel;
use axum::http::StatusCode;
use std::error::Error;
use uuid::Uuid;

use sea_orm::{ActiveModelTrait, Set};

// lifetime for password & username
// we can keep borrowing it unlike String where we'd have to clone it.
pub(crate) struct CreateStruct<'a> {
	uuid: Uuid,
	username: &'a str,
	password: &'a str,
}

impl AuthApi for CreateStruct<'_> {}

impl<'a> CreateStruct<'_> {
	pub async fn create_account(&self) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		let database: Database = self.get_database().await?;

		let uuid = self.uuid;
		let username = self.username;
		let password = self.password;

		let new_account = ActiveModel {
			uid: Set(uuid),
			username: Set(username.to_lowercase()),
			password: Set(password.to_string()),
		};

		new_account.insert(database.conn()).await?;

		Ok(StatusCode::CREATED)
	}
}
