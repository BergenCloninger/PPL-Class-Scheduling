use std::{future::Future, pin::Pin};
use serde::Serialize;
use worker::*;
use crate::auth::ActiveUser;

mod general_update;
#[allow(unused_imports)]
pub use general_update::*;

mod class_details;
#[allow(unused_imports)]
pub use class_details::*;

mod get_combined_details;
#[allow(unused_imports)]
pub use get_combined_details::*;

mod login;
#[allow(unused_imports)]
pub use login::*;

mod define;
#[allow(unused_imports)]
pub use define::*;

mod user;
#[allow(unused_imports)]
pub use user::*;

mod faculty;
#[allow(unused_imports)]
pub use faculty::*;

mod class;
#[allow(unused_imports)]
pub use class::*;

mod room;
#[allow(unused_imports)]
pub use room::*;

mod schedule;
#[allow(unused_imports)]
pub use schedule::*;

mod feature;
#[allow(unused_imports)]
pub use feature::*;

mod preference;
#[allow(unused_imports)]
pub use preference::*;

mod csr;
#[allow(unused_imports)]
pub use csr::*;

mod cf;
#[allow(unused_imports)]
pub use cf::*;

mod rf;
#[allow(unused_imports)]
pub use rf::*;

#[derive(Serialize)]
pub struct GroupEntry<T> where T: Serialize  {
	pub id: i32,
	pub data: T
}

#[derive(Serialize)]
pub struct GroupWrapper<'a, T> where T: Serialize { entries: &'a Vec<T>}

// Horrific code required to get async closures to work. :(
// Can't return an `impl` because it's arbitrarily not allowed.
pub type CallbackFuture<'a> = Pin<Box<dyn 'a + Future<Output = Result<Response>>>>;
pub type Callback = for<'a> fn(&'a D1Database, &'a ActiveUser, &'a mut Query, &'a str) -> CallbackFuture<'a>; // Explicit lifetimes require that the parameters live as long as the future.

// Uses local functions instead of closures because they don't currently support async.
macro_rules! resource_callback {
	($database:ident, $user:ident, $query:ident, $body:ident, $block:block) => {
		{
			use crate::endpoints::{CallbackFuture, Query};
			use crate::auth::ActiveUser;
			
			fn callback<'a>($database: &'a D1Database, $user: &'a ActiveUser, $query: &'a mut Query, $body: &'a str) -> CallbackFuture<'a> { Box::pin(async move {
				$block // Technically adds an unnecessary set of braces, but doesn't matter. (don't want to place directly in Box constructor to avoid the `async move`)
			})} callback as Callback // HashMap constructor doesn't seem to able to deduce type of callback properly, hence the casting.
		}
	};
}

pub(crate) use resource_callback;

pub struct Query<'a> {
	pairs: &'a Vec<(&'a str, &'a str)>,
	index: usize
}

impl<'a> From<&'a Vec<(&'a str, &'a str)>> for Query<'a> {
	fn from(pairs: &'a Vec<(&'a str, &'a str)>) -> Self {
		Query { pairs: pairs, index: 0 }
	}
}

impl<'a> Query<'a> {
	fn consume(&mut self, key: &str) -> Option<&str> {
		let pair: &(&str, &str) = self.pairs.get(self.index)?;

		if pair.0 == key {
			self.index += 1;
			Some(pair.1)
		} else {
			None
		}
	}

	fn consume_expect(&mut self, key: &str) -> Result<&str> {
		self.consume(key).ok_or(Error::from("Query key missing!"))
	}
}