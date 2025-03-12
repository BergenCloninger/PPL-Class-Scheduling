#[derive(Clone, PartialEq)]
pub enum UserPerms {
	None = 0,
	General = 1,
	High = 2,
	Admin = 3
}

impl From<&str> for UserPerms {
	fn from(val: &str) -> Self {
		match val {
			"student" => UserPerms::General,
			"faculty" => UserPerms::High,
			"admin" => UserPerms::Admin,
			_ => UserPerms::General
		}
	}
}

pub struct ActiveUser<'a> {
	pub name: &'a str,
	pub perms: UserPerms
}

impl<'a> ActiveUser<'a> {
	pub fn require_perm(&self, req: &UserPerms) -> Result<&Self, worker::Error> {
		if (self.perms.clone() as u32) < (req.clone() as u32) {
			return Err(worker::Error::RustError("Failed Authorization".to_string()));
		}

		Ok(self)
	}

	pub fn require_name(&self, req: &str) -> Result<&Self, worker::Error> {
		if (self.name != "") && (self.perms != UserPerms::Admin) && (self.name != req) {
			return Err(worker::Error::RustError("Failed Authorization".to_string()));
		}

		Ok(self)
	}
}