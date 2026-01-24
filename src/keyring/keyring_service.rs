use keyring::Entry;
use rand::Rng;
use std::error::Error;

use log::error;

use tokio::task::spawn_blocking;

pub struct KeyringService {
	service_name: String,
}

impl KeyringService {
	pub fn new(service_name: impl Into<String>) -> Self {
		Self {
			service_name: service_name.into(),
		}
	}
	pub async fn set_secret(
		&self, key: &str, secret: &str,
	) -> Result<(), Box<dyn Error + Send + Sync>> {
		let service_name: String = self.service_name.clone();
		let key: String = key.to_string();
		let secret: String = secret.to_string();

		spawn_blocking(move || {
			let entry = Entry::new(&service_name, &key)?;
			entry.set_password(&secret)?;
			Ok::<(), keyring::Error>(())
		})
		.await??;

		Ok(())
	}

	pub async fn get_secret(&self, key: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
		let service_name: String = self.service_name.clone();
		let key: String = key.to_string();

		spawn_blocking(move || match Entry::new(&service_name, &key) {
			Ok(entry) => match entry.get_password() {
				Ok(secret) => Ok(secret),
				Err(e) => {
					error!("Unable to get keyring variable: {:?}", e);
					Ok("N/A".to_string())
				}
			},
			Err(e) => {
				panic!("Unable to connect to keyring: {:?}", e);
			}
		})
		.await?
	}

	#[allow(dead_code)]
	pub async fn delete_secret(&self, key: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
		let service_name: String = self.service_name.clone();
		let key: String = key.to_string();

		spawn_blocking(move || {
			let entry = Entry::new(&service_name, &key)?;
			entry.delete_credential()?;
			Ok::<(), keyring::Error>(())
		})
		.await??;
		Ok(())
	}

	pub async fn secret_exists(&self, key: &str) -> bool {
		let service_name: String = self.service_name.clone();
		let key: String = key.to_string();

		spawn_blocking(move || match Entry::new(&service_name, &key) {
			Ok(entry) => entry.get_password().is_ok(),
			Err(_) => false,
		})
		.await
		.unwrap_or(false)
	}

	pub fn generate_key_128() -> [u8; 32] {
		let mut arr: [u8; 32] = [0u8; 32];
		rand::rng().fill(&mut arr[..]);
		arr
	}
}
