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
use crate::mc::plugin::Plugin;
use log::info;
use reqwest::{
	Client,
	Response,
};
use serde::{
	Deserialize,
	Serialize,
};
use std::{
	error::Error,
	fs::File,
	io::Write,
};
use tokio::{
	fs::{
		create_dir_all,
		read_to_string,
		remove_dir_all,
	},
	process::Command,
};

#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq)]
pub enum ServerBrand {
	Vanilla,
	Paper,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct BuildInfo {
	pub(crate) version: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MinecraftServer {
	brand: ServerBrand,
	build: BuildInfo,
	name: Option<String>,
	plugins: Option<Vec<Plugin>>,
	#[serde(skip)]
	log_cache: Option<String>,
}

pub(crate) const PARENT_PATH: &str = "/app/servers";

impl MinecraftServer {
	pub(crate) fn new() -> Self {
		Self {
			brand: ServerBrand::Vanilla,
			build: BuildInfo {
				version: String::from("NaN"),
			},
			name: None,
			plugins: None,
			log_cache: None,
		}
	}
	pub(crate) async fn add_plugin(
		&self,
		plugin: &Plugin,
	) -> Result<(), Box<dyn Error + Sync + Send>> {
		let plugins_dir = format!("{}/plugins", self.directory());
		create_dir_all(&plugins_dir).await?;

		let url: String = plugin.download_url();
		let dest: String = format!("{}/{}.jar", plugins_dir, plugin.name());

		let response: Response = Client::new().get(&url).send().await?;
		let bytes = response.bytes().await?;

		let mut file: File = File::create(&dest)?;
		file.write_all(&bytes)?;

		info!("Plugin downloaded to {}", dest);
		Ok(())
	}

	pub(crate) async fn turn_on(&self) -> Result<(), Box<dyn Error + Sync + Send>> {
		Command::new("java")
			.arg("-jar")
			.arg(format!("{}.jar", self.name()))
			.current_dir(self.directory())
			.spawn()?;

		Ok(())
	}

	pub(crate) async fn delete(&self) -> Result<(), Box<dyn Error + Sync + Send>> {
		let path_str: &String = &self.directory();

		remove_dir_all(path_str).await?;
		Ok(())
	}

	pub(crate) async fn delete_plugin(
		&self,
		target: Plugin,
	) -> Result<(), Box<dyn Error + Sync + Send>> {
		let path_str: &String = &format!("{}/plugins/{}.jar", &self.directory(), target.name());

		remove_dir_all(path_str).await?;
		Ok(())
	}

	pub(crate) async fn refresh_log_cache(&mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
		let log_path = format!("{}/logs/latest.log", self.directory());
		let content = read_to_string(&log_path).await?;
		self.log_cache = Some(content);

		Ok(())
	}

	pub(crate) fn log_chunks(&self, chunk_size: usize) -> impl Iterator<Item = &str> {
		self.log_cache
			.as_deref()
			.unwrap_or("")
			.as_bytes()
			.chunks(chunk_size)
			.map(|chunk| std::str::from_utf8(chunk).unwrap_or(""))
	}

	pub(crate) fn with_version(&mut self, version: BuildInfo) -> &mut Self {
		self.build = version;
		self
	}

	pub(crate) fn with_name(&mut self, name: Option<String>) -> &mut Self {
		self.name = name;
		self
	}

	pub(crate) fn with_brand(&mut self, brand: ServerBrand) -> &mut Self {
		self.brand = brand;
		self
	}

	pub(crate) fn build_info(&self) -> &BuildInfo {
		&self.build
	}

	pub(crate) fn plugins(&self) -> Option<&Vec<Plugin>> {
		self.plugins.as_ref()
	}

	pub(crate) fn name(&self) -> &str {
		self.name.as_deref().unwrap_or("No server name found. NaN")
	}

	pub(crate) fn brand(&self) -> &ServerBrand {
		&self.brand
	}

	pub(crate) fn version(&self) -> &str {
		&self.build.version
	}

	pub(crate) fn build(&self) -> &MinecraftServer {
		self
	}

	pub(crate) fn directory(&self) -> String {
		format!("{}/{}", PARENT_PATH, self.name())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_server_has_defaults() {
		let server = MinecraftServer::new();

		assert!(matches!(server.brand(), ServerBrand::Vanilla));
		assert_eq!(server.version(), "NaN");
		assert_eq!(server.name(), "No server name found. NaN");
		assert!(server.plugins().is_none());
	}

	#[test]
	fn server_builder_with_name() {
		let mut server = MinecraftServer::new();
		server.with_name(Some(String::from("TestServer")));

		assert_eq!(server.name(), "TestServer");
	}

	#[test]
	fn server_builder_with_brand() {
		let mut server = MinecraftServer::new();
		server.with_brand(ServerBrand::Paper);

		assert!(matches!(server.brand(), ServerBrand::Paper));
	}

	#[test]
	fn server_builder_with_version() {
		let mut server = MinecraftServer::new();
		server.with_version(BuildInfo {
			version: String::from("1.20.4"),
		});

		assert_eq!(server.version(), "1.20.4");
	}

	#[test]
	fn server_builder_chain() {
		let mut server = MinecraftServer::new();
		server
			.with_name(Some(String::from("ChainTest")))
			.with_brand(ServerBrand::Paper)
			.with_version(BuildInfo {
				version: String::from("1.19.2"),
			});

		assert_eq!(server.name(), "ChainTest");
		assert!(matches!(server.brand(), ServerBrand::Paper));
		assert_eq!(server.version(), "1.19.2");
	}

	#[test]
	fn server_build_returns_self() {
		let server = MinecraftServer::new();
		let built = server.build();

		assert_eq!(built.version(), server.version());
	}

	#[test]
	fn build_info_version() {
		let mut server = MinecraftServer::new();
		server.with_version(BuildInfo {
			version: String::from("1.21"),
		});

		assert_eq!(server.build_info().version, "1.21");
	}

	#[test]
	fn server_serialize_deserialize() {
		let mut server = MinecraftServer::new();
		server
			.with_name(Some(String::from("SerdeServer")))
			.with_brand(ServerBrand::Paper)
			.with_version(BuildInfo {
				version: String::from("1.20.1"),
			});

		let json = serde_json::to_string(&server).unwrap();
		let deserialized: MinecraftServer = serde_json::from_str(&json).unwrap();

		assert_eq!(deserialized.name(), server.name());
		assert_eq!(deserialized.version(), server.version());
	}

	#[test]
	fn server_brand_debug() {
		let vanilla = ServerBrand::Vanilla;
		let paper = ServerBrand::Paper;

		assert_eq!(format!("{:?}", vanilla), "Vanilla");
		assert_eq!(format!("{:?}", paper), "Paper");
	}
}
