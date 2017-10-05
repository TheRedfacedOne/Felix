use commands;
use commands::CommandResult;
use discord::Discord;
use discord::model::Message;

pub fn help_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	let ch = m.channel_id;
	let mut help_msg = String::new();
	if args.len() == 0 {
		for cmd in commands::COMMANDS.iter() {
			let ref label = cmd.label;
			let ref help_txt = cmd.help_txt;
			help_msg.push_str(&format!{"Command: {}\n{}\n\n", label, help_txt});
		}
		let _ = s.send_message(ch, &help_msg, "", false);
	} else {
		for cmd in commands::COMMANDS.iter() {
			let ref label = cmd.label;
			if label.trim_left_matches('!') == args[0].trim_left_matches('!').to_lowercase() {
				let ref help_txt = cmd.help_txt;
				help_msg.push_str(&help_txt);
			}
		}
		if help_msg == "" {
			let _ = s.send_message(ch, "Command not found.", "", false);
		} else {
			let _ = s.send_message(ch, &help_msg, "", false);
		}
	}
	CommandResult::Success
}
