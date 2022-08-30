use bytes::Bytes;

pub fn get_final_url(url:&str) -> String {
	
	let client = reqwest::blocking::Client::new();
	let mut headers = reqwest::header::HeaderMap::new();

	headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
	headers.insert(reqwest::header::ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
	headers.insert(reqwest::header::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,en-US;q=0.5,en;q=0.3".parse().unwrap());
	headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
	headers.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/68.0.3440.106 Safari/537.36".parse().unwrap());

	let res = client
		.get(url)
		.headers(headers)
		.send();

	match res {
		Ok(r) => {
			return r.url().as_str().to_string();
		},
		Err(e) => {
			return url.to_string();
		}
	}
}
/*
pub fn get_final_url(&self) -> String{
	let client = reqwest::Client::new();
	client.get(url)
	let body = reqwest::blocking::get(url)?
    .bytes()?;

}
*/
pub fn download(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> { 
	let body = reqwest::blocking::get(url)?
    .bytes()?;
	Ok(body)
}