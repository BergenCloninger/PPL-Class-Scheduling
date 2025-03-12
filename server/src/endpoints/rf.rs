use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct RF {
	room: i32,
	feature: i32
}

impl From<db::read::RoomFeature> for RF {
	fn from(entry: db::read::RoomFeature) -> Self {
		RF { room: entry.room_id, feature: entry.feature_id }
	}
}

pub static RF: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match db::read::read_from_room_feature(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
				Ok(rf) => Response::ok(serde_json::to_string(&RF::from(rf))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	),
	(
		http::Method::POST,
		resource_callback!(database, user, _query, body, { user.require_perm(&crate::auth::UserPerms::High)?;
			let request: RF = serde_json::from_str(body)?;

			match db::create::create_room_feature(&database, request.room, request.feature).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });