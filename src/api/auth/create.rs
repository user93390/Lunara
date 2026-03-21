use super::AuthApi;
use crate::Database;
use crate::entity::accounts::ActiveModel;
use axum::http::StatusCode;
use base64::{
	Engine, alphabet,
	engine::{GeneralPurpose, general_purpose},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

use sea_orm::{ActiveModelTrait, Set};

// lifetime for password & username
// we can keep borrowing it unlike String where we'd have to clone it.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateStruct<'a> {
	uuid: Uuid,
	username: &'a str,
	password: &'a str,
}

impl AuthApi for CreateStruct<'_> {}

impl<'a> CreateStruct<'a> {
	pub fn builder() -> CreateStructBuilder<'a> {
		CreateStructBuilder::default()
	}

	// Requires all credentials to be base64 encoded.
	pub async fn create_account(
		&self,
	) -> Result<(StatusCode, &Self), Box<dyn Error + Sync + Send>> {
		let database: Database = self.get_database().await?;

		let uuid_str = Self::decode(self.uuid)?;
		let username = Self::decode(self.username)?;
		let password = Self::decode(self.password)?;

		let uuid = Uuid::parse_str(uuid_str.as_str())?;

		let new_account = ActiveModel {
			uid: Set(uuid),
			username: Set(username.to_lowercase()),
			password: Set(password.to_string()),
		};

		new_account.insert(database.conn()).await?;

		let tuple = (StatusCode::CREATED, self);
		Ok(tuple)
	}

	pub fn decode<S: AsRef<[u8]>>(encoded: S) -> Result<String, Box<dyn Error + Sync + Send>> {
		let decoded_bytes =
			GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD).decode(encoded)?;

		Ok(String::from_utf8(decoded_bytes)?)
	}
}

#[derive(Default)]
pub(crate) struct CreateStructBuilder<'a> {
	uuid: Option<Uuid>,
	username: Option<&'a str>,
	password: Option<&'a str>,
}

impl<'a> CreateStructBuilder<'a> {
	pub fn uuid(mut self, uuid: Uuid) -> Self {
		self.uuid = Some(uuid);
		self
	}

	pub fn username(mut self, username: &'a str) -> Self {
		self.username = Some(username);
		self
	}

	pub fn password(mut self, password: &'a str) -> Self {
		self.password = Some(password);
		self
	}

	pub fn build(self) -> CreateStruct<'a> {
		CreateStruct {
			uuid: self.uuid.expect("uuid is required"),
			username: self.username.expect("username is required"),
			password: self.password.expect("password is required"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::panic::{AssertUnwindSafe, catch_unwind};

	#[test]
	fn builder_creates_struct_with_expected_values() {
		let uuid = Uuid::new_v4();

		let created = CreateStruct::builder()
			.uuid(uuid)
			.username("TestUser")
			.password("test-password")
			.build();

		assert_eq!(created.uuid, uuid);
		assert_eq!(created.username, "TestUser");
		assert_eq!(created.password, "test-password");
	}

	#[test]
	fn builder_panics_when_uuid_missing() {
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = CreateStruct::builder()
				.username("test-user")
				.password("secret")
				.build();
		}));

		assert!(result.is_err());
	}

	#[test]
	fn builder_panics_when_username_missing() {
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = CreateStruct::builder()
				.uuid(Uuid::new_v4())
				.password("secret")
				.build();
		}));

		assert!(result.is_err());
	}

	#[test]
	fn builder_panics_when_password_missing() {
		let result = catch_unwind(AssertUnwindSafe(|| {
			let _ = CreateStruct::builder()
				.uuid(Uuid::new_v4())
				.username("test-user")
				.build();
		}));

		assert!(result.is_err());
	}
}
