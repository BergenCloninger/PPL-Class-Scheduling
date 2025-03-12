use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct User {
	username: String,
	password: String,
	role: String,
}

impl From<db::read::User> for User {
	fn from(entry: db::read::User) -> Self {
		User { username: entry.username, password: entry.password, role: entry.role }
	}
}

pub static USER: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, auth, query, _body, {
			// TODO: mandate null body for all reads
			match db::read::read_from_user(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
				Ok(user) => { auth.require_name(user.username.as_str())?; Response::ok(serde_json::to_string(&User::from(user))?) }, // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	),
	(
		http::Method::POST,
		resource_callback!(database, _auth, _query, body, {
			let request: User = serde_json::from_str(body)?;

			match db::create::create_user(&database, &request.username, &request.password, &request.role).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });