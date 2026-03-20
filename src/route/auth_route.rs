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

use std::str::FromStr;

use log::{error, warn};

use crate::api::auth::create::CreateStruct;
use crate::api::auth::login::LoginStruct;
use crate::keyring_service::KeyringService;
use axum::Router;
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use axum_cookie::cookie::Cookie;
use axum_cookie::{CookieLayer, CookieManager};
use base64::engine::general_purpose;
use base64::{Engine, alphabet, engine};
use jsonwebtoken::errors::Error;
use uuid::Uuid;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

const SESSION_COOKIE_NAME: &str = "AUTH";

pub(crate) async fn auth_api() -> Router {
	Router::new()
		.route("/signup/{uuid}/{username}/{password}", get(signup))
		.route("/login/", get(login))
		.layer(CookieLayer::default())
}

async fn signup(
	manager: CookieManager,
	username: Query<String>,
	uuid: Query<String>,
	password: Query<String>,
) -> Response {
	let keyring_service = KeyringService::new("Lunara");

	// parse queries

	let username_parsed: String = username.0;
	let password_parsed: String = password.0;
	let uuid_parsed = match Uuid::from_str(&uuid.0) {
		Ok(u) => u,
		Err(_) => return response(StatusCode::BAD_REQUEST, "invalid uuid"),
	};

	let signup_struct = CreateStruct::builder()
		.uuid(uuid_parsed)
		.username(&username_parsed)
		.password(&password_parsed)
		.build();

	let signup_result = signup_struct.create_account().await;

	return match signup_result {
		Ok(result) => response(result, "Done!"),
		Err(_) => response(StatusCode::BAD_REQUEST, "Unable to create account."),
	};

	//todo: make the fucking shit work.
}

/// Creates a jwt token based on url-provided credentials (queries)
/// Additionally, the function will check inside KeyringService and attempt to login using a jwt token
/// Take further notice that queries must be base64 encoded
async fn login(manager: CookieManager, uuid: Query<String>, password: Query<String>) -> Response {
	let cookie = manager.get(SESSION_COOKIE_NAME).unwrap();
	let keyring_service = KeyringService::new("Lunara");

	let key: &[u8] = &keyring_service
		.get_secret("key")
		.await
		.unwrap()
		.into_bytes();

	if let Ok(jwt_res) = validate_jwt(key, cookie.value()) {
		let login_result = jwt_res.login().await;

		return login_result.unwrap_or_default().0;
	}

	let uuid_parsed = match Uuid::from_str(&uuid.0) {
		Ok(u) => u,
		Err(_) => return response(StatusCode::BAD_REQUEST, "invalid uuid"),
	};

	let credentials = LoginStruct::builder()
		.uuid(uuid_parsed)
		.password(password.0)
		.build();

	let login_result = credentials.login().await;

	return match login_result {
		Ok(result) => {
			return result.0;
		}

		Err(error) => {
			error!("Error while using credential login. {}", error);
			warn!("Maybe you didn't use the right password?");

			response(StatusCode::BAD_REQUEST, "Failed to login")
		}
	};
}

// Util
fn generate_jwt(login: &LoginStruct, key: &[u8]) -> Result<String, Error> {
	let header = Header::new(Algorithm::HS256);
	let encoding_key = EncodingKey::from_secret(key);

	encode(&header, &login, &encoding_key)
}

fn validate_jwt(key: &[u8], token: &str) -> Result<LoginStruct, Error> {
	let decoding_key = DecodingKey::from_secret(key);
	let mut validation = Validation::new(Algorithm::HS256);
	validation.required_spec_claims.clear();
	validation.validate_exp = false;

	decode::<LoginStruct>(token, &decoding_key, &validation).map(|data| data.claims)
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

#[cfg(test)]
mod tests {
	use std::usize;

	use super::*;
	use axum::body::Body;
	use axum::http::{Request, StatusCode};
	use axum::response::Response;
	use base64::{Engine, alphabet, engine::GeneralPurpose, engine::general_purpose};
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
						"/signup/placeholder/placeholder/placeholder?uuid={}&username={}&password={}",
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
}
