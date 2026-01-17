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

pub mod db;

pub use db::Database;

use std::fs::File;
use std::io::BufReader;
use log::info;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::error::Error;

static CONNECTION_STRING: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

pub async fn database() -> Result<Database, Box<dyn Error + Send + Sync>> {
    if let Some(conn_str) = CONNECTION_STRING.lock().ok().and_then(|g| g.clone()) {
        info!("Using cached database connection string");
        return Ok(
            Database::connect(&conn_str).await?
        );
    }

    let properties_path = "database.properties";
    info!("Loading database configuration from {}", properties_path);

    let file = File::open(properties_path)?;
    let reader = BufReader::new(file);
    let props = java_properties::read(reader)?;

    let host = props.get("db.host").ok_or("Missing db.host")?;
    let port = props.get("db.port").ok_or("Missing db.port")?;
    let name = props.get("db.name").ok_or("Missing db.name")?;
    let user = props.get("db.user").ok_or("Missing db.user")?;
    let password = props.get("db.password").ok_or("Missing db.password")?;

    let connection_string = format!(
        "host={} port={} dbname={} user={} password={}",
        host, port, name, user, password
    );

    info!("Connecting to database at {}:{}", host, port);

    if let Ok(mut cached) = CONNECTION_STRING.lock() {
        *cached = Some(connection_string.clone());
    }

    Ok(Database::connect(&connection_string).await?)
}
