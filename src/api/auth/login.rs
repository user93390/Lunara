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

use crate::database::database;
use axum::extract::Path;
use log::warn;
use uuid::Uuid;

pub(crate) async fn try_login(Path((username, password)): Path<(String, String)>) -> Option<Uuid> {
    let db = database().await.ok()?;

    let rows = db
        .select("accounts", &["uid"], Some("username = $1"), &[&username])
        .await
        .ok()?;

    let row = match rows.first() {
        Some(r) => r,
        None => {
            warn!("Cannot find uuid inside row.");
            return None;
        }
    };

    let uuid: Uuid = row.get(0);

    let password_row = db
        .select_one("accounts", &["password"], "uid = $1", &[&uuid])
        .await
        .ok()?;

    let valid = password_row
        .get::<usize, String>(0)
        .eq(&password);

    if !valid {
        return None;
    }

    Some(uuid)
}
