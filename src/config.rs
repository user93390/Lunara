use std::{error::Error, str::from_utf8};

use log::warn;
use serde::{Deserialize, Serialize};
use std::path::Path;

use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},
};

const CONF_LOCATION: &str = "config.toml";

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Config {
	keyring_key: [u8; 32],
	connection_string: String,
}
impl Config {
	pub(crate) fn default() -> Self {
		Self {
			keyring_key: [0u8; 32],
			connection_string: "NaN".to_string(),
		}
	}

	pub(crate) fn key(&self) -> [u8; 32] {
		self.keyring_key
	}

	pub(crate) fn conn_str(&self) -> &String {
		&self.connection_string
	}

	pub(crate) fn with_key(&mut self, key: [u8; 32]) -> &mut Self {
		self.keyring_key = key;
		self
	}

	pub(crate) fn with_conn_str(&mut self, conn_str: String) -> &mut Self {
		self.connection_string = conn_str;
		self
	}

	pub(crate) async fn get_from_toml(
		&self,
	) -> Result<Option<Config>, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(CONF_LOCATION);

		let mut file: File = File::open(path).await?;

		let mut contents: Vec<u8> = vec![];
		file.read_to_end(&mut contents).await?;

		let cont: &str = from_utf8(&contents)?;

		if cont.is_empty() {
			warn!("Config file has nothing inside it");

			let cfg = Config::default();
			return Ok(Some(cfg));
		}

		let config: Config = toml::from_str(cont)?;
		Ok(Some(config))
	}

	pub(crate) async fn write_toml(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(CONF_LOCATION);

		if !path.exists() {
			warn!("file doesn't exist: {}", CONF_LOCATION);
			return Ok(false);
		}

		if !path.is_file() {
			warn!("Should be a file name.");
			return Ok(false);
		}

		let toml = toml::to_string(self)?;

		let mut file = File::create(path).await?;
		file.write_buf(&mut toml.as_bytes()).await?;

		Ok(true)
	}

	pub fn get_toml(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
		let toml = toml::to_string_pretty(&self)?;

		Ok(toml)
	}
}
