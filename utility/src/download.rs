use bytes::{Buf, Bytes};
//use downloader::Downloader;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub fn get_redirected_url(url: &str) -> std::io::Result<String> {
    let client = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        "gzip, deflate".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        "zh-CN,zh;q=0.8,en-US;q=0.5,en;q=0.3".parse().unwrap(),
    );
    headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/68.0.3440.106 Safari/537.36".parse().unwrap());

    let res = client.get(url).headers(headers).send();

    match res {
        Ok(r) => Ok(r.url().as_str().to_string()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
    }
}

pub fn file_name_from_url(url: &str) -> std::io::Result<String> {
    let fields: Vec<&str> = url.split('/').collect();
    if fields.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "bad request"));
    }
    let file_name = fields[fields.len() - 1];
    if file_name.len() > 1 {
        Ok(file_name.to_string())
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "can't separte file name",
        ));
    }
}

pub fn download(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.bytes()?;
    Ok(body)
}
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

pub fn download_file(url: &str, folder: &Path, over_write: bool) -> std::io::Result<()> {
    let file_name = file_name_from_url(url)?;
    println!("get filename: {}", file_name);
    let over_write = true;
    let target_file = folder.join(file_name);
    if target_file.exists() {
        if over_write {
            std::fs::remove_file(target_file.as_path());
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "file exists"));
        }
    }

    let client = reqwest::blocking::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        "gzip, deflate".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        "zh-CN,zh;q=0.8,en-US;q=0.5,en;q=0.3".parse().unwrap(),
    );
    headers.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/68.0.3440.106 Safari/537.36".parse().unwrap());

    let res = client.get(url).headers(headers).send();

    match res {
        Ok(r) => {
            let mut dest = File::create(target_file).unwrap();
            let content = r.bytes();
            if let Ok(data) = content {
                copy(&mut data.reader(), &mut dest)?;
            }
            Ok(())
        }
        Err(e) => {
            println!("Error reading");
            return Err(io::Error::new(io::ErrorKind::Other, "request failed {}"));
        }
    }
}
/*
// Define a custom progress reporter:
struct SimpleReporterPrivate {
    last_update: std::time::Instant,
    max_progress: Option<u64>,
    message: String,
}
struct SimpleReporter {
    private: std::sync::Mutex<Option<SimpleReporterPrivate>>,
}

impl SimpleReporter {
    fn create() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            private: std::sync::Mutex::new(None),
        })
    }
}

impl downloader::progress::Reporter for SimpleReporter {
    fn setup(&self, max_progress: Option<u64>, message: &str) {
        let private = SimpleReporterPrivate {
            last_update: std::time::Instant::now(),
            max_progress,
            message: message.to_owned(),
        };

        let mut guard = self.private.lock().unwrap();
        *guard = Some(private);
    }

    fn progress(&self, current: u64) {
        if let Some(p) = self.private.lock().unwrap().as_mut() {
            let max_bytes = match p.max_progress {
                Some(bytes) => format!("{:?}", bytes),
                None => "{unknown}".to_owned(),
            };
            if p.last_update.elapsed().as_millis() >= 1000 {
                println!("{} of {} bytes. [{}]", current, max_bytes, p.message);
                p.last_update = std::time::Instant::now();
            }
        }
    }

    fn set_message(&self, message: &str) {
        println!("Message changed to: {}", message);
    }

    fn done(&self) {
        let mut guard = self.private.lock().unwrap();
        *guard = None;
        println!("[DONE]");
    }
}
*/

pub fn download_file_depend_openssl(
    url: &str,
    folder: &Path,
    over_write: bool,
) -> std::io::Result<()> {
    /*
        let file_name = file_name_from_url(url)?;
        println!("get filename: {}", file_name);
        let over_write = true;
        let target_file = folder.join(file_name);
        if target_file.exists() {
            if over_write {
                std::fs::remove_file(target_file.as_path());
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "file exists"));
            }
        }

        let mut downloader = Downloader::builder()
            .download_folder(folder)
            .parallel_requests(1)
            .build()
            .unwrap();

        let dl = downloader::Download::new(url);
        let dl = dl.progress(SimpleReporter::create());

        let result = downloader.download(&[dl]).unwrap();

        for r in result {
            match r {
                Err(e) => {
                    println!("Error: {}", e.to_string());
                    return Err(io::Error::new(io::ErrorKind::Other, "failed download file"));
                }
                Ok(s) => {
                    //				println!("Success: {}", &s);
                    return Ok(());
                }
            };
        }
    */
    Ok(())
}
