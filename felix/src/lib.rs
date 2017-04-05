extern crate discord;
extern crate dpermissions;

//use discord::Discord;
use dpermissions::Permissions;
use dpermissions::Error;

pub struct DContext {
	pub session: discord::Discord,
	pub connection: discord::Connection,
	pub state: discord::State,
	/// `app_info` should NOT be used to check strings at all. Only cached owner-id.
	pub app_info: discord::model::ApplicationInfo
}

impl DContext {
	pub fn from_bot_token(token: &str) -> DContext {
		let mut session = Discord::from_bot_token(token).expect("Login failed. Invalid token?");
		let (mut connection, ready) = session.connect().expect("Connection failed.");
		let mut app_info = session.get_application_info().unwrap();
		DContext {
			session: session,
			connection: connection,
			state: discord::State::new(ready),
			app_info: app_info
		}
	}

	pub fn send_message(&self, c: ChannelId, text: &str, nonce: &str, tts: bool) {
		match self.session.send_message(c, text, nonce, tts) {
			Ok(_) => {}
			Err(e) => {
				println!("Unable to send message:\n {:?}", e);
			}
		}
	}

	pub fn send_file<R: Read>(&self, c: ChannelId, text: &str, file: R, filename: &str) {
		match self.session.send_file(c, text, file, filename) {
			Ok(_) => {}
			Err(e) => {
				println!("Unable to send message:\n {:?}", e);
			}
		}
	}
}

pub fn init_perms(dctx: &DContext, path: &str) -> Permissions {
	let perms = match dpermissions::load_perms(path) {
		Ok(p) => p,
		Err(err) => {
			match err {
				Error::Io(_) => {
					let app_info = dctx.session.get_application_info().unwrap();
					let owner_id = format!("{}", app_info.owner.id);
					let perms = dpermissions::create_default(owner_id);
					match dpermissions::save_perms(path, &perms) {
						Ok(_) => return perms,
						Err(e) => {
							panic!("Error saving default {}:\n{}", path, e);
						}
					}
				}
				Error::Parse(e) => {
					panic!("Error parsing {}:\n{}", path, e);
				}
			}
		}
	};
	perms
}
