use bytes::Bytes;
pub fn download(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> { 
	let body = reqwest::blocking::get(url)?
    .bytes()?;
	Ok(body)
}