use std::{collections::HashMap, sync::LazyLock};
use serde::{Deserialize, Serialize};
use crate::endpoints::{GroupEntry, GroupWrapper};

use super::{resource_callback, Callback};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Feature {
	name: String,
	description: String
}

impl From<db::read::Feature> for Feature {
	fn from(entry: db::read::Feature) -> Self {
		Feature { name: entry.name, description: entry.description }
	}
}

pub static FEATURE: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(database, user, query, _body, { user.require_perm(&crate::auth::UserPerms::General)?;
			// TODO: mandate null body for all reads
			match query.consume("scope").unwrap_or("single") {
				"single" => match db::read::read_from_feature(&database, str::parse::<i32>(query.consume_expect("id")?).or(Err("Parse Error"))?).await {
					Ok(feature) => Response::ok(serde_json::to_string(&Feature::from(feature))?), // ? What will this return if the `Result` is an `Err`? See other `/read` methods as well.
					Err(e) => Response::error(format!("{:?}", e), 500),
				},
				"all" =>  match db::read::read_all_from_features(&database).await {
					Ok(features) => Response::ok(serde_json::to_string(&GroupWrapper::<GroupEntry::<Feature>> { entries: &features.into_iter().map(
						|s: db::read::Feature| GroupEntry::<Feature> { id: s.id, data: Feature::from(s) }
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
			let request: Feature = serde_json::from_str(body)?;

			match db::create::create_feature(&database, &request.name, &request.description).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });