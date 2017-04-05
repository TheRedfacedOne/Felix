
extern crate discord;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::error;
use std::fmt;
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::path::Path;
use discord::model::UserId;

#[derive(Serialize, Deserialize)]
pub struct Permissions {
	pub groups: Vec<PermGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct PermGroup {
	pub name:      String,
	pub users:     Vec<u64>,
	pub subgroups: Vec<String>,
	pub perms:     Vec<String>,
}

impl Permissions {
	pub fn get_group(&self, name: &str) -> Option<&PermGroup> {
		for group in self.groups.iter() {
			if group.name == name { return Some(group); }
		}
		None
	}

	pub fn group_has_perm(&self, group: &PermGroup, perm: &str) -> bool {
		for p in group.perms.iter() {
			if p == perm { return true; }
		}
		for name in group.subgroups.iter() {
			if let Some(group) = self.get_group(name) {
				if self.group_has_perm(group, perm) { return true; }
			}
		}
		false
	}

	pub fn get_user_groups(&self, id: UserId) -> Vec<&PermGroup> {
		let mut groups: Vec<&PermGroup> = Vec::new();
		for group in self.groups.iter() {
			if group.users.contains(&id.0) { groups.push(group) }
		}
		groups
	}
	pub fn user_has_perm(&self, id: UserId, perm: &str) -> bool {
		for group in self.get_user_groups(id) {
			if self.group_has_perm(group, perm) { return true; }
		}
		false
	}
	pub fn save(&self, path: &str) -> Result<(), Error> {
		let mut file = try!(File::create(path));
		let data = try!(serde_json::to_string_pretty(self));
		file.write_all(data.as_bytes()).map_err(Error::Io)
	}
}
impl Default for Permissions {
	fn default() -> Permissions {
		Permissions{
			groups: vec![
				PermGroup {
					name: "default".into(),
					users: vec![],
					subgroups: vec![],
					perms: vec![
						"help".into(),
						"ping".into()
					]
				}
			]
		}
	}
}

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Parse(serde_json::Error),
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Io(ref e) => e.description(),
			Error::Parse(ref e) => e.description(),
		}
	}
	fn cause(&self) -> Option<&error::Error> {
		match *self {
			Error::Io(ref e) => Some(e),
			Error::Parse(ref e) => Some(e),
		}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Io(ref e) => e.fmt(f),
			Error::Parse(ref e) => write!(f, "Error parsing permissions:\n{}", e),
		}
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Error {
		Error::Io(e)
	}
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Error {
		Error::Parse(e)
	}
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Permissions, Error> {
	let file = try!(File::open(path));
	let mut reader = BufReader::new(file);
	let mut contents = String::new();
	try!(reader.read_to_string(&mut contents));
	let perms: Permissions = try!(serde_json::from_str(contents.as_str()));
	Ok(perms)
}
