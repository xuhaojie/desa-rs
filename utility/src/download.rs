use anyhow::*;
use bytes::{Buf, Bytes};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header, Client};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::result::Result::Ok;

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

pub fn file_name_from_url(url: &str) -> Result<String, anyhow::Error> {
    let fields: Vec<&str> = url.split('/').collect();
    if fields.len() < 2 {
        return Err(anyhow!("bad request"));
    }
    let file_name = fields[fields.len() - 1];
    if file_name.len() > 1 {
        Ok(file_name.to_string())
    } else {
        return Err(anyhow!("can't separte file name"));
    }
}

pub fn download(url: &str) -> Result<Bytes, anyhow::Error> {
    let body = reqwest::blocking::get(url)?.bytes()?;
    Ok(body)
}
use std::fs::File;
use std::io::copy;

pub fn download_file_to_folder_pre(
    url: &str,
    folder: &Path,
    over_write: bool,
) -> Result<(), anyhow::Error> {
    let file_name = file_name_from_url(url)?;
    println!("filename to get: {}", file_name);
    let target_file = folder.join(file_name);
    if target_file.exists() {
        if over_write {
            let _ = std::fs::remove_file(target_file.as_path());
        } else {
            return Err(anyhow!("file exists"));
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
            println!("Error reading {}", e.to_string());
            return Err(anyhow!("request failed"));
        }
    }
}

pub async fn download_file_to_folder(
    url: &str,
    folder: &Path,
    over_write: bool,
) -> Result<(), anyhow::Error> {
    let file_name = file_name_from_url(url)?;
    println!("filename to get: {}", file_name);
    let target_file = folder.join(file_name);
    if target_file.exists() {
        if over_write {
            let _ = std::fs::remove_file(target_file.as_path());
        } else {
            return Err(anyhow!("file exists"));
        }
    }

    println!("source: {}\ntarget: {:?}", url, target_file);

    let client = Client::new();
    let total_size = {
        let resp = client.head(url).send().await?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(anyhow!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ));
        }
    };
    let client = Client::new();
    let mut request = client.get(url);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    if target_file.exists() {
        let size = target_file.metadata()?.len().saturating_sub(1);
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }
    let mut source = request.send().await?;
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&target_file)?;
    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }
    println!("Finish.");
    Ok(())
}
