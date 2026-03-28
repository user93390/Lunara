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

use std::{
	error::Error,
	str::from_utf8,
};

use log::{
	info,
	warn,
};
use serde::{
	Deserialize,
	Serialize,
};
use std::path::Path;

use crate::CONFIG;
use tokio::{
	fs::File,
	io::{
		AsyncReadExt,
		AsyncWriteExt,
	},
};

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Config {
	keyring_key: [u8; 32],
	connection_string: String,
	port: u16,
}

impl Config {
	pub(crate) fn default() -> Self {
		Self {
			keyring_key: [0u8; 32],
			connection_string: "NaN".to_string(),
			port: 5050,
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

	pub(crate) fn with_port(&mut self, port: u16) -> &mut Self {
		self.port = port;
		self
	}

	pub(crate) fn with_conn_str(&mut self, conn_str: String) -> &mut Self {
		self.connection_string = conn_str;
		self
	}

	/// Returns 'Some' result.
	/// Returns 'None' result if config is empty.
	pub(crate) async fn get_from_toml(
		&self,
	) -> Result<Option<Config>, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(CONFIG);

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

	/// Returns 'false' if either:
	/// Path doesn't exist or Path IS a folder
	/// Returns true if successful
	pub(crate) async fn write_toml(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
		let path: &Path = Path::new(CONFIG);

		if !path.exists() {
			warn!("file doesn't exist: {}", CONFIG);
			return Ok(false);
		}

		if !path.is_file() {
			warn!("Should be a file name.");
			return Ok(false);
		}

		let toml = toml::to_string(self)?;

		let mut file = File::create(path).await?;

		file.write_buf(&mut toml.as_bytes()).await?;

		info!("Wrote to path.");

		Ok(true)
	}

	/// Returns a prettified version of config file.
	pub fn get_toml(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
		let toml = toml::to_string_pretty(&self)?;

		Ok(toml)
	}
}
