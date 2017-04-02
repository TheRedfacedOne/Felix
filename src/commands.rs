use discord::Discord;
use discord::model::Message;
use dpermissions::Permissions;

pub struct Command {
	pub label: String,
	pub desc: String,
	pub help_txt: String,
	pub perm: String,
	pub run: fn(c: &Command, s: &Discord, m: &Message),
}

impl Command {
	pub fn exec(&self, s: &Discord, m: &Message, c: &Command, p: &Permissions) {
		let author = format!("{}", m.author.id);
		if let Some(group) = p.get_group("default") {
			if p.group_has_perm(group, c.perm.as_str()) { (self.run)(c, s, m) }
		}
		if p.user_has_perm(author.as_str(), c.perm.as_str()) {
			(self.run)(c, s, m)
		}
	}
}

pub fn parse_cmd(s: &Discord, m: &Message, c: &Vec<Command>, p: &Permissions) {
	if !m.content.starts_with("!") {return;}
	let usage: Vec<&str> = m.content.split(' ').collect();
	if usage[0] == "!help" {
		help_cmd(c, s, m);
		return;
	}
	for cmd in c.iter() {
		if usage[0] == cmd.label {
			cmd.exec(s, m, &cmd, p);
		}
	}
}

pub fn help_cmd(cmd_list: &Vec<Command>, s: &Discord, m: &Message) {
	let ch = m.channel_id;
	let usage: Vec<&str> = m.content.split(' ').collect();
	let mut msg = String::from("```");
	if usage.len() == 1 {
		for cmd in cmd_list.iter() {
			msg = msg + format!("{}: {}\nUsage: {}\n\n",
				cmd.label,
				cmd.desc,
				cmd.help_txt
			).as_str();
		}
		msg.push_str("```");
		match s.send_message(ch, msg.as_str(), "", false) {
			Ok(_) => {}
			Err(e) => {
				println!("Unable to send message:\n {:?}", e);
			}
		}
	} else {
		for cmd in cmd_list.iter() {
			if usage[1] == cmd.label.trim_left_matches('!') {
				msg = format!("```{}: {}\nUsage: {}```",
					cmd.label,
					cmd.desc,
					cmd.help_txt
				);
			}
		}
		match s.send_message(ch, msg.as_str(), "", false) {
			Ok(_) => {}
			Err(e) => {
				println!("Unable to send message:\n {:?}", e);
			}
		}
	}
}

pub fn ping_cmd(_c: &Command, s: &Discord, m: &Message) {
	let ch = m.channel_id;
	match s.send_message(ch, "pong", "", false) {
		Ok(_) => {}
		Err(e) => {
			println!("Unable to send message:\n {:?}", e);
		}
	}
}

pub fn jt_cmd(_c: &Command, s: &Discord, m: &Message) {
	let ch = m.channel_id;
	let _ = s.send_embed(ch, "", |embed| { embed
		.title("Jisho results for query 'cat'")
		.thumbnail("http://assets.jisho.org/assets/jisho-logo-v4@2x-7330091c079b9dd59601401b052b52e103978221c8fb6f5e22406d871fcc746a.png")
		.color(u64::from_str_radix("56d926", 16).unwrap())
		.fields(|fields| { fields
			.field("Word", "猫", true)
			.field("Definitions", "cat\nshamisen\ngeisha", true)
			.field("Reading(s)", "ねこ", true)
		})
	});
}
