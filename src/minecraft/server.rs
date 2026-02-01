/*
 * Copyright 2026 seasnail1
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

use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use tokio::{
	fs::{File, create_dir_all},
	io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Clone, Deserialize, Serialize)]
pub struct QuickOptions {
	pub whitelist: bool,
	pub command_blocks: bool,
	pub max_players: i32,
}

#[derive(Deserialize, Serialize)]
struct Cache {
	servers: Vec<Server>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Server {
	path: PathBuf,
	name: String,
	options: QuickOptions,
}

impl Server {
	pub(crate) fn new() -> Self {
		Self {
			path: PathBuf::from("NaN"),
			name: String::from("NaN"),
			options: QuickOptions {
				whitelist: false,
				command_blocks: false,
				max_players: 10,
			},
		}
	}

	pub(crate) fn with_path(&mut self, path: &str) -> &mut Self {
		self.path.push(path);
		self
	}

	pub(crate) fn with_name(&mut self, name: String) -> &mut Self {
		self.name = name;
		self
	}

	pub(crate) fn with_options(&mut self, quick_options: QuickOptions) -> &mut Self {
		self.options = quick_options;
		self
	}

	pub(crate) async fn build(&self) -> Self {
		let cached: &Path = Path::new("cache/servers.toml");

		create_dir_all("cache")
			.await
			.expect("Cannot create cache directory.");

		let mut servers: Vec<Server> = if cached.exists() {
			let mut contents = String::new();
			let mut file = File::open(cached).await.unwrap();
			file.read_to_string(&mut contents).await.unwrap();

			toml::from_str::<Cache>(&contents)
				.map(|c| c.servers)
				.unwrap_or_else(|_| vec![])
		} else {
			vec![]
		};

		servers.push(self.clone());

		let cache = Cache { servers };
		let content = toml::to_string(&cache).unwrap();

		let mut file = File::create(cached)
			.await
			.expect("Cannot create cached file.");

		file.write_all(content.as_bytes())
			.await
			.expect("Error writing cached file.");

		self.clone()
	}
}
