use std::str;
use actix_web::Result;
use curl::easy::{Easy2, Handler, WriteError};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn get_from(url: &str) -> String {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(url).unwrap();
    easy.perform().unwrap();

    let content = easy.get_ref();
    String::from_utf8_lossy(&content.0).to_string()
}
