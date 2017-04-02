
use discord::Discord;
use discord::model::Message;
use commands::Command;
use std::fs::File;

pub fn strokes_cmd(c: &Command, s: &Discord, m: &Message) {
	let usage: Vec<&str> = m.content.split(' ').collect();
	let chan = m.channel_id;
	if usage.len() < 2 {
		let response = format!("Usage: {}", c.help_txt.as_str());
		let _ = s.send_message(chan, response.as_str(), "", false);
		return;
	}
	let mut chars: Vec<char> = Vec::new();
	'outer: for (i, arg) in usage.iter().enumerate() {
		if i == 0 { continue; }
		for ch in arg.chars() {
			if chars.len() < 3 { chars.push(ch); }
			else { break 'outer; }
		}
	}
	for ch in chars {
		let mut hex = ch.escape_unicode().to_string();
		hex = String::from(hex.trim_left_matches("\\u{"));
		hex = String::from(hex.trim_right_matches("}"));
		hex.insert(0, '0');
		hex.push_str(".png");
		let mut filename = String::from("./data/kanji/png/");
		filename.push_str(hex.as_str());
		match File::open(filename.as_str()) {
			Ok(file) => {
				let _ = s.send_file(chan, "", file, hex.as_str());
			}
			Err(e) => {
				let response = format!("Error opening image for character `{}`", ch);
				println!("Error opening image {}:\n{}", filename, e);
				let _ = s.send_message(chan, response.as_str(), "", false);
			}
		}
	}
}
