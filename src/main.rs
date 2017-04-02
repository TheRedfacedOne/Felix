
extern crate discord;
extern crate dpermissions;

mod commands;
mod strokes;

use discord::Discord;
use discord::model::Event;
use std::env;
use commands::Command;
use dpermissions::Permissions;
use dpermissions::Error;
//~ use hyper::client::Client;

fn main() {
	let token: String;
	token = env::args().nth(1).unwrap_or(String::from(""));
	if token == "" {
		println!("No token specified. Use felix [token]");
		return;
	} else {
		println!("Token: {}", token);
	}
	let session = Discord::from_bot_token(&token).expect("Login failed. Invalid token?");
	let (mut conn, _) = session.connect().expect("Connection failed.");
	let cmd_list = init_cmd_list();
	let perms = init_perms("./perms.json");
	println!("Felix is running.");
	loop {
		match conn.recv_event() {
			Ok(Event::MessageCreate(m)) => {
				commands::parse_cmd(&session, &m, &cmd_list, &perms);
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed with code {:?}:\n{}", code, body);
			}
			Err(err) => println!("Error {:?}", err)
		}
	}
}

fn init_cmd_list() -> Vec<Command> {
	vec![
		Command {
			label: String::from("!ping"),
			desc: String::from("Pong!"),
			help_txt: String::from("ping"),
			perm: String::from("felix.ping"),
			run: commands::ping_cmd
		},
		Command {
			label: String::from("!jt"),
			desc: String::from("Jisho test."),
			help_txt: String::from("???"),
			perm: String::from("felix.jt"),
			run: commands::jt_cmd
		},
		Command {
			label: String::from("!strokes"),
			desc: String::from("Shows stroke order for given character(s) (max 3)."),
			help_txt: String::from("strokes <characters>"),
			perm: String::from("felix.strokes"),
			run: strokes::strokes_cmd
		}
	]
}

fn init_perms(path: &str) -> Permissions {
	let perms = match dpermissions::load_perms(path) {
		Ok(p) => p,
		Err(err) => {
			match err {
				Error::Io(_) => {
					let perms = dpermissions::create_default();
					match dpermissions::save_perms(path, &perms) {
						Ok(_) => return perms,
						Err(e) => {
							panic!("Error saving default {}:\n{}", path, e);
						}
					}
				}
				Error::Parse(e) => {
					panic!("Error parsing {}:\n{}", path, e);
				}
			}
		}
	};
	perms
}
