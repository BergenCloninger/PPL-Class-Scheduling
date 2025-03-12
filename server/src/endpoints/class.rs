use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::endpoints::{GroupEntry, GroupWrapper};

use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
pub struct Class {
	name: String,
	description: String,
	capacity: i32,
	code: String,
	kind: String,
	section: String,
	term: String
}

impl From<db::read::Class> for Class {
	fn from(entry: db::read::Class) -> Self {
		Class {
			name: entry.name,
			description: entry.description,
			capacity: entry.capacity,
			code: entry.code,
			kind: entry.class_type,
			section: entry.section,
			term: entry.term
		}
	}
}

pub static CLASS: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			match query.consume("scope").unwrap_or("single") {
				"single" => match db::read::read_from_class(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
					Ok(class) => Response::ok(serde_json::to_string(&Class::from(class))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				"all" =>  match db::read::read_all_from_classes(&database).await {
					Ok(classes) => {
						Response::ok(serde_json::to_string(&GroupWrapper::<GroupEntry::<Class>> { entries: &classes.into_iter().map(
							|s: db::read::Class| GroupEntry::<Class> { id: s.id, data: Class::from(s) }
						).collect() })?)
					},
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				_ => Response::error("Invalid scope parameter", 400)
			}
		})
	),
	(
		http::Method::POST,
		resource_callback!(database, user, _query, body, { user.require_perm(&crate::auth::UserPerms::High)?;
			let request: Class = serde_json::from_str(body)?;

			match db::create::create_class(
				&database,
				&request.name,
				&request.description,
				request.capacity,
				&request.code,
				&request.kind,
				&request.section,
				&request.term
			).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });