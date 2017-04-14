use commands;
use commands::CommandResult;
use discord::Discord;
use discord::model::Message;

pub fn help_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	let ch = m.channel_id;
	let mut help_msg = String::from("```");
	if args.len() == 0 {
		for cmd in commands::COMMANDS.iter() {
			let ref label = cmd.label;
			help_msg.push_str("Command: ");
			help_msg.push_str(label.as_str());
			help_msg.push('\n');
			let ref help_txt = cmd.help_txt;
			help_msg.push_str(help_txt.as_str());
			help_msg.push_str("\n\n");
		}
		help_msg.push_str("```");
		let _ = s.send_message(ch, help_msg.as_str(), "", false);
	} else {
		for cmd in commands::COMMANDS.iter() {
			let ref label = cmd.label;
			if label.trim_left_matches('!') == args[0].to_lowercase() {
				let ref help_txt = cmd.help_txt;
				help_msg.push_str(help_txt.as_str());
			}
		}
		if help_msg == "```" {
			let _ = s.send_message(ch, "Command not found.", "", false);
		} else {
			help_msg.push_str("```");
			let _ = s.send_message(ch, help_msg.as_str(), "", false);
		}
	}
	CommandResult::Success
}
