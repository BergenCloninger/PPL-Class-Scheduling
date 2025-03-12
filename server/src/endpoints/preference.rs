use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::endpoints::{GroupEntry, GroupWrapper};

use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Preference {
	faculty: i32,
	kind: String,
	value: String,
}

impl From<db::read::Preference> for Preference {
	fn from(entry: db::read::Preference) -> Self {
		Preference { faculty: entry.faculty_id, kind: entry.preference_type, value: entry.value }
	}
}

pub static PREFERENCE: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match query.consume("scope").unwrap_or("single") {
				"single" => match db::read::read_from_preference(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
					Ok(preference) => Response::ok(serde_json::to_string(&Preference::from(preference))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				"all" =>  match db::read::read_all_from_preferences(&database).await {
					Ok(preferences) => Response::ok(serde_json::to_string(&GroupWrapper::<GroupEntry::<Preference>> { entries: &preferences.into_iter().map(
						|s: db::read::Preference| GroupEntry::<Preference> { id: s.id, data: Preference::from(s) }
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
			let request: Preference = serde_json::from_str(body)?;

			match db::create::create_preference(&database, request.faculty, &request.kind, &request.value).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });