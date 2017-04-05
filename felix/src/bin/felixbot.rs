
extern crate discord;
extern crate dpermissions;

use discord::Discord;
use discord::model::Event;
use std::env;
use dpermissions::Permissions;
use dpermissions::Error;
//~ use hyper::client::Client;
//use felix::*;
use felix::DContext;
use felix::commands::Command;

const cmd_list: &'static [Command] = &[
	Command {
		label: "!ping".into(),
		desc: "Pong!".into(),
		help_text: "ping".into(),
		perm: "felix.ping".into(),
		run: commands::ping_cmd
	},
	Command {
		label: "!jt".into(),
		desc: "Jisho test.".into(),
		help_text: "???".into(),
		perm: "felix.jt".into(),
		run: commands::jt_cmd
	},
	Command {
		label: "!strokes".into(),
		desc: "Shows stroke order for given character(s) (max 3).".into(),
		help_text: "strokes <characters>".into(),
		perm: "felix.strokes".into(),
		run: strokes::strokes_cmd
	},
];

fn main() {
	let token = env::args().nth(1).expect("No token specified. Use felixbot [token]");
	println!("Token: {}", token);

	let mut dctx = DContext::from_bot_token(&token);

	let perms = felix::init_perms(&dctx, "./perms.json");
	println!("Felix is running.");
	loop {
		match conn.recv_event() {
			Ok(event) => dctx.state.update(&event)
			Ok(Event::MessageCreate(m)) => {
				commands::parse_cmd(&dctx, &m, cmd_list, &perms);
			}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed with code {:?}:\n{}", code, body);
			}
			Err(err) => println!("Error {:?}", err)
		}
	}
}
