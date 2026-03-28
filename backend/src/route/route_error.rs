/*
Copyright 2026 seasnail1

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/
use crate::route::route_error::RouteError::{
	InternalError,
	NotFound,
};
use axum::{
	http::StatusCode,
	response::{
		IntoResponse,
		Response,
	},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouteError {
	#[error("Internal route error: {0}")]
	InternalError(String),
	#[error("Not found: {0}")]
	NotFound(String),
}

impl IntoResponse for RouteError {
	fn into_response(self) -> Response {
		match self {
			InternalError(error) => {
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Internal route error: {}", error),
				)
					.into_response()
			}
			NotFound(error) => {
				(StatusCode::NOT_FOUND, format!("Not found: {}", error)).into_response()
			}
		}
	}
}
