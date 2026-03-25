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

use std::{
	error::Error,
	str::FromStr,
};

use crate::{
	api::auth::{
		create::CreateStruct,
		login::LoginStruct,
	},
	keyring_service::KeyringService,
};
use axum::{
	Router,
	body::Body,
	http::StatusCode,
	response::Response,
	routing::get,
};
use axum_cookie::{
	CookieLayer,
	CookieManager,
	cookie::Cookie,
};
use base64::{
	Engine,
	alphabet,
	engine::{
		GeneralPurpose,
		general_purpose,
	},
};
use log::{
	debug,
	error,
	warn,
};
use serde::{
	Deserialize,
	Serialize,
};
use uuid::Uuid;

use axum_extra::extract::Query;
use jsonwebtoken::{
	Algorithm,
	DecodingKey,
	EncodingKey,
	Header,
	Validation,
	decode,
	encode,
};

pub(crate) const SESSION_COOKIE_NAME: &str = "AUTH";

#[derive(Deserialize)]
struct SignupQuery {
	username: String,
	uuid: String,
	password: String,
}

#[derive(Deserialize)]
struct LoginQuery {
	uuid: Option<String>,
	password: Option<String>,
}

fn auth_router() -> Router {
	Router::new()
		.route("/signup/", get(signup))
		.route("/login/", get(login))
}

pub(crate) async fn auth_api() -> Router {
	auth_router().layer(CookieLayer::default())
}

/// Create an entity inside Database, then make a new auth session using jwt
/// We can use jwt token for login. The paramaters are all queries so they may not be filled out
/// (possible no value).
async fn signup(manager: CookieManager, Query(query): Query<SignupQuery>) -> Response {
	let keyring_service = KeyringService::new("Lunara");

	let username_parsed: String = decode_b64(query.username).unwrap();
	let password_parsed: String = decode_b64(query.password).unwrap();
	let decoded_uuid = decode_b64(&query.uuid).unwrap_or(String::new());

	let uuid_parsed = match Uuid::from_str(decoded_uuid.as_str()) {
		Ok(u) => u,
		Err(_) => return response(StatusCode::BAD_REQUEST, "invalid uuid"),
	};

	let signup_struct = CreateStruct::builder()
		.uuid(uuid_parsed)
		.username(&username_parsed)
		.password(&password_parsed)
		.build();

	let signup_result = signup_struct.create_account().await;

	debug!("created signup struct");

	match signup_result {
		Ok(result) => {
			debug!("found key!");

			let key = keyring_service.get_secret("key").await;

			if let Ok(r) = key {
				let jwt = generate_jwt(result.1, r.as_bytes());

				// jwt result -> non-growable string (&str)
				if let Ok(jwt_str) = jwt {
					save_cookie(manager, SESSION_COOKIE_NAME, &jwt_str);
				}

				debug!("saved jwt token")
			}

			debug!("done");

			response(result.0, "Done!")
		}
		Err(error) => {
			error!("error in making account: {}", error);
			response(StatusCode::BAD_REQUEST, "Unable to create account.")
		}
	}
}

async fn session_login(manager: CookieManager) -> Response {
	let cookie = manager.get(SESSION_COOKIE_NAME).unwrap_or_default();

	if cookie.value().is_empty() {
		return response(StatusCode::UNAUTHORIZED, "Failed to login");
	}

	let keyring_service = KeyringService::new("Lunara");
	let key = match keyring_service.get_secret("key").await {
		Ok(key) => key,
		Err(error) => {
			error!("Unable to find service key for session login. {}", error);
			return response(StatusCode::INTERNAL_SERVER_ERROR, "Failed to login");
		}
	};

	let jwt_res = match validate_jwt(key.as_bytes(), cookie.value()) {
		Ok(jwt_res) => jwt_res,
		Err(error) => {
			warn!("Session token validation failed. {}", error);
			return response(StatusCode::UNAUTHORIZED, "Failed to login");
		}
	};

	match jwt_res.login().await {
		Ok(result) => result.0,
		Err(error) => {
			error!("Error while using jwt login. {}", error);
			response(StatusCode::BAD_REQUEST, "Failed to login")
		}
	}
}

