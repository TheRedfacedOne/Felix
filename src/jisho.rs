use serde_json;
use serde_json::Value;
use hyper::client::Client;
use commands::CommandResult;
use discord::Discord;
use discord::model::Message;

// Maybe I'll make this less terrible eventually.

pub fn jisho_cmd(s: &Discord, m: &Message, args: Vec<&str>) -> CommandResult {
	use std::io::Read;
	if args.len() < 1 {
		return CommandResult::Syntax
	}
	let client = Client::new();
	let mut url = String::from("http://jisho.org/api/v1/search/words?keyword=");
	let mut query = String::new();
	for (i, arg) in args.iter().enumerate() {
		query.push_str(arg);
		if i < args.len() - 1 {
			query.push_str("%20");
		}
	}
	url.push_str(&query);
	let mut resp = match client.get(&url).send() {
		Ok(resp) => resp,
		Err(e) => return CommandResult::HttpError(e)
	};
	println!("Response from {}:\n  Status:{}", url, resp.status);
	let ch = m.channel_id;
	let mut resp_string = String::new();
	let _ = resp.read_to_string(&mut resp_string).unwrap();
	let jisho_json: Value = serde_json::from_str(&resp_string).unwrap();
	let ref data = jisho_json["data"];
	if data.is_null() {
		let _ = s.send_message(ch,
			&format!("No results for query '{}'", query.replace("%20", " ")), "", false);
		return CommandResult::Success
	}
	let ref word = data[0];
	let ref jp = word["japanese"];
	let ref senses = word["senses"];
	let _ = s.send_embed(ch, "", |mut embed| {
		embed = embed.title(&format!{"Jisho results for query '{}'", query.replace("%20", " ")})
			.thumbnail("http://assets.jisho.org/assets/favicon-062c4a0240e1e6d72c38aa524742c2d558ee6234497d91dd6b75a182ea823d65.ico")
			.url(&format!{"http://jisho.org/search/{}", query})
			.color(5691686)
			.fields(|mut builder| {
				let ref w = jp[0]["word"];
				let ref r = jp[0]["reading"];
				if !w.is_null() {
					builder = builder.field("Word", w.as_str().unwrap(), false);
				}
				if !r.is_null() {
					builder = builder.field("Reading", r.as_str().unwrap(), false);
				}
				let mut definitions = String::new();
				for sense in senses.as_array().unwrap().iter() {
					for def in sense["english_definitions"].as_array().unwrap().iter() {
						definitions.push_str(def.as_str().unwrap());
						definitions.push_str(", ");
					}
				}
				builder.field("Definitions", definitions.trim_right_matches(", "), false)
			}
		);
		if word["is_common"].as_bool().unwrap() {
			embed = embed.description("Common word");
		}
		let mut footer_txt = String::new();
		for (i, j) in jp.as_array().unwrap().iter().enumerate() {
			if i == 0 { continue; }
			else if i == 1 {
				footer_txt.push_str("Other forms: ");
			}
			if !j["word"].is_null() {
				footer_txt.push_str(&format!{"{} 【{}】",
					j["word"].as_str().unwrap(), j["reading"].as_str().unwrap()}
				);
			} else {
				footer_txt.push_str(j["reading"].as_str().unwrap());
			}
			footer_txt.push(' ');
		}
		embed.footer(|footer| {
			footer.text(footer_txt.as_str())
		})
	});
	CommandResult::Success
}
