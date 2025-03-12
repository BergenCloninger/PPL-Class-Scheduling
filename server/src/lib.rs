mod endpoints;
mod wasm;
mod crypto;
mod auth;

use std::{collections::HashMap, sync::LazyLock};
use endpoints::Callback;
use http::request::Parts;
use worker::*;
use http_body_util::BodyExt;
use std::str;

mod test;

const ENABLE_AUTH: bool = false;

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<Response> {
	console_error_panic_hook::set_once();

	let database: D1Database = match env.d1("Class_Scheduling") {
		Ok(db) => db,
		Err(_e) => return Response::error("Failed to Retrieve Database!", 500),
	};

	let (parts, body) = req.into_parts();

	let response: Result<Response>;

	if parts.method == http::Method::GET {
		response = process_operation(&database, &parts, "").await // Slight optimization
	} else {
		let body_bytes: &[u8] = match body.collect().await {
			Ok(collected) => &collected.to_bytes(),
			Err(_e) => return Response::error("Failed to Read Body", 500),
		};
	
		let body_string: &str = match str::from_utf8(body_bytes) {
			Ok(text) => text,
			Err(_e) => return Response::error("Invalid Body!", 400),
		};
	
		response = process_operation(&database, &parts, &body_string).await
	}

	match response {
		Ok(mut r) => { r.headers_mut().set("Access-Control-Allow-Origin", "*")?; Ok(r) },
		Err(e) => Err(e)
	}
}

async fn process_operation(database: &D1Database, parts: &Parts, body: &str) -> Result<Response> {
	// Incredibly lame double HashMap (no easy + fast way to build a LUT over `http::Method`)
	static RESOURCE_MAP: LazyLock<HashMap<&str, &HashMap<http::Method, Callback>>> = LazyLock::new(|| { HashMap::from([
		("/admin/define", &*endpoints::DEFINE),
		("/login", &*endpoints::LOGIN),
		("/user", &*endpoints::USER),
		("/faculty", &*endpoints::FACULTY),
		("/class", &*endpoints::CLASS),
		("/room", &*endpoints::ROOM),
		("/schedule", &*endpoints::SCHEDULE),
		("/feature", &*endpoints::FEATURE),
		("/pref", &*endpoints::PREFERENCE),
		("/csr", &*endpoints::CSR),
		("/cf", &*endpoints::CF),
		("/rf", &*endpoints::RF)
	]) });

	let methods: &HashMap<http::Method, Callback> = match (*RESOURCE_MAP).get(&parts.uri.path()) {
		Some(methods) => methods,
		None => return Response::error("Invalid path!", 400)
	};

	// CORS handling
	if parts.method == http::Method::OPTIONS {
		return Ok(
			Response::builder()
				.with_status(200)
				.with_header("Access-Control-Allow-Methods", &methods.keys().map(|m: &http::Method| -> &str { m.as_str() }).collect::<Vec<&str>>().join(", "))?
				.with_header("Access-Control-Allow-Headers", "Content-Type,X-MsgId,X-Signature")?
				.with_header("Access-Control-Max-Age", "86400")?
				.empty()
		);
	}

	// Rough proof-of-concept auth
	let mut user: auth::ActiveUser = auth::ActiveUser { name: "", perms: auth::UserPerms::Admin };

	if ENABLE_AUTH {
		match parts.headers.get("X-Signature") {
			Some(h) => {
				let creds: &str = h.to_str().or(Err("Bad Signature Formatting"))?;

				let (username, password) = creds.split_once(' ').ok_or(worker::Error::RustError("".to_string()))?;
				user.name = username;

				let query: db::read::User = db::read::read_from_user_with_username(&database, username).await.or(Err("User Lookup Failure"))?;

				if query.password != password {
					return Response::error("Invalid Credentials!", 400);
				}

				user.perms = auth::UserPerms::from(query.role.as_str());
			},
			None => user.perms = auth::UserPerms::None
		};
	}

	// Passing an empty query to the inner match causes it to resolve to `None`, which is very annoying, hence this
	let pairs: Vec<(&str, &str)> = match parts.uri.query() {
		Some(query) =>  match query.split("&").map(|kv: &str| -> Option<(&str, &str)> { kv.split_once("=") }).collect::<Option<Vec<(&str, &str)>>>() {
			Some(v) => v,
			None => return Response::error("Invalid query parameter!", 400)
		},
		None => vec![]
	};

	match methods.get(&parts.method) {
		Some(callback) => callback(&database, &user, &mut endpoints::Query::from(&pairs), &body).await,
		None => return Response::error("Invalid method!", 400)
	}
}