extern crate discord;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate hyper;

mod commands;
mod strokes;
mod help;
mod jisho;
mod io;
mod http;

use discord::Discord;
use discord::model::Event;
use std::env;

fn main() {
	let token = env::args().nth(1).expect("No token specified. Use felix <token>");
	let session = Discord::from_bot_token(&token).expect("Login failed. Invalid token?");
	let (mut conn, _) = session.connect().expect("Connection failed.");
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

