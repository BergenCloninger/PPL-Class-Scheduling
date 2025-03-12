use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::endpoints::{GroupEntry, GroupWrapper};

use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Time {
	hour: i32,
	minute: i32
}

#[derive(Deserialize, Serialize)]
struct Schedule {
	start: Time,
	end: Time,
	days: String
}

impl From<db::read::Schedule> for Schedule {
	fn from(entry: db::read::Schedule) -> Self {
		Schedule { start: Time { hour: entry.start_hour, minute: entry.start_minute}, end: Time { hour: entry.end_hour, minute: entry.end_minute}, days: entry.days }
	}
}

pub static SCHEDULE: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads	
			match query.consume("scope").unwrap_or("single") {
				"single" => match db::read::read_from_schedule(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
					Ok(schedule) => Response::ok(serde_json::to_string(&Schedule::from(schedule))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				"all" =>  match db::read::read_all_from_schedule(&database).await {
					Ok(schedules) => Response::ok(serde_json::to_string(&GroupWrapper::<GroupEntry::<Schedule>> { entries: &schedules.into_iter().map(
						|s: db::read::Schedule| GroupEntry::<Schedule> { id: s.id, data: Schedule::from(s) }
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
			let request: Schedule = serde_json::from_str(body)?;

			match db::create::create_schedule(&database, request.start.hour, request.start.minute, request.end.hour, request.end.minute, &request.days).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });