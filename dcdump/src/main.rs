
extern crate discord;
extern crate dpermissions;
extern crate felix;
extern crate rusqlite;
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use discord::model::{Event, Message, ChannelId};
//use discord::model::UserId;
//use std::env;
//use std::sync::RwLock;
//use std::collections::HashMap;
use felix::DContext;
use felix::commands::Command;
use std::borrow::Cow::Borrowed;
use std::sync::Mutex;
use std::thread;


const DCDUMP_FOLDER: &'static str = "~/.dcdump";


struct LoggingInfo {
	active: bool,
	should_stop: bool,
	//channel: ChannelId,
}

lazy_static! {
	static ref DB: Mutex<rusqlite::Connection> = {
		let mut db_file = PathBuf::from(&*DCDUMP_FOLDER);
		db_file.push("dcdb.sqlite");
		Mutex::new(
			rusqlite::Connection::open(db_file).expect("Failed to open DB")
		)
	};

	static ref LOGGING_MUTEX: Mutex<LoggingInfo> = {
		Mutex::new(
			LoggingInfo {
				active: false,
				should_stop: false,
				//channel: ChannelId(0),
			}
		)
	};
}

const CMD_LIST: &'static [Command] = &[
	Command {
		label: Borrowed("!log_channel"),
		desc: Borrowed("Log that channel"),
		help_text: Borrowed("<ChannelId>"),
		perm: Borrowed("dcdump.log_channel"),
		run: log_channel_cmd
	},
	Command {
		label: Borrowed("!stop_logging"),
		desc: Borrowed("Stop logging any channels"),
		help_text: Borrowed("hmm"),
		perm: Borrowed("dcdump.log_channel"), // intended
		run: stop_logging_cmd
	},
];

fn stop_logging_cmd(_: &Command, dctx: &DContext, m: &Message, _: &[&str]) {
	let msg = {
		let mut info = LOGGING_MUTEX.lock().unwrap();

		if info.active {
			if info.should_stop {
				"Error: already stopping"
			} else {
				info.should_stop = true;
				"Logging should stop soon"
			}
		} else {
			"Error: logging not active"
		}
	};

	dctx.send_message(m.channel_id, msg, "", false);
}

fn log_channel_cmd(_: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	if args.len() != 1 {
		dctx.send_message(m.channel_id, "Error: provide channel_id arg", "", false);
		return;
	}

	let channel_to_log = match u64::from_str_radix(args[1], 10) {
		Ok(num) => { ChannelId(num) },
		Err(err) => {
			let msg = format!("Error: failed to parse channel-id ({:?})", err);
			dctx.send_message(m.channel_id, msg.as_str(), "", false);
			return;
		}
	};

	if dctx.state.find_channel(channel_to_log).is_none() {
		dctx.send_message(m.channel_id, "Error: can't find channel", "", false);
		return;
	}

	let is_active = {
		let mut info = LOGGING_MUTEX.lock().unwrap();
		if info.active {
			true
		} else {
			info.active = true;
			false
		}
	};

	if is_active {
		dctx.send_message(m.channel_id, "Error: already logging", "", false);
		return;
	}

	let builder = thread::Builder::new()
		.name("discord_logging_thread".into());

	let _ = builder.spawn(logging_thread).unwrap();
}

fn logging_thread() {

}

fn main() {
	std::fs::create_dir_all(&DCDUMP_FOLDER).expect("Failed to create directories");

	DB.lock().unwrap().execute("CREATE TABLE messages (
		id                  INTEGER PRIMARY KEY NOT NULL,
		channel_id          INTEGER NOT NULL,
		author_id           INTEGER NOT NULL,
		content             TEXT    NOT NULL
	)", &[]).unwrap();
	DB.lock().unwrap().execute("CREATE TABLE attachments (
		id                  TEXT    PRIMARY KEY NOT NULL,
		message_id          INTEGER NOT NULL,
		filename            TEXT    NOT NULL,
		url                 TEXT    NOT NULL,
		proxy_url           TEXT    NOT NULL,
		size                INTEGER NOT NULL,
		dimensions0         INTEGER,
		dimensions1         INTEGER
	)", &[]).unwrap();

	println!("Database is setup.");

	let mut dctx = DContext::from_bot_token(
		&std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env-var.")
	);

	let mut perms_file = PathBuf::from(DCDUMP_FOLDER);
	perms_file.push("perms.json");
	let perms = felix::init_perms(perms_file);
	println!("## dcdump is running.");

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
