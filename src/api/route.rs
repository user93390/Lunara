use crate::database::Database;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn api_users(database: Database) -> Router {
    Router::new()
        .route("/users", get(users))
        .with_state(Arc::new(database))
}

#[axum::debug_handler]
async fn users(
    State(db): State<Arc<Database>>,
) -> Result<Json<Vec<HashMap<Uuid, String>>>, (StatusCode, String)> {
    let mut users: Vec<HashMap<Uuid, String>> = Vec::new();

    let rows = db
        .select("accounts", &["uid", "username"], None, &[])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for row in rows {
        let uid: Uuid = row.get(0);
        let username: String = row.get(1);

        let mut hash = HashMap::new();
        hash.insert(uid, username);

        users.push(hash);
    }

    Ok(Json(users))
}
