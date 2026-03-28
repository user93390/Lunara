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

pub(crate) mod auth;

#[cfg(test)]
mod tests {
	use super::auth::{
		create::CreateStruct,
		login::LoginStruct,
	};
	use uuid::Uuid;

	#[test]
	fn api_auth_builders_are_wired() {
		let uuid = Uuid::new_v4();

		let _create = CreateStruct::builder()
			.uuid(uuid)
			.username("tester")
			.password("secret")
			.build();

		let login = LoginStruct::builder()
			.uuid(uuid)
			.password(String::from("secret"))
			.build();

		assert_eq!(login.uuid, uuid);
		assert_eq!(login.password, "secret");
	}
}
