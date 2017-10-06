use serde_json;
use commands::CommandResult;
use discord::Discord;
use discord::model::{Message, ChannelId};
use std::io::ErrorKind;
use http;
use rand::os::OsRng;
use io;


// Some fields are commented out because they are unused, may as well not use the extra memory.

#[derive(Serialize, Deserialize)]
pub struct Jisho {
	//meta: JishoMeta,
	data: Vec<JishoResult>
}

//#[derive(Serialize, Deserialize)]
//pub struct JishoMeta {
	//status: i32
//}

#[derive(Serialize, Deserialize)]
pub struct JishoResult {
	is_common: bool,
	tags: Vec<String>,
	japanese: Vec<JishoWord>,
	senses: Vec<JishoSense>,
	//attribution: JishoAttribution
}

#[derive(Serialize, Deserialize)]
pub struct JishoWord {
	#[serde(default)]
	word: String,
	reading: String
}

#[derive(Serialize, Deserialize)]
pub struct JishoSense {
	english_definitions: Vec<String>,
	parts_of_speech: Vec<String>,
	//links: Vec<JishoLink>,
	tags: Vec<String>,
	//restrictions: Vec<String>,
	//see_also: Vec<String>,
	//antonyms: Vec<String>,
	//source: Vec<String>,
	//info: Vec<String>
}

//#[derive(Serialize, Deserialize)]
//pub struct JishoLink {
	//text: String,
	//url: String
//}


/* This entire struct is commented out because the Jisho API is dumb
 * and will either give you a boolean or a string. I hope I don't end up needing it.
 */

//#[derive(Serialize, Deserialize)]
//pub struct JishoAttribution {
	//jmdict: String,
	//jmnedict: String,
	//dbpedia: String,
//}


fn jisho_search_fs(query: &str) -> Result<Jisho, CommandResult> {
	let path = &format!("./data/jisho/{}.json", query);
	let data = match io::read_file(path) {
		Ok(d) => {
			let age = match io::file_age_seconds(path) {
				Ok(age) => age,
				Err(e) => return Err(CommandResult::IoError(e))
			};
			// 2 weeks
			if age <= 1209600  {
				d
			} else {
				return jisho_search_http(query)
			}
		}
		Err(e) => {
			match e.kind() {
				ErrorKind::NotFound => {
					return jisho_search_http(query)
				}
				_ => return Err(CommandResult::IoError(e))
			}
		}
	};
	let j: Jisho = match serde_json::from_str(&data) {
		Ok(j) => j,
		Err(e) => return Err(CommandResult::JsonError(e))
	};
	Ok(j)
}

fn jisho_search_http(query: &str) -> Result<Jisho, CommandResult> {
	let url = &format!("http://jisho.org/api/v1/search/words?keyword={}", query);
	let resp = match http::get_resp_str(&url) {
		Ok(r) => r,
		Err(e) => return Err(CommandResult::HttpError(e))
	};
	println!("Response from {}", url);
	let j: Jisho = match serde_json::from_str(&resp) {
		Ok(j) => j,
		Err(e) => return Err(CommandResult::JsonError(e))
	};
	cache_results(query, &resp);
	Ok(j)
}

fn cache_results(query: &str, data: &str) -> CommandResult {
	let path = &format!("./data/jisho/{}.json", query);
	match io::write_file(path, data.as_bytes()) {
		Ok(_) => return CommandResult::Success,
		Err(_) => return CommandResult::Warning("Error caching Jisho search results.".into())
	}
}

pub fn format_results(j: Jisho, s: &Discord, ch: &ChannelId, query: &str, i: usize) {
	if j.data.len() == 0 {
		let _ = s.send_message(*ch, "No results found.", "", false);
		return;
	}
	let ref is_common = j.data[i].is_common;
	let ref jp = j.data[i].japanese;
	let ref senses = j.data[i].senses;
	let ref tags = j.data[i].tags;
	let _ = s.send_embed(*ch, "", |mut embed| {
		embed = embed
		.color(5691686)
		.fields(|mut builder| {
			//TODO make the difference between word and reading clearer in formatting
			let mut words_str = String::new();
			for word in jp.iter() {
				words_str.push_str(&format!("{} ({}) ", &word.word, &word.reading));
			}
			if words_str != "" {
				builder = builder.field("Words", &words_str, true);
			}
			let mut tags_str = String::new();
			for tag in tags.iter() {
				tags_str.push_str(&format!("`{}` ", &tag));
			}
			if &tags_str != "" {
				builder = builder.field("Tags", &tags_str, true);
			}
			//TODO fix rogue commas in some definitions
			let mut definitions = String::new();
			for sense in senses.iter() {
				for pos in sense.parts_of_speech.iter() {
					definitions.push_str(&format!("**({})**\n", &pos));
				}
				for (i, def) in sense.english_definitions.iter().enumerate() {
					if i < sense.english_definitions.len() {
						if i == 0 {
							definitions.push_str(&format!("• {}", &def));
						} else {
							definitions.push_str(&format!(", {}, ", &def));
						}
					} else {
						definitions.push_str(&format!("{}", &def));
					}
				}
				definitions.push_str("\n");
			}
			builder.field("Definitions", &definitions, false)
		})
		.author(|builder| {
			builder
			.name(&format!("Jisho results for query '{}'", query.replace("%20", " ")))
			.url(&format!("http://jisho.org/search/{}", query))
			.icon_url("http://assets.jisho.org/assets/touch-icon-017b99ca4bfd11363a97f66cc4c00b1667613a05e38d08d858aa5e2a35dce055.png")
	});
	if *is_common {
		embed = embed.description("Common word");
	}
	embed
	//TODO footer if I feel like it, not high priority
	});
}

fn random_word(level: u32) -> Result<Jisho, CommandResult> {
	use rand::Rng;
	let mut rng = OsRng::new().unwrap();
	let page = match level {
		5 => rng.gen_range(0, 32),
		4 => rng.gen_range(0, 28),
		3 => rng.gen_range(0, 88),
		2 => rng.gen_range(0, 91),
		1 => rng.gen_range(0, 173),
		_ => return Err(CommandResult::InvalidArg("JLPT level must be a positive integer from 1-5.".into()))
	};
	let j = match jisho_search_fs(&format!("%23jlpt-n{}&page={}", level, page)) {
		Ok(j) => j,
		Err(e) => return Err(e)
	};
	Ok(j)
}

pub fn jisho_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	if args.len() < 1 {
		return CommandResult::Syntax
	}
	let mut query = String::new();
	for (i, arg) in args.iter().enumerate() {
		query.push_str(arg);
		if i < args.len() - 1 {
			query.push_str("%20");
		}
	}
	let j = match jisho_search_fs(&query.replace("#", "%23")) {
		Ok(j) => j,
		Err(e) => return e
	};

	format_results(j, &s, &m.channel_id, &query, 0);
	CommandResult::Success
}

pub fn random_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	use rand::Rng;
	let mut rng = OsRng::new().unwrap();
	let index: usize = rng.gen_range(0, 19);
	if args.len() < 1 {
		return CommandResult::Syntax
	}
	let level = match u32::from_str_radix(args[0], 10) {
		Ok(level) => level,
		Err(_) => return CommandResult::InvalidArg("JLPT level must be a positive integer from 1-5.".into())
	};
	let j = match random_word(level) {
		Ok(j) => j,
		Err(e) => return e
	};
	format_results(j, s, &m.channel_id, &format!("#jlpt-n{}", level), index);
	CommandResult::Success
}
