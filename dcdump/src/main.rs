extern crate discord;
extern crate dpermissions;
extern crate felix;
extern crate rusqlite;

use std::path::PathBuf;
use discord::Discord;
use discord::model::{Message, UserId, ChannelId};
use dpermissions::Permissions;
//use discord::model::UserId;
//use std::env;
//use std::sync::RwLock;
//use std::collections::HashMap;
use felix::DContext;
use felix::commands::Command;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT}

//static ChannelStatuses: RwLock<HashMap<discord::model::ChannelId, ()>> = RwLock::new(
//    HashMap::new()
//);

static mut DCDATA_FOLDER: &str = "";
static SCRAPING_ACTIVE: AtomicBool = ATOMIC_BOOL_INIT;
//static mut DB: Option<rusqlite::Connection> = None;
static mut DB: rusqlite::Connection = unsafe { std::mem::unitialized() };

const cmd_list: &'static [Command] = &[
	Command {
		label: "!scrape_channel".into(),
		desc: "Scrape that channel".into(),
		help_text: "<ChannelId>".into(),
		perm: "dcdump.scrape_channel".into(),
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
		dctx.send_message(m.channel_id, err, "", false);
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
	unsafe {
		DCDATA_FOLDER = std::env::args().nth(1).unwrap_or("./dcdata");
	}

	std::fs::create_dir_all(DCDATA_FOLDER).unwrap();
	println!("DCDATA_FOLDER = {:?}", DCDATA_FOLDER);

	let mut db_file = PathBuf::from(DCDATA_FOLDER);
	db_file.push("dcdb.sqlite");

	unsafe {
		DB = Some(rusqlite::Connection::open(db_file).unwrap());
	}

	DB.execute("CREATE TABLE messages (
		id                  INTEGER PRIMARY KEY NOT NULL,
		channel_id          INTEGER NOT NULL,
		author_id           INTEGER NOT NULL,
		content             TEXT    NOT NULL
	)", &[]).unwrap();

	DB.execute("CREATE TABLE attachments (
		id                  TEXT    PRIMARY KEY NOT NULL,
		message_id          INTEGER NOT NULL,
		filename            TEXT    NOT NULL,
		url                 TEXT    NOT NULL,
		proxy_url           TEXT    NOT NULL,
		size                INTEGER NOT NULL,
		dimensions0         INTEGER,
		dimensions1         INTEGER
	)", &[]).unwrap();

	println!("Database opened.");

	let mut dctx = DContext::from_bot_token(
		&std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env-var.")
	);

	let perms = felix::init_perms(&dctx, "./perms.json");
	println!("## dcdump is running.");

	loop {
		match dctx.connection.recv_event() {
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
