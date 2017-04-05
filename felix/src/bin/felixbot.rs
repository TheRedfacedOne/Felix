
extern crate discord;
extern crate dpermissions;
extern crate felix;

use discord::model::Event;
use std::env;
//~ use hyper::client::Client;
//use felix::*;
use felix::DContext;
use felix::commands::Command;
use std::borrow::Cow::Borrowed;

const CMD_LIST: &'static [Command] = &[
	Command {
		label: Borrowed("!jt"),
		desc: Borrowed("Jisho test."),
		help_text: Borrowed("???"),
		perm: Borrowed("felix.jt"),
		run: felix::commands::jt_cmd
	},
	Command {
		label: Borrowed("!strokes"),
		desc: Borrowed("Shows stroke order for given character(s) (max 3)."),
		help_text: Borrowed("strokes <characters>"),
		perm: Borrowed("felix.strokes"),
		run: felix::strokes::strokes_cmd
	},
];

fn main() {
	let token = env::args().nth(1).expect("No token specified. Use felixbot [token]");
	println!("Token: {}", token);

	let mut dctx = DContext::from_bot_token(&token);

	let perms = felix::init_perms("./perms.json");
	println!("Felix is running.");

	loop {
		match dctx.connection.recv_event() {
			Ok(Event::MessageCreate(m)) => {
				felix::commands::parse_cmd(&dctx, &m, CMD_LIST, &perms);
			}
			Ok(event) => {
				dctx.state.update(&event);
			}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed with code {:?}:\n{}", code, body);
			}
			Err(err) => {
				println!("Error {:?}", err);
			}
		}
	}
}
