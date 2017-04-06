
extern crate discord;
extern crate dpermissions;
extern crate felix;
extern crate rusqlite;
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use discord::model::{Event, Message, UserId, ChannelId};
//use discord::model::UserId;
//use std::env;
//use std::sync::RwLock;
//use std::collections::HashMap;
use felix::DContext;
use felix::commands::Command;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use std::borrow::Cow::Borrowed;
use std::sync::Mutex;


static SCRAPING_ACTIVE: AtomicBool = ATOMIC_BOOL_INIT;

const DCDUMP_FOLDER: &'static str = "~/.dcdump";

lazy_static! {
	static ref DB: Mutex<rusqlite::Connection> = {
		let mut db_file = PathBuf::from(&*DCDUMP_FOLDER);
		db_file.push("dcdb.sqlite");
		Mutex::new(
			rusqlite::Connection::open(db_file).expect("Failed to open DB")
		)
	};
}

const CMD_LIST: &'static [Command] = &[
	Command {
		label: Borrowed("!scrape_channel"),
		desc: Borrowed("Scrape that channel"),
		help_text: Borrowed("<ChannelId>"),
		perm: Borrowed("dcdump.scrape_channel"),
		run: scrape_channel
	},
];

fn scrape_channel(c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	if args.len() != 1 {
		dctx.send_message(m.channel_id, "Error: provide channel_id arg", "", false);
		return;
	}

	let channel_u64 = u64::from_str_radix(args[1], 10);

	if channel_u64.is_err() {
		let err = format!("Error: failed to parse channel-id string to u64 ({:?})", channel_u64);
		dctx.send_message(m.channel_id, err.as_str(), "", false);
		return;
	}

	let channel_to_scrape = ChannelId(channel_u64.unwrap());

	if dctx.state.find_channel(channel_to_scrape).is_none() {
		dctx.send_message(m.channel_id, "Error: can't find channel", "", false);
		return;
	}

	if SCRAPING_ACTIVE.swap(true, Ordering::SeqCst) {
		dctx.send_message(m.channel_id, "Error: something is currently being scraped", "", false);
		return;
	}
	// the SCRAPING_ACTIVE bool is now true


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
