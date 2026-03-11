pub(crate) mod create;
pub(crate) mod login;

use crate::{database::Database, keyring_service::KeyringService};
use std::error::Error;

pub trait AuthApi {
	async fn get_database(&self) -> Result<Database, Box<dyn Error + Sync + Send>> {
		let keyring_service: KeyringService = KeyringService::new("Lunara");

		// Same thing as main.rs
		let host = keyring_service.get_secret("db.host").await?;
		let port = keyring_service.get_secret("db.port").await?;
		let name = keyring_service.get_secret("db.name").await?;
		let user = keyring_service.get_secret("db.user").await?;
		let password = keyring_service.get_secret("db.password").await?;

		let connection_string: String = format!(
			"postgres://{}:{}@{}:{}/{}",
			user, password, host, port, name
		);

		let db = Database::connect(&connection_string).await.unwrap();

		Ok(db)
	}
}
