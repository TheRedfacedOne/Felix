
use discord::Discord;
use discord::model::Message;
use dpermissions::Permissions;
use std::borrow::Cow;
//use felix::*;

pub fn null_cmd(c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	// null command
}

const GLOBAL_COMMANDS: &'static [Command] = &[
	Command {
		label: "!help".into(),
		desc: "Lists commands and their stuff.".into(),
		help_text: "!help [command]".into(),
		perm: "help".into(),
		run: null_cmd
	},
	Command {
		label: "!ping".into(),
		desc: "Pong!".into(),
		help_text: "ping".into(),
		perm: "ping".into(),
		run: ping_cmd
	},
	Command {
		label: "!accept_invite".into(),
		desc: "Accept that invite".into(),
		help_text: "<invite-code or invite-url>".into(),
		perm: "accept_invite".into(),
		run: accept_invite_cmd
	},
];

pub struct Command<'a> {
	pub label: Cow<'a, str>,
	pub desc: Cow<'a, str>,
	pub help_text: Cow<'a, str>,
	pub perm: Cow<'a, str>,
	pub run: fn(c: &Command, dctx: &DContext, m: &Message, args: &[&str]),
}

impl Command<'a> {
	pub fn exec(&self, dctx: &DContext, m: &Message, c: &Command, p: &Permissions, args: &[&str]) {
		let author = format!("{}", m.author.id);
		if let Some(group) = p.get_group("default") {
			if p.group_has_perm(group, c.perm) { (self.run)(c, dctx, m, args) }
		}
		if p.user_has_perm(author.as_str(), c.perm) {
			(self.run)(c, dctx, m, args)
		}
	}
}

pub fn parse_cmd(dctx: &DContext, m: &Message, c: &[Command], p: &Permissions) {
	if !m.content.starts_with("!") { return; }
	let args_iter = m.content.split(' ');
	let command = args_iter.next();

	let mut cmd_iter = GLOBAL_COMMANDS.iter().chain(c.iter());

	if command == "!help" {
		help_cmd(cmd_iter, dctx, m, &args_iter.collect());
		return;
	}

	//for cmd in cmd_iter {
	//	if command == cmd.label {
	//		//let args: Vec<&str> = args_iter.collect();
	//		cmd.exec(dctx, m, &cmd, p, &args_iter.collect());
	//		return;
	//	}
	//}

	match cmd_iter.find(|&&cmd| command == cmd.label) {
		Some(x) => {
			//let args: Vec<&str> = args_iter.collect();
			cmd.exec(dctx, m, &cmd, p, &args_iter.collect());
		}
		_ => {}
	}
}

pub fn help_cmd<'a, I>(cmd_iter: &I, dctx: &DContext, m: &Message, args: &[&str])
	where I: Iterator<Item=&'a Command>
{
	if args.len() == 0 {
		let mut msg = String::from("```");
		for cmd in cmd_iter {
			msg += format!(
				"{}: {}\nUsage: {}\n\n",
				cmd.label,
				cmd.desc,
				cmd.help_txt
			);
		}
		msg.push_str("```");
		dctx.send_message(m.channel_id, msg.as_str(), "", false);
	} else {
		match cmd_iter.find(|&&cmd| args[1] == cmd.label.trim_left_matches('!')) {
			Some(x) => {
				let msg = format!(
					"```{}: {}\nUsage: {}```",
					x.label,
					x.desc,
					x.help_text
				);
				dctx.send_message(m.channel_id, msg.as_str(), "", false);
			}
			None => {
				dctx.send_message(m.channel_id, "Error: command not found", "", false);
			}
		}
	}
}

pub fn accept_invite_cmd(_c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	if args.len() != 1 {
		dctx.send_message(m.channel_id, "Error: provide invite arg", "", false);
		return;
	}

	match dctx.session.accept_invite(args[0]) {
		Ok(invite) => {
			println!("## Accepted invite `{}`\n{:?}", args[0], invite);
			let msg = format!("Successfully accepted invite `{}`", args[0]);
			dctx.send_message(m.channel_id, msg.as_str(), "", false);
		}
		Err(err) => {
			println!("Invite error: {:?}", err);
			dctx.send_message(m.channel_id, "Error: some error that I won't tell", "", false);
		}
	}
}

pub fn ping_cmd(_c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	dctx.send_message(m.channel_id, "pong", "", false);
}

pub fn jt_cmd(_c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	let result = dctx.session.send_embed(m.channel_id, "", |embed| {
		embed
		.title("Jisho results for query 'cat'")
		.thumbnail("http://assets.jisho.org/assets/jisho-logo-v4@2x-7330091c079b9dd59601401b052b52e103978221c8fb6f5e22406d871fcc746a.png")
		.color(0x56d926)//u64::from_str_radix("56d926", 16).unwrap())
		.fields(|fields| { fields
			.field("Word", "猫", true)
			.field("Definitions", "cat\nshamisen\ngeisha", true)
			.field("Reading(s)", "ねこ", true)
		})
	});

	match result {
		Ok(_) => {}
		Err(e) => {
			println!("Unable to send message:\n {:?}", e);
		}
	}
}
