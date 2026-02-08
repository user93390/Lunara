use reqwest::Client;
use serde_json::Value;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::mc::server::ServerBrand;
use crate::mc::server::MinecraftServer;

#[derive(Debug, Error)]
pub enum ServerError {
	#[error("Failed to fetch manifest: {0}")]
	ManifestFetch(#[from] reqwest::Error),
	#[error("Invalid manifest format: {0}")]
	InvalidManifest(&'static str),
	#[error("Version not found: {0}")]
	VersionNotFound(String),
	#[error("File operation failed: {0}")]
	FileError(#[from] std::io::Error),
}

trait ServerCreator {
	async fn resolve_paper_url(&self, client: &Client) -> Result<String, ServerError>;
	async fn resolve_vanilla_url(&self, client: &Client) -> Result<String, ServerError>;
	async fn resolve_download_url(&self, client: &Client) -> Result<String, ServerError>;
	async fn try_download(&self, dir_builder: &str) -> Result<(), ServerError>;
}

impl ServerCreator for MinecraftServer {
	async fn resolve_paper_url(&self, client: &Client) -> Result<String, ServerError> {
		let mc_version = &self.build().version();
		let builds_url = format!(
			"https://api.papermc.io/v2/projects/paper/versions/{}/builds",
			mc_version
		);

		let resp: Value = client.get(builds_url).send().await?.json().await?;

		let builds = resp["builds"]
			.as_array()
			.ok_or(ServerError::InvalidManifest("missing builds array"))?;

		let latest_build = builds
			.last()
			.ok_or_else(|| ServerError::VersionNotFound(mc_version.to_string()))?;

		let build_id = latest_build["build"]
			.as_u64()
			.ok_or(ServerError::InvalidManifest("missing build id"))?;

		let file_name = latest_build["downloads"]["application"]["name"]
			.as_str()
			.ok_or(ServerError::InvalidManifest("missing download file name"))?;

		Ok(format!(
			"https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}",
			mc_version, build_id, file_name
		))
	}

	async fn resolve_vanilla_url(&self, client: &Client) -> Result<String, ServerError> {
		let manifest: Value = client
			.get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
			.send()
			.await?
			.json()
			.await?;

		let latest_id = manifest["latest"]["release"]
			.as_str()
			.ok_or(ServerError::InvalidManifest("missing latest release"))?;

		let versions = manifest["versions"]
			.as_array()
			.ok_or(ServerError::InvalidManifest("missing versions array"))?;

		let version_entry = versions
			.iter()
			.find(|v| v["id"] == latest_id)
			.ok_or_else(|| ServerError::VersionNotFound(latest_id.to_string()))?;

		let version_url = version_entry["url"]
			.as_str()
			.ok_or(ServerError::InvalidManifest("missing version url"))?;

		let version_data: Value = client.get(version_url).send().await?.json().await?;

		version_data["downloads"]["server"]["url"]
			.as_str()
			.map(String::from)
			.ok_or(ServerError::InvalidManifest("missing server download url"))
	}

	async fn resolve_download_url(&self, client: &Client) -> Result<String, ServerError> {
		match self.clone().brand() {
			ServerBrand::Vanilla => self.resolve_vanilla_url(client).await,
			ServerBrand::Paper => self.resolve_paper_url(client).await,
		}
	}

	async fn try_download(&self, dir_builder: &str) -> Result<(), ServerError> {
		let client: Client = Client::new();
		let url: String = self.resolve_download_url(&client).await?;

		let file_name: &str = url.split('/').next_back().unwrap_or("server.jar");
		let mut path : PathBuf= PathBuf::from(dir_builder);
		path.push(file_name);

		println!("Downloading to: {:?}", path);

		let response = reqwest::get(&url).await?;
		let mut file = fs::File::create(path).await?;
		let bytes = response.bytes().await?;
		file.write_all(&bytes).await?;

		Ok(())
	}
}
