use std::fs::File;
use commands::CommandResult;
use discord::Discord;
use discord::model::Message;

pub fn strokes_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	let chan = m.channel_id;
	if args.len() < 1 {
		return CommandResult::Syntax
	}
	// Takes first 3 characters. The images are 600x600, so it would get spammy pretty quickly.
	// I'd like to find a better way of doing this.
	let mut chars: Vec<char> = vec![];
	'outer: for arg in args {
		for ch in arg.chars() {
			if chars.len() == 3 { break 'outer; }
			chars.push(ch);
		}
	}
	for ch in chars {
		let hex = &format!{"0{:x}.png", ch as u16};
		let filename = &format!{"./data/kanji/png/{}", hex};
		match File::open(filename) {
			Ok(file) => {
				let _ = s.send_file(chan, "", file, hex);
			}
			Err(e) => {
				let _ = s.send_message(chan,
					&format!{"No graphic for character '{}' ({})", ch, hex}, "", false
				);
				println!("No graphic found for character {}:\n{:?}", ch, e);
			}
		}
	}
	CommandResult::Success
}
