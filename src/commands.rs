use help;
use strokes;
use jisho;
use hyper;
use discord::Discord;
use discord::model::Message;

pub enum CommandResult {
	Success,
	Syntax,
	HttpError(hyper::error::Error)
}

pub struct Command {
	pub label: String,
	pub desc: String,
	pub help_txt: String,
	pub perm: String,
	pub run: fn(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult
}

impl Command {
	pub fn exec(&self, s: &Discord, m: &Message, args: Vec<&str>){
		let author = format!("{}", m.author.id);
		let ch = m.channel_id;
		if check_perms(author.as_str(), self.perm.as_str()) {
			match (self.run)(s, m, args) {
				CommandResult::Success => {},
				CommandResult::Syntax => {
					let help_msg = &format!{"```{}```", self.help_txt};
					let _ = s.send_message(ch, help_msg, "", false);
				}
				CommandResult::HttpError(e) => {
					let _ = s.send_message(ch,
						&format!{"An HTTP error occured while running {} :(", self.label},
						"", false
					);
					println!("HTTP error while running {}:\n  {:?}", self.label, e);
				}
			}
		}
	}
}

lazy_static! {
	pub static ref COMMANDS: Vec<Command> = vec! [
		Command {
			label: "!help".into(),
			desc: "Shows this help message.".into(),
			help_txt: "Usage:\n  help [command]\nExample:\n  help strokes".into(),
			perm: "felix.help".into(),
			run: help::help_cmd
		},
		Command {
			label: "!strokes".into(),
			desc: "Show the stroke order of a given character.".into(),
			help_txt: "Usage:\n  strokes <chars>\nExample:\n  strokes 猫".into(),
			perm: "felix.strokes".into(),
			run: strokes::strokes_cmd
		},
		Command {
			label: "!jisho".into(),
			desc: "Searches jisho.org for a given query.".into(),
			help_txt: "Usage:\n  jisho <query>\nExample:\n  jisho 猫".into(),
			perm: "felix.jisho".into(),
			run: jisho::jisho_cmd
		}
	];
}

pub fn parse_cmd(s: &Discord, m: &Message) {
	let mut usage: Vec<&str> = m.content.split(' ').collect();
	let label = usage.remove(0).to_lowercase();
	for cmd in COMMANDS.iter() {
		if cmd.label == label {
			cmd.exec(s, m, usage);
			break;
		}
	}
}

fn check_perms(id: &str, perm: &str) -> bool {
	use PERMS;
	if let Some(g) = PERMS.get_group("default") {
		if PERMS.group_has_perm(g, perm) {
			return true
		}
	} else if PERMS.user_has_perm(id, perm) {
		return true
	}
	false
}
