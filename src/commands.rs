use help;
use strokes;
use jisho;
use hyper;
use serde_json;
use std;
use discord::Discord;
use discord::model::Message;

pub enum CommandResult {
	Success,
	Syntax,
	InvalidArg(String),
	Warning(String),
	HttpError(hyper::error::Error),
	JsonError(serde_json::error::Error),
	IoError(std::io::Error)
}

pub struct Command {
	pub label: String,
	pub desc: String,
	pub help_txt: String,
	pub run: fn(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult
}

impl Command {
	pub fn exec(&self, s: &Discord, m: &Message, args: Vec<&str>){
		//let author = format!("{}", m.author.id);
		let ch = m.channel_id;
		match (self.run)(s, m, args) {
			CommandResult::Success => {}
			CommandResult::Syntax => {
				let help_msg = &self.help_txt;
				let _ = s.send_message(ch, &help_msg, "", false);
			}
			CommandResult::InvalidArg(msg) => {
				let _ = s.send_message(ch, &msg, "", false);
			}
			CommandResult::Warning(msg) => {
				println!("{}", &msg);
			}
			// Maybe there's a way to combine these 3 while still handling the error.
			CommandResult::HttpError(e) => {
				let _ = s.send_message(ch, "Something broke, rip", "", false);
				println!("HTTP error while running {}:\n  {:?}", self.label, e);
			}
			CommandResult::JsonError(e) => {
				let _ = s.send_message(ch, "Something broke, rip", "", false);
				println!("JSON error while running {}:\n  {:?}", self.label, e);
			}
			CommandResult::IoError(e) => {
				let _ = s.send_message(ch, "Something broke, rip", "", false);
				println!("I/O error while running {}:\n  {:?}", self.label, e);
			}
		}
	}
}

lazy_static! {
	pub static ref COMMANDS: Vec<Command> = vec! [
		Command {
			label: "!help".into(),
			desc: "Shows this help message.".into(),
			help_txt: "Usage:\n`help [command]`\nExample:\n`help strokes`".into(),
			run: help::help_cmd
		},
		Command {
			label: "!strokes".into(),
			desc: "Show the stroke order of a given character.".into(),
			help_txt: "Usage:\n`strokes <chars>`\nExample:\n`strokes 猫`".into(),
			run: strokes::strokes_cmd
		},
		Command {
			label: "!jisho".into(),
			desc: "Searches jisho.org for a given query.".into(),
			help_txt: "Usage:\n`jisho <query>`\nExample:\n`jisho 猫`".into(),
			run: jisho::jisho_cmd
		},
		Command {
			label: "!random".into(),
			desc: "Grabs a random word from Jisho.".into(),
			help_txt: "Usage:\n`random <jlpt-level>`\nExample:\n`random 5`".into(),
			run: jisho::random_cmd
		}
	];
}

pub fn parse_cmd(s: &Discord, m: &Message) {
	let mut usage: Vec<&str> = m.content.split(' ').collect();
	let label = usage.remove(0).to_lowercase();
	for cmd in COMMANDS.iter() {
		if cmd.label == label {
			cmd.exec(s, m, usage);
			break;
		}
	}
}
