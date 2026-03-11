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

extern crate alloc;
extern crate core;
extern crate log;

mod api;
mod config;
mod database;
mod entity;
mod http;
pub(crate) mod keyring_service;
mod mc;
mod route;
use axum::{Json, Router};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

use crate::{
	config::Config,
	database::Database,
	route::{api_route, auth_route},
};

use log::{LevelFilter, debug, error, info, warn};

use crate::route::mc_route::mc_route;
use axum::routing::get;
use keyring_service::KeyringService;
use tokio::{fs::File, net::TcpListener};

const LOG_LEVEL: LevelFilter = LevelFilter::Info;

const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;

// Defaults for any postgres database
const POSTGRES_PORT_DEF: &str = "5432";
const POSTGRES_HOST_DEF: &str = "postgres_database";
const POSTGRES_NAME_DEF: &str = "postgres";
const POSTGRES_USER_DEF: &str = "postgres";
const POSTGRES_PASSWORD_DEF: &str = "postgres";

pub(crate) struct App {
	pub config: Config,
}

impl App {
	/// Returns a result that contains database-required routes.
	/// This function initializes a database variable and creates a pointer for it.
	/// If the database's connection times out or something goes wrong, the functionality of returned routes won't work as intended.
	async fn start(self) -> Result<(Router, Router), Box<dyn Error + Send + Sync>> {
		let conn_str: &String = self.config.conn_str();

		info!("database connection str: {}", conn_str);

		let result: Database = database::database(conn_str).await?;
		let db: Arc<Database> = Arc::new(result);

		let auth_route: Router<_> = auth_route::auth_api((*db).clone()).await;
		let api_route: Router<_> = api_route::user_api((*db).clone()).await;

		Ok((auth_route, api_route))
	}

	pub async fn init_keyring(
		keyring_service: &KeyringService,
	) -> Result<(), Box<dyn Error + Send + Sync>> {
		let secrets = [
			("db.host", POSTGRES_HOST_DEF),
			("db.port", POSTGRES_PORT_DEF),
			("db.name", POSTGRES_NAME_DEF),
			("db.user", POSTGRES_USER_DEF),
			("db.password", POSTGRES_PASSWORD_DEF),
		];

		info!("Initializing database credentials");

		let hash: HashMap<&str, &str> = secrets.iter().cloned().collect();

		for (key, value) in hash {
			if let Err(error) = keyring_service.set_secret(key, value).await {
				error!("Failed to store secret:");
				error!("{}: {}", error, key);
			}
		}
		Ok(())
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	env_logger::builder()
		.format_timestamp_secs()
		.format_level(true)
		.filter_level(LOG_LEVEL)
		.init();

	let config_path: &Path = Path::new("config.toml");

	if !config_path.exists() {
		File::create_new("config.toml").await?;

		debug!("Creating new config.toml file");
	}

	let toml_result = Config::default().get_from_toml().await;

	if let Err(ref e) = toml_result {
		error!("Config error: {:?}", e);
	};

	// Get toml result, fallback to default config if something didn't work.
	let mut config: Config = toml_result?.unwrap_or(Config::default());

	info!("Running Lunara.");

	let keyring_service: KeyringService = KeyringService::new("Lunara");
	let key: bool = keyring_service.secret_exists("key").await;
	let first_time: bool = !key;

	if first_time {
		warn!("This is your first time running Lunara.");

		// gen new 128 key
		let new_key: [u8; 32] = KeyringService::generate_key_128();

		info!("Generating new key.");

		let keyring_service_result = keyring_service
			.set_secret("key", &hex::encode(new_key))
			.await;

		if keyring_service_result.is_err() {
			error!("Error in keyring service.")
		}

		config.with_key(new_key);
	}

	info!("Init keyring secrets...");

	App::init_keyring(&keyring_service).await?;

	let key = keyring_service.get_secret("key").await?;

	let host = keyring_service.get_secret("db.host").await?;
	let port = keyring_service.get_secret("db.port").await?;
	let name = keyring_service.get_secret("db.name").await?;
	let user = keyring_service.get_secret("db.user").await?;
	let password = keyring_service.get_secret("db.password").await?;

	info!("Creating connection...");

	let connection_string: String = format!(
		"postgres://{}:{}@{}:{}/{}",
		user, password, host, port, name
	);

	let vec: Vec<u8> = hex::decode(key)?;
	let arr: [u8; 32] = conv_vec_arr(vec);

	config
		.with_key(arr)
		.with_conn_str(connection_string.clone())
		.with_port(SERVER_PORT);

	let app: App = App {
		config: config.clone(),
	};

	let (db_routes, health_msg) = match app.start().await {
		Ok((auth, api)) => {
			info!("Database connected successfully");
			(Some((auth, api)), "Healthy!")
		}
		Err(error) => {
			error!("connection timed out. More information: {}", error);
			warn!("Don't worry! You can still use Lunara without a database.");
			(None, "Healthy (degraded mode - no database)")
		}
	};

	let mc_route: Router<_> = mc_route();
	let mut app = Router::new()
		.route("/health", get(Json(health_msg)))
		.nest("/mc", mc_route);

	if let Some((auth, api)) = db_routes {
		app = app.nest("/auth/v1", auth).nest("/api", api);
	}

	let string_addr: String = format!("{}:{}", SERVER_ADDR, SERVER_PORT);
	let alt_addr: String = format!("localhost:{}", SERVER_PORT);

	info!(
		"Done! Now serving {}. Alternatively: {}",
		string_addr, alt_addr
	);

	let listener: TcpListener = TcpListener::bind(string_addr).await?;

	debug!("serving...");

	config.write_toml().await?;
	axum::serve(listener, app).await?;

	Ok(())
}

/// Converts a vector array into an array.
/// This function is dangerous due to it using panic.
/// note: Ensure you pass the right size
fn conv_vec_arr<T, const V: usize>(v: Vec<T>) -> [T; V] {
	v.try_into()
		.unwrap_or_else(|_| panic!("Expected vec of length {}", V))
}

// they losin bud
