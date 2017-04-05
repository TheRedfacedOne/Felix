
use discord::model::Message;
use commands::Command;
use std::fs::File;
use std::path::PathBuf;
//use std::iter::FromIterator;
use ::DContext;

pub fn strokes_cmd(c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	let chan = m.channel_id;
	if args.len() < 1 {
		let response = format!("Usage: {}", c.help_text);
		dctx.send_message(chan, response.as_str(), "", false);
		return;
	}
	let mut chars: Vec<char> = Vec::new();
	'outer: for (i, arg) in args.iter().enumerate() {
		if i == 0 { continue; }
		for ch in arg.chars() {
			if chars.len() < 3 { chars.push(ch); }
			else { break 'outer; }
		}
	}
	//let chars = Vec::from_iter(args[0].chars().take(3));
	for ch in chars {
		let filename;
		if ch as u32 <= 0xFFFF {
			filename = format!("0{:04x}.png", ch as u32);
		} else {
			filename = format!("0{:06x}.png", ch as u32);
		}
		let mut filepath = PathBuf::from(".");
		filepath.push("data");
		filepath.push("kanji");
		filepath.push("png");
		filepath.push(&filename);
		match File::open(filepath) {
			Ok(file) => {
				dctx.send_file(chan, "", file, filename.as_str());
			}
			Err(e) => {
				let response = format!("Error opening image for character `{}`", ch);
				println!("Error opening image {}:\n{}", filename, e);
				dctx.send_message(chan, response.as_str(), "", false);
			}
		}
	}
}
