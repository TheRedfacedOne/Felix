use std::fs::File;
use commands::CommandResult;
use discord::Discord;
use discord::model::Message;

pub fn strokes_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	let chan = m.channel_id;
	if args.len() < 1 {
		return CommandResult::Syntax
	}
	let mut chars: Vec<char> = Vec::new();
	'outer: for arg in args.iter() {
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
				let _ = s.send_message(chan,
					format!{"No graphic for character '{}'", ch}.as_str(),
					"", false);
				println!("No graphic found for character {}:\n{:?}", ch, e);
			}
		}
	}
	CommandResult::Success
}
