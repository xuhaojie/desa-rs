pub fn download(rul: &str) -> Result<(), Box<dyn std::error::Error>> { 
	let body = reqwest::blocking::get("https://www.rust-lang.org")?
    .text()?;
	println!("body = {:?}", body);
	Ok(())
}