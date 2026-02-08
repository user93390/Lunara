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
use crate::api::authentication::login::LoginAuth;
use crate::api::authentication::signup::SignupAuth;
use axum::http::StatusCode;
use std::error::Error;

pub(crate) mod login;
pub(crate) mod signup;

pub trait Authentication {
	async fn await_login(
		&self,
		auth: LoginAuth,
	) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		let _ = auth;

		Err(Box::new(std::io::Error::other(
			"await_login not implemented",
		)))
	}

	async fn await_signup(
		&self,
		auth: SignupAuth,
	) -> Result<StatusCode, Box<dyn Error + Sync + Send>> {
		let _ = auth;

		Err(Box::new(std::io::Error::other(
			"await_signup not implemented",
		)))
	}
}
