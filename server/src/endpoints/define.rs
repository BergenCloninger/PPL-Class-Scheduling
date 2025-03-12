use std::{collections::HashMap, sync::LazyLock};
use super::{resource_callback, Callback};
use worker::*;

pub static DEFINE: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::POST,
		resource_callback!(database, user, _query, _body, { user.require_perm(&crate::auth::UserPerms::Admin)?;
			match db::define::define_db(&database).await {
				Ok(_) => Response::ok(""),
				Err(e) => Response::error(format!("{:?}", e), 500),
			}
		})
	)
]) });