use anyhow::Result;

use crate::value::Value;

pub trait Downloader {
    fn download(&self, url: &str) -> Result<Value>;
}

pub struct UrlTextDownloader;

impl Downloader for UrlTextDownloader {
    fn download(&self, _url: &str) -> Result<Value> {
        Ok(Value::String(String::new()))
    }
}

pub struct UrlByteDownloader;

impl Downloader for UrlByteDownloader {
    fn download(&self, _url: &str) -> Result<Value> {
        Ok(Value::Bytes(Vec::new()))
    }
}
