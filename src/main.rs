extern crate discord;
extern crate dpermissions;
#[macro_use]
extern crate lazy_static;

mod commands;
mod strokes;
mod help;

use discord::Discord;
use discord::model::Event;
use std::env;
use dpermissions::Permissions;

lazy_static! {
	static ref PERMS: Permissions = dpermissions::load("./perms.json")
		.expect("Unable to load permissions, check that perms.json exists and is readable.");
}

fn main() {
	let token = env::args().nth(1).expect("No token specified. Use felix <token>");
	let session = Discord::from_bot_token(&token).expect("Login failed. Invalid token?");
	let (mut conn, _) = session.connect().expect("Connection failed.");
	lazy_static::initialize(&PERMS);
	println!("Felix is running.");
	loop {
		match conn.recv_event() {
			Ok(Event::MessageCreate(m)) => {
				commands::parse_cmd(&session, &m);
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed with code {:?}:\n{}", code, body);
			}
			Err(err) => println!("Error {:?}", err)
		}
	}
}