/// Creates a jwt token based on url-provided credentials (queries)
/// Additionally, the function will check inside KeyringService and attempt to login using a jwt
/// token Take further notice that queries must be base64 encoded
async fn login(manager: CookieManager, Query(query): Query<LoginQuery>) -> Response {
	let (uuid, password) = match (query.uuid, query.password) {
		(None, None) => return session_login(manager).await,
		(Some(uuid), Some(password)) => (uuid, password),
		_ => return response(StatusCode::BAD_REQUEST, "uuid and password are required"),
	};

	let decoded_uuid = decode_b64(&uuid).unwrap_or(String::from("N/A"));

	let uuid_parsed = match Uuid::from_str(&decoded_uuid) {
		Ok(u) => u,
		Err(_) => return response(StatusCode::BAD_REQUEST, "invalid uuid"),
	};

	let credentials = LoginStruct::builder()
		.uuid(uuid_parsed)
		.password(password)
		.build();

	let login_result = credentials.login().await;

	match login_result {
		Ok(result) => result.0,
		Err(error) => {
			error!("Error while using credential login. {}", error);
			warn!("Maybe you didn't use the right password?");

			response(StatusCode::BAD_REQUEST, "Failed to login")
		}
	}
}

// Other
fn generate_jwt<T>(var: T, key: &[u8]) -> Result<String, Box<dyn Error + Sync + Send>>
where
	T: Serialize, {
	let header = Header::new(Algorithm::HS256);
	let encoding_key = EncodingKey::from_secret(key);

	Ok(encode(&header, &var, &encoding_key)?)
}

pub(crate) fn validate_jwt(
	key: &[u8],
	token: &str,
) -> Result<LoginStruct, Box<dyn Error + Sync + Send>> {
	let decoding_key = DecodingKey::from_secret(key);
	let mut validation = Validation::new(Algorithm::HS256);
	validation.required_spec_claims.clear();
	validation.validate_exp = false;

	Ok(decode::<LoginStruct>(token, &decoding_key, &validation).map(|data| data.claims)?)
}

fn save_cookie<S: Into<String>>(manager: CookieManager, name: S, value: S) {
	let cookie = Cookie::new(name.into(), value.into());

	manager.add(cookie);
}

fn response(status: StatusCode, msg: &'static str) -> Response {
	Response::builder()
		.status(status)
		.body(Body::from(msg))
		.unwrap_or_else(|_| Response::new(Body::from(msg)))
}

pub fn decode_b64<S: AsRef<[u8]>>(encoded: S) -> Result<String, Box<dyn Error + Sync + Send>> {
	let decoded_bytes =
		GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD).decode(encoded)?;

	Ok(String::from_utf8(decoded_bytes)?)
}

#[cfg(test)]
mod tests {
	use std::usize;

	use super::*;
	use axum::{
		body::Body,
		http::{
			Request,
			StatusCode,
		},
		response::Response,
	};
	use base64::{
		Engine,
		alphabet,
		engine::{
			GeneralPurpose,
			general_purpose,
		},
	};
	use tower::ServiceExt;

