/*
Copyright 2026 seasnail1

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/

pub(crate) mod create;
pub(crate) mod login;

use crate::{
	database::Database,
	keyring_service::KeyringService,
};
use std::error::Error;

pub trait AuthApi {
	async fn get_database(&self) -> Result<Database, Box<dyn Error + Sync + Send>> {
		let keyring_service: KeyringService = KeyringService::new("Lunara");

		let connection_string = match keyring_service.get_secret("connection_string").await {
			Ok(connection) => connection,
			Err(_) => {
				let host = keyring_service.get_secret("db.host").await?;
				let port = keyring_service.get_secret("db.port").await?;
				let name = keyring_service.get_secret("db.name").await?;
				let user = keyring_service.get_secret("db.user").await?;
				let password = keyring_service.get_secret("db.password").await?;

				format!(
					"postgres://{}:{}@{}:{}/{}",
					user, password, host, port, name
				)
			}
		};

		Ok(Database::connect(&connection_string).await?)
	}
}

#[cfg(test)]
mod tests {
	use super::{
		AuthApi,
		create::CreateStruct,
		login::LoginStruct,
	};

	#[test]
	fn auth_types_implement_auth_api() {
		fn assert_auth_api<T: AuthApi>() {}

		assert_auth_api::<CreateStruct<'static>>();
		assert_auth_api::<LoginStruct>();
	}
}
