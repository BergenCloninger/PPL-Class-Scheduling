use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::endpoints::{GroupEntry, GroupWrapper};

use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Faculty {
	name: String,
	email: String,
	department: String
}

impl From<db::read::Faculty> for Faculty {
	fn from(entry: db::read::Faculty) -> Self {
		Faculty { name: entry.name, email: entry.email, department: entry.department }
	}
}

pub static FACULTY: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match query.consume("scope").unwrap_or("single") {
				"single" => match db::read::read_from_faculty(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
					Ok(faculty) => Response::ok(serde_json::to_string(&Faculty::from(faculty))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				"all" =>  match db::read::read_all_from_faculty(&database).await {
					Ok(faculties) => Response::ok(serde_json::to_string(&GroupWrapper::<GroupEntry::<Faculty>> { entries: &faculties.into_iter().map(
						|s: db::read::Faculty| GroupEntry::<Faculty> { id: s.id, data: Faculty::from(s) }
					).collect() })?),
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				_ => Response::error("Invalid scope parameter", 400)
			}
		})
	),
	(
		http::Method::POST,
		resource_callback!(database, user, _query, body, { user.require_perm(&crate::auth::UserPerms::High)?;
			let request: Faculty = serde_json::from_str(body)?;

			match db::create::create_faculty(&database, &request.name, &request.email, &request.department).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });