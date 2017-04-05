
use discord::model::Message;
use dpermissions::Permissions;
use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use ::DContext;

pub fn null_cmd(c: &Command, dctx: &DContext, m: &Message, args: &[&str]) {
	// null command
}

const GLOBAL_COMMANDS: &'static [Command] = &[
	Command {
		label: Borrowed("!help"),
		desc: Borrowed("Lists commands and their stuff."),
		help_text: Borrowed("!help [command]"),
		perm: Borrowed("help"),
		run: null_cmd
	},
	Command {
		label: Borrowed("!ping"),
		desc: Borrowed("Pong!"),
		help_text: Borrowed("ping"),
		perm: Borrowed("ping"),
		run: ping_cmd
	},
	Command {
		label: Borrowed("!accept_invite"),
		desc: Borrowed("Accept that invite"),
		help_text: Borrowed("<invite-code or invite-url>"),
		perm: Borrowed("accept_invite"),
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

impl<'a> Command<'a> {
	pub fn should_run(
		&self,
		dctx: &DContext,
		m: &Message,
		c: &Command,
		p: &Permissions,
		args: &[&str]
	) -> bool
	{
		if m.author.id == dctx.app_info.owner.id { return true; }
		if let Some(_) = p.get_group("default") { return true; }
		if p.user_has_perm(m.author.id, &c.perm) { return true; }
		false
	}
}

pub fn parse_cmd(dctx: &DContext, m: &Message, c: &[Command], p: &Permissions) {
	if !m.content.starts_with("!") { return; }
	let mut args_iter = m.content.split(' ');
	let command = args_iter.next().unwrap();

	let mut cmd_iter = GLOBAL_COMMANDS.iter().chain(c.iter());

	if command == "!help" {
		let args: Vec<&str> = args_iter.collect();
		if args.len() == 0 {
			let mut msg = String::from("```");
			for cmd in cmd_iter {
				msg += format!(
					"{}: {}\nUsage: {}\n\n",
					cmd.label,
					cmd.desc,
					cmd.help_text
				).as_str();
			}
			msg.push_str("```");
			dctx.send_message(m.channel_id, msg.as_str(), "", false);
		} else {
			match cmd_iter.find(|ref cmd| args[1] == cmd.label.trim_left_matches('!')) {
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
		return;
	}

	//for cmd in cmd_iter {
	//	if command == cmd.label {
	//		//let args: Vec<&str> = args_iter.collect();
	//		cmd.should_run(dctx, m, &cmd, p, &args_iter.collect());
	//		return;
	//	}
	//}

	match cmd_iter.find(|ref cmd| command == cmd.label) {
		Some(x) => {
			let args: Vec<&str> = args_iter.collect();
			if x.should_run(dctx, m, &x, p, &args) {
				(x.run)(x, dctx, m, &args);
			}
		}
		_ => {}
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
