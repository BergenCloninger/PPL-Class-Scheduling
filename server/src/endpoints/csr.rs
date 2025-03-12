use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct CSR {
	class: i32,
	room: i32,
	schedule: i32
}

impl From<db::read::ClassScheduleRoom> for CSR {
	fn from(entry: db::read::ClassScheduleRoom) -> Self {
		CSR { class: entry.class_id, room: entry.room_id, schedule: entry.schedule_id }
	}
}

pub static CSR: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match db::read::read_from_class_schedule_room(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
				Ok(csr) => Response::ok(serde_json::to_string(&CSR::from(csr))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	),
	(
		http::Method::POST,
		resource_callback!(database, user, _query, body, { user.require_perm(&crate::auth::UserPerms::High)?;
			let request: CSR = serde_json::from_str(body)?;

			match db::create::create_class_schedule_room(&database, request.class, request.schedule, request.room).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	),
	(
		http::Method::PUT,
		resource_callback!(database, user, query, body, { user.require_perm(&crate::auth::UserPerms::High)?;
			#[derive(Debug, PartialEq, Deserialize)]
			struct RequestBody {
				column: String,
				value: String
			}

			let request: RequestBody = serde_json::from_str(body)?;

			match db::update::update_class_schedule_room(&database, &request.column, &request.value, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });