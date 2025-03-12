use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::endpoints::{GroupEntry, GroupWrapper};

use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Room {
	number: String,
	capacity: i32,
	kind: String // `type` is reserved (can get around, but mildly annoying)
}

impl From<db::read::Room> for Room {
	fn from(entry: db::read::Room) -> Self {
		Room { number: entry.room_number, capacity: entry.capacity, kind: entry.room_type }
	}
}

pub static ROOM: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match query.consume("scope").unwrap_or("single") {
				"single" => match db::read::read_from_room(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
					Ok(room) => Response::ok(serde_json::to_string(&Room::from(room))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				"all" =>  match db::read::read_all_from_rooms(&database).await {
					Ok(rooms) => Response::ok(serde_json::to_string(&GroupWrapper::<GroupEntry::<Room>> { entries: &rooms.into_iter().map(
						|s: db::read::Room| GroupEntry::<Room> { id: s.id, data: Room::from(s) }
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
			let request: Room = serde_json::from_str(body)?;

			match db::create::create_room(&database, &request.number, request.capacity, &request.kind).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });