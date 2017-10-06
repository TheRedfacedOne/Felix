use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;
use std;

// I want to change this so I don't need to open a file once to get its age, and again to read/write it.
// Too lazy for now.

pub fn write_file(path: &str, bytes: &[u8]) -> Result<(), std::io::Error> {
	let mut file = File::create(path)?;
	file.write_all(bytes)?;
	Ok(())
}

pub fn read_file(path: &str) -> Result<String, std::io::Error> {
	let file = File::open(path)?;
	let mut reader = BufReader::new(file);
	let mut contents = String::new();
	reader.read_to_string(&mut contents)?;
	Ok(contents)
}

pub fn file_age_seconds(path: &str) -> Result<u64, std::io::Error> {
	let file = File::open(path)?;
	let meta = file.metadata()?;
	// I don't foresee anyone running this on a platform that doesn't have this metadata.
	// Fingers crossed.
	let modi = meta.modified().unwrap();
	let now = SystemTime::now();
	if let Ok(dur) = now.duration_since(modi) {
		return Ok(dur.as_secs())
	} else {
		// The only way this goes wrong is the system time going backwards, which doesn't really
		// cause any problems, so just return 0.
		return Ok(0)
	}
}