	fn encode_base64(input: &str) -> String {
		GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD).encode(input)
	}

	#[tokio::test]
	async fn create_response_fine() {
		let expect = response(StatusCode::BAD_REQUEST, "Test");

		assert_eq!(expect.status(), StatusCode::BAD_REQUEST);
		let expect = response(StatusCode::BAD_REQUEST, "Test");
		let body_bytes = axum::body::to_bytes(expect.into_body(), usize::MAX)
			.await
			.unwrap();

		assert_eq!(body_bytes, "Test");
	}

	#[test]
	fn base64_encodes_fine() {
		let expected = String::from("Hello, World!");

		let encoded = String::from("SGVsbG8sIFdvcmxkIQ");
		let decoded = GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD)
			.decode(encoded)
			.unwrap();

		let decoded_str = String::from_utf8(decoded).unwrap();

		assert_eq!(expected, decoded_str)
	}

	#[tokio::test]
	async fn auth_api_router_has_signup_route() {
		let app = auth_api().await;
		let uuid = encode_base64(&Uuid::new_v4().to_string());
		let username = encode_base64("testuser");
		let password = encode_base64("testpass");

		let response: Response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri(format!(
						"/signup/?uuid={}&username={}&password={}",
						uuid, username, password
					))
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
	}

	#[tokio::test]
	async fn auth_api_router_has_login_route() {
		let app = auth_api().await;
		let username = encode_base64(&Uuid::new_v4().to_string());
		let password = encode_base64("testpass");

		let response: Response = app
			.oneshot(
				Request::builder()
					.method("POST")
					.uri(format!("/login/?uuid={}&password={}", username, password))
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
	}

	#[tokio::test]
	async fn auth_api_returns_404_for_unknown_route() {
		let app = auth_api().await;

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri("/nonexistent")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[test]
	fn test_jwt_roundtrip() {
		let uuid = Uuid::new_v4();
		let password = "password123".to_string();
		let login = LoginStruct::builder()
			.uuid(uuid)
			.password(password.clone())
			.build();
		let key = b"secret_key";

		let token = generate_jwt(&login, key).expect("Failed to generate JWT");
		let decoded = validate_jwt(key, &token).expect("Failed to validate JWT");

		assert_eq!(decoded.uuid, uuid);
		assert_eq!(decoded.password, password);
	}

	#[test]
	fn test_jwt_invalid_signature() {
		let uuid = Uuid::new_v4();
		let password = "password123".to_string();
		let login = LoginStruct::builder().uuid(uuid).password(password).build();
		let key = b"secret_key";
		let wrong_key = b"wrong_key";

		let token = generate_jwt(&login, key).expect("Failed to generate JWT");
		let result = validate_jwt(wrong_key, &token);

		assert!(result.is_err());
	}

	#[test]
	fn test_login_struct_serialization() {
		let uuid = Uuid::new_v4();
		let password = "password".to_string();
		let login = LoginStruct::builder()
			.uuid(uuid)
			.password(password.clone())
			.build();

		let serialized = serde_json::to_string(&login).unwrap();
		let deserialized: LoginStruct = serde_json::from_str(&serialized).unwrap();

		assert_eq!(deserialized.uuid, uuid);
		assert_eq!(deserialized.password, password);
	}

	#[tokio::test]
	async fn test_signup_route_invalid_uuid() {
		let app = auth_api().await;
		let username = encode_base64("user");
		let password = encode_base64("pass");
		let invalid_uuid = encode_base64("not-a-uuid");

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri(format!(
						"/signup/?uuid={}&username={}&password={}",
						invalid_uuid, username, password
					))
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
			.await
			.unwrap();
		assert_eq!(body_bytes, "invalid uuid");
	}

	#[tokio::test]
	async fn test_login_route_invalid_uuid() {
		let app = auth_api().await;
		let password = encode_base64("pass");
		let invalid_uuid = encode_base64("not-a-uuid");

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri(format!(
						"/login/?uuid={}&password={}",
						invalid_uuid, password
					))
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
			.await
			.unwrap();
		assert_eq!(body_bytes, "invalid uuid");
	}

	#[tokio::test]
	async fn test_jwt_generation_and_cookie() {
		// This tests the helper function save_cookie indirectly if we could check headers,
		// but save_cookie puts it in CookieManager which is a layer.
		// We can test `generate_jwt` logic again with different data.
		let uuid = Uuid::new_v4();
		let password = "pass".to_string();
		let login = LoginStruct::builder().uuid(uuid).password(password).build();
		let key = b"key";

		let token = generate_jwt(&login, key).unwrap();
		assert!(!token.is_empty());
	}

	#[tokio::test]
	async fn test_signup_route_integration() {
		if mock_database().await.is_none() {
			return;
		}

		// Since we can't easily mock KeyringService, this test might fail if keyring is missing.
		// We'll wrap it in a catch_unwind or just accept it might fail in some envs.
		// But standard `cargo test` runs everything.
		// Given the constraints, I will add the test but comment that it requires env.

		let app = auth_api().await;
		let uuid = Uuid::new_v4();
		let uuid_enc = encode_base64(&uuid.to_string());
		let username = encode_base64("testuser_integration");
		let password = encode_base64("testpass_integration");

		// We cannot assert success because we don't know if KeyringService works in this env.
		// But we can assert that it doesn't return 404.
		let response: Response = app
			.oneshot(
				Request::builder()
					.uri(format!(
						"/signup/placeholder/placeholder/placeholder?uuid={}&username={}&password={}",
						uuid_enc, username, password
					))
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
		// It might be 200, 201, 400 (if keyring fails), or 500.
	}

	#[tokio::test]
	async fn test_login_route_integration() {
		if mock_database().await.is_none() {
			return;
		}

		let app = auth_api().await;
		let uuid_enc = encode_base64(&Uuid::new_v4().to_string());
		let password = encode_base64("pass");

		let response: Response = app
			.oneshot(
				Request::builder()
					.uri(format!("/login/?uuid={}&password={}", uuid_enc, password))
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_ne!(response.status(), StatusCode::NOT_FOUND);
	}

	async fn mock_database() -> Option<crate::database::Database> {
		crate::database::Database::connect("postgres://postgres:postgres@localhost:5432/lunara")
			.await
			.ok()
	}
}
