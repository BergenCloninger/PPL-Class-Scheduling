use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct CF {
	class: i32,
	faculty: i32
}

impl From<db::read::ClassFaculty> for CF {
	fn from(entry: db::read::ClassFaculty) -> Self {
		CF { class: entry.class_id, faculty: entry.faculty_id }
	}
}

pub static CF: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match db::read::read_from_class_faculty(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
				Ok(cf) => Response::ok(serde_json::to_string(&CF::from(cf))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	),
	(
		http::Method::POST,
		resource_callback!(database, user, _query, body, { user.require_perm(&crate::auth::UserPerms::High)?;
			let request: CF = serde_json::from_str(body)?;

			match db::create::create_class_faculty(&database, request.class, request.faculty).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });