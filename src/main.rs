/*
 * Copyright 2025 seasnail1
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

extern crate log;
mod api;
mod config;
mod database;
mod entity;
mod keyring;
mod minecraft;
mod route;

use axum::Router;
use std::collections::HashMap;
use tower_http::services::{ServeDir, ServeFile};

use crate::{
	config::Config,
	database::Database,
	keyring::keyring_service::KeyringService,
	route::{
		api_route,
		auth_route,
	},
};

use log::{
	error,
	info,
	warn,
	LevelFilter,
};

use crate::route::servers::server_api;
use std::{
	convert::TryInto,
	error::Error,
	path::Path,
	sync::Arc,
};
use axum::routing::get;
use tokio::{
	fs::File,
	net::TcpListener,
};

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;

// Fallbacks or defaults
const POSTGRES_PORT_DEF: &str = "5432";
const POSTGRES_HOST_DEF: &str = "postgres_database";
const POSTGRES_NAME_DEF: &str = "postgres";
const POSTGRES_USER_DEF: &str = "postgres";
const POSTGRES_PASSWORD_DEF: &str = "postgres";

pub struct App {
	config: Config,
}
impl App {
	/// <p>Create an asynchronous router.</p>
	/// <p>Return all routes by nesting them inside a brand-new route.</p>
	/// Pretty expensive function.
	async fn start(self) -> Result<Router, Box<dyn Error + Send + Sync>> {
		let conn_str: &String = self.config.conn_str();

		info!("{}", conn_str);

		let result: Database = database::database(conn_str).await?;

		// create pointer.
		let db: Arc<Database> = Arc::new(result);

		let auth_route: Router<_> = auth_route::auth_api((*db).clone()).await;
		let api_route: Router<_> = api_route::user_api((*db).clone()).await;
		let server_api: Router<_> = server_api().await;

		let serve_dir = ServeDir::new("static")
			.append_index_html_on_directories(true)
			.not_found_service(ServeFile::new("static/index.html"));

		Ok(Router::new()
			.route("/health", get("Healthy!"))
			.nest("/auth/v1", auth_route)
			.nest("/api", api_route)
			.nest("/api", server_api)
			.fallback_service(serve_dir)
		)
	}

	pub async fn init_kering(
		keyring_service: &KeyringService,
	) -> Result<(), Box<dyn Error + Send + Sync>> {
		let secrets = [
			("db.host", POSTGRES_HOST_DEF),
			("db.port", POSTGRES_PORT_DEF),
			("db.name", POSTGRES_NAME_DEF),
			("db.user", POSTGRES_USER_DEF),
			("db.password", POSTGRES_PASSWORD_DEF),
		];

		let hash: HashMap<&str, &str> = secrets.iter().cloned().collect();

		for hashmap in hash {
			keyring_service.set_secret(hashmap.0, hashmap.1).await?;
		}

		Ok(())
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	let config_path = Path::new("config.toml");

	if !config_path.exists() {
		File::create_new("config.toml").await?;
	}

	let toml_result = Config::default().get_from_toml().await;

	if let Err(ref e) = toml_result {
		error!("Config error: {:?}", e);
	};

	let mut config: Config = toml_result?.unwrap();

	env_logger::builder()
		.format_timestamp_secs()
		.format_level(true)
		.filter_level(LevelFilter::Info)
		.init();

	info!("Running Lunara.");

	let keyring_service: KeyringService = KeyringService::new("Lunara");
	let key: bool = keyring_service.secret_exists("key").await;
	let first_time: bool = !key;

	if first_time {
		warn!("This is your first time running Lunara.");

		// gen new 128 key
		let new_key: [u8; 32] = KeyringService::generate_key_128();

		keyring_service
			.set_secret("key", &hex::encode(new_key))
			.await?;

		config.with_key(new_key);
	}

	App::init_kering(&keyring_service).await?;

	let key = keyring_service.get_secret("key").await?;

	let host = keyring_service.get_secret("db.host").await?;
	let port = keyring_service.get_secret("db.port").await?;
	let name = keyring_service.get_secret("db.name").await?;
	let user = keyring_service.get_secret("db.user").await?;
	let password = keyring_service.get_secret("db.password").await?;

	let connection_string = format!(
		"postgres://{}:{}@{}:{}/{}",
		user, password, host, port, name
	);

	let vec: Vec<u8> = hex::decode(key)?;
	let arr: [u8; 32] = conv_vec_arr(vec);

	config
		.with_key(arr)
		.with_path(String::from("config.toml"))
		.with_conn_str(connection_string.clone());

	let app: App = App {
		config: config.clone(),
	};

	// Wait for app
	let app: Router = app.start().await?;

	let string_addr: String = format!("{}:{}", SERVER_ADDR, SERVER_PORT);
	let alt_addr: String = format!("localhost:{}", SERVER_PORT);

	info!(
		"Done! Now serving {}. Alternatively: {}",
		string_addr, alt_addr
	);

	let listener: TcpListener = TcpListener::bind(string_addr).await?;

	config.write_toml().await.expect("Error writing toml");
	axum::serve(listener, app).await?;
	Ok(())
}

fn conv_vec_arr<T, const V: usize>(v: Vec<T>) -> [T; V] {
	v.try_into()
		.unwrap_or_else(|_| panic!("Expected a Vec of length {}", V))
}
