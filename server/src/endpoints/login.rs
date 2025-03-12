use std::{collections::HashMap, sync::LazyLock};
use super::{resource_callback, Callback};
use worker::*;

pub static LOGIN: LazyLock<HashMap<http::Method, Callback>> = LazyLock::new(|| { HashMap::from([
	(
		http::Method::GET,
		resource_callback!(_database, _user, _query, _body, {
			Response::ok(&crate::test::test().await)
		})
	)
]) });