use hyper;
use hyper::client::Client;

pub fn get_resp_str(url: &str) -> Result<String, hyper::error::Error> {
	use std::io::Read;
	let client = Client::new();
	let mut resp = client.get(url).send().unwrap();
	let mut resp_str = String::new();
	resp.read_to_string(&mut resp_str).unwrap();
	Ok(resp_str)
}

