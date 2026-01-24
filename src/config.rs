use std::error::Error;
use std::str::from_utf8;

use log::warn;
use serde::{Deserialize, Serialize};
use std::path::Path;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Config {
	config_path: String,
	keyring_key: [u8; 32],
	connection_string: String,
}
impl Config {
	pub(crate) fn default() -> Self {
		Self {
			config_path: "config.toml".to_string(),
			keyring_key: [0u8; 32],
			connection_string: "NaN".to_string(),
		}
	}

	pub(crate) fn build(self) -> Self {
		Self {
			config_path: self.config_path,
			keyring_key: self.keyring_key,
			connection_string: self.connection_string,
		}
	}

	pub(crate) fn key(&self) -> [u8; 32] {
		self.keyring_key
	}

	pub(crate) fn conn_str(&self) -> &String {
		&self.connection_string
	}

	pub(crate) fn cfg_path(&self) -> &String {
		&self.config_path
	}

	pub(crate) fn with_key(&mut self, key: [u8; 32]) -> &mut Self {
		self.keyring_key = key;
		self
	}

	pub(crate) fn with_conn_str(&mut self, conn_str: String) -> &mut Self {
		self.connection_string = conn_str;
		self
	}

	pub(crate) fn with_path(&mut self, path: String) -> &mut Self {
		self.config_path = path;
		self
	}

	pub(crate) async fn get_from_toml(&self) -> Result<Option<Config>, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(&self.config_path);

		let mut file: File = File::open(path).await?;

		let mut contents: Vec<u8> = vec![];
		file.read_to_end(&mut contents).await?;

		let cont: &str = from_utf8(&contents)?;

		if cont.is_empty() {
			warn!("Config file has nothing inside it");

			let mut cfg = Config::default();
			cfg.with_path(String::from("config.toml"));
			return Ok(Some(cfg));
		}

		let config: Config = toml::from_str(cont)?;
		Ok(Some(config))
	}

	pub(crate) async fn write_toml(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(&self.config_path);

		if !path.exists() {
			warn!("file doesn't exist: {}", self.config_path);
			return Ok(false);
		}

		if !path.is_file() {
			warn!("Should be a file name.");
			return Ok(false);
		}

		let toml = toml::to_string(self)?;

		let mut file = File::create(path).await?;
		file.write_all(toml.as_bytes()).await?;

		Ok(true)
	}

	pub fn get_toml(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
		let toml = toml::to_string_pretty(&self)?;

		Ok(toml)
	}
}
