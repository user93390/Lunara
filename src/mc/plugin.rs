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
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Plugin {
	name: String,
	version: String,
}

impl Plugin {
	pub fn new(name: String, version: String) -> Self {
		Self { name, version }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn download_url(&self) -> String {
		format!(
			"https://hangar.papermc.io/api/v1/projects/{}/versions/{}/PAPER/download",
			self.name, self.version
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn plugin_download_url_format() {
		let plugin = Plugin {
			name: String::from("TestPlugin"),
			version: String::from("1.0.0"),
		};

		let url = plugin.download_url();

		assert_eq!(
			url,
			"https://hangar.papermc.io/api/v1/projects/TestPlugin/versions/1.0.0/PAPER/download"
		);
	}

	#[test]
	fn plugin_download_url_with_special_characters() {
		let plugin = Plugin {
			name: String::from("My-Plugin"),
			version: String::from("2.1.3-SNAPSHOT"),
		};

		let url = plugin.download_url();

		assert!(url.contains("My-Plugin"));
		assert!(url.contains("2.1.3-SNAPSHOT"));
	}

	#[test]
	fn plugin_clone() {
		let plugin = Plugin {
			name: String::from("CloneTest"),
			version: String::from("1.0"),
		};

		let cloned = plugin.clone();

		assert_eq!(plugin.download_url(), cloned.download_url());
	}

	#[test]
	fn plugin_serialize_deserialize() {
		let plugin = Plugin {
			name: String::from("SerdePlugin"),
			version: String::from("3.2.1"),
		};

		let json = serde_json::to_string(&plugin).unwrap();
		let deserialized: Plugin = serde_json::from_str(&json).unwrap();

		assert_eq!(plugin.download_url(), deserialized.download_url());
	}
}
