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
mod http;
pub(crate) mod keyring_service;
mod mc;
mod route;
use axum::Router;
use std::{collections::HashMap, error::Error, path::Path};

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

// Postgres
const POSTGRES_PORT_DEF: &str = "5432";
const POSTGRES_HOST_DEF: &str = "postgres_database";
const POSTGRES_NAME_DEF: &str = "postgres";
const POSTGRES_USER_DEF: &str = "postgres";
const POSTGRES_PASSWORD_DEF: &str = "postgres";

// Routes
const HEALTH_ROUTE: &str = "/health";
const MC_ROUTE_PREFIX: &str = "/mc";
const AUTH_ROUTE_PREFIX: &str = "/auth/v1";
const API_ROUTE_PREFIX: &str = "/api";

// Other
const LOG_LEVEL: LevelFilter = LevelFilter::Info;
const SERVER_ADDR: &str = "0.0.0.0";
const SERVER_PORT: u16 = 5000;
const CONFIG: &str = "config.toml";
const DEGRADED_HEALTH_MESSAGE: &str = "Running. No database.";
const HEALTHY_MESSAGE: &str = "Lunara is running.";

type Routes = (Router, Router);

pub(crate) struct App {
	config: Config,
}

impl App {
	/// Returns a result that contains database-required routes.
	/// This function initializes a database variable and creates a pointer for it.
	/// If the database's connection times out or something goes wrong, the functionality of
	/// returned routes won't work as intended.
	fn new(cfg: Config) -> Self {
		App { config: cfg }
	}

	async fn start(&self) -> Result<Routes, Box<dyn Error + Send + Sync>> {
		let conn_str = self.config.conn_str();

		info!("database connection str: {}", conn_str);

		let database = database::database(conn_str).await?;
		let auth_routes: Router<_> = auth_route::auth_api().await;
		let api_routes: Router<_> = api_route::user_api(database).await;

		Ok((auth_routes, api_routes))
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

	pub async fn init(&mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
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

			self.config.with_key(new_key);
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

		self.config
			.with_key(arr)
			.with_conn_str(connection_string)
			.with_port(SERVER_PORT);

		let database_routes = match self.start().await {
			Ok((auth_routes, api_routes)) => {
				info!("Database connected successfully");
				Some((auth_routes, api_routes))
			}
			Err(error) => {
				error!("connection timed out. More information: {}", error);
				warn!("Don't worry! You can still use Lunara without a database.");
				None
			}
		};

		let app = impl_routes(database_routes);

		let string_addr: String = format!("{}:{}", SERVER_ADDR, SERVER_PORT);
		let alt_addr: String = format!("localhost:{}", SERVER_PORT);

		info!(
			"Done! Now serving {}. Alternatively: {}",
			string_addr, alt_addr
		);

		let listener: TcpListener = TcpListener::bind(string_addr).await?;

		debug!("serving...");

		self.config.write_toml().await?;
		axum::serve(listener, app).await?;

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

	let config_path: &Path = Path::new(CONFIG);

	if !config_path.exists() {
		File::create_new(CONFIG).await?;

		debug!("Creating new config file");
	}

	let toml_result = Config::default().get_from_toml().await;

	if let Err(ref e) = toml_result {
		error!("Config error: {:?}", e);
	};

	let config: Config = toml_result?.unwrap_or(Config::default());

	info!("Running Lunara.");

	App::new(config).init().await
}

fn impl_routes(db_routes: Option<Routes>) -> Router {
	let app = base_routes();
	let app = health_route(app, db_routes.is_some());

	db_route_optional(app, db_routes)
}

fn base_routes() -> Router {
	Router::new().nest(MC_ROUTE_PREFIX, mc_route())
}

fn health_route(app: Router, db_available: bool) -> Router {
	if db_available {
		return app.route(HEALTH_ROUTE, get(health_ok));
	}

	app.route(HEALTH_ROUTE, get(health_degraded))
}

fn db_route_optional(app: Router, db_routes: Option<Routes>) -> Router {
	match db_routes {
		Some((auth_routes, api_routes)) => app
			.nest(AUTH_ROUTE_PREFIX, auth_routes)
			.nest(API_ROUTE_PREFIX, api_routes),
		None => app,
	}
}

async fn health_ok() -> &'static str {
	HEALTHY_MESSAGE
}

async fn health_degraded() -> &'static str {
	DEGRADED_HEALTH_MESSAGE
}

// Source - https://stackoverflow.com/a/29570662
fn conv_vec_arr<T, const V: usize>(v: Vec<T>) -> [T; V] {
	v.try_into()
		.unwrap_or_else(|_| panic!("Expected vec of length {}", V))
}
