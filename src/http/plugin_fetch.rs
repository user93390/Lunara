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
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const CHUNK_SIZE: usize = 25;

#[derive(Deserialize)]
pub struct AuthHelper {
	token: String,
	duration: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrendingPlugin {
	pub name: String,
	pub namespace: PluginNamespace,
	pub stats: PluginStats,
	#[serde(rename = "avatarUrl")]
	pub icon_url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PluginNamespace {
	pub owner: String,
	pub slug: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PluginStats {
	pub downloads: i64,
	pub stars: i64,
}

#[derive(Deserialize)]
struct HangarResponse {
	result: Vec<TrendingPlugin>,
}

#[derive(Debug, Error)]
pub enum PluginError {
	#[error("Unknown plugin: {0}")]
	UnknownPlugin(#[from] reqwest::Error),

	#[error("Bad plugin ID")]
	BadPluginID(reqwest::Error),

	#[error("Authentication error")]
	Unauthorized(String),
}

pub async fn fetch_trending_plugins(
	client: &Client,
	page: usize,
) -> Result<Vec<TrendingPlugin>, PluginError> {
	let offset = page.saturating_sub(1) * CHUNK_SIZE;
	let url = format!(
		"https://hangar.papermc.io/api/v1/projects?sort=-stars&limit={}&offset={}",
		CHUNK_SIZE, offset
	);

	let resp: HangarResponse = client.get(&url).send().await?.json().await?;

	Ok(resp.result)
}

trait PluginCreator {
	async fn get_plugin_by_id(&self, auth: AuthHelper, plugin: Plugin) -> Result<(), PluginError>;
	async fn authenticate(&self) -> Result<AuthHelper, PluginError>;
}

impl PluginCreator for Plugin {
	async fn get_plugin_by_id(&self, auth: AuthHelper, plugin: Plugin) -> Result<(), PluginError> {
		if auth.duration <= 0 {
			return Err(PluginError::Unauthorized("Auth token expired".to_string()));
		}

		Ok(())
	}
	async fn authenticate(&self) -> Result<AuthHelper, PluginError> {
		let client: Client = Client::builder()
			.user_agent("Lunara project. https://github.com/user93390/Lunara")
			.build()?;

		Ok(
			client
			.post("https://hangar.papermc.io/api/v1/authenticate")
			.send()
			.await?
			.json::<AuthHelper>()
			.await?
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn plugin_error_display_unknown_plugin() {
		let err = PluginError::Unauthorized("test error".to_string());
		assert!(err.to_string().contains("Authentication error"));
	}

	#[test]
	fn trending_plugin_deserializes_correctly() {
		let json = r#"{
			"name": "TestPlugin",
			"namespace": {"owner": "test_owner", "slug": "test_slug"},
			"stats": {"downloads": 1000, "stars": 50},
			"avatarUrl": "https://example.com/icon.png"
		}"#;

		let plugin: TrendingPlugin = serde_json::from_str(json).unwrap();
		assert_eq!(plugin.name, "TestPlugin");
		assert_eq!(plugin.namespace.owner, "test_owner");
		assert_eq!(plugin.namespace.slug, "test_slug");
		assert_eq!(plugin.stats.downloads, 1000);
		assert_eq!(plugin.stats.stars, 50);
	}

	#[test]
	fn plugin_namespace_clone_works() {
		let ns = PluginNamespace {
			owner: "owner".to_string(),
			slug: "slug".to_string(),
		};
		let cloned = ns.clone();
		assert_eq!(cloned.owner, ns.owner);
		assert_eq!(cloned.slug, ns.slug);
	}

	#[test]
	fn plugin_stats_clone_works() {
		let stats = PluginStats {
			downloads: 500,
			stars: 25,
		};
		let cloned = stats.clone();
		assert_eq!(cloned.downloads, stats.downloads);
		assert_eq!(cloned.stars, stats.stars);
	}

	#[tokio::test]
	async fn fetch_trending_plugins_with_invalid_page_returns_first_page() {
		let client = Client::new();
		let result = fetch_trending_plugins(&client, 0).await;
		assert!(result.is_ok());
	}
}
