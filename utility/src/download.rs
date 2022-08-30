use bytes::{Bytes, Buf};

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
use std::thread::sleep;
use std::time::Duration;
use std::io::prelude::*;
use std::io::{self, BufWriter, Write};

pub fn progress(percent: usize) {
    let mut _str: [char;101] = [' ';101];
    let nchar:[char;4] = ['-','\\', '|', '/',];
    let mut sw = BufWriter::new(io::stdout());
	let progress_v = percent;
    {
        // 注意 /r xxxx 和 xxxx /r的区别： 前者是先定位到行头再输出xxxx， 后者则是先输出xxxx，再定位到行头。
        // 前者再mac osx上能正确地看到输出，后者则一直不显示输出。
        sw.write_fmt(format_args!("\r{}",'['));
        _str[progress_v] = '=';
        for i in _str.iter(){
            sw.write_fmt(format_args!("{}", i));
        }
        sw.write_fmt(format_args!("{}", ']'));
        let _ = sw.write_fmt(format_args!("\t({:3}%)\t[{}]", progress_v, nchar[progress_v % 4]));
        sw.flush();

    }
}

pub fn download(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> { 
	let body = reqwest::blocking::get(url)?
    .bytes()?;
	Ok(body)
}
use tempfile::Builder;
use std::io::copy;
use std::fs::File;

pub fn download_file(url:&str,file_name: &str) -> std::io::Result<()> {
	let tmp_dir = Builder::new().prefix("example").tempdir()?;

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

			let mut dest = File::create(file_name).unwrap();
			let content =  r.bytes();
			if let Ok(data) = content {
				copy(&mut data.reader(), &mut dest)?;
			}
			Ok(())			
		},
		Err(e) => {
			println!("Error reading");
			return Err(io::Error::new(io::ErrorKind::Other,"request failed {}"));
		}
	}


}

