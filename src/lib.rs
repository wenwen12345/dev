pub mod func;
use serde::Deserialize;

use base64::{engine::general_purpose, Engine as _};
use clap::{Parser, Subcommand};
use func::*;
use reqwest::blocking::{get, Client};
use std::io::{self, Read, Write};

#[derive(Deserialize)]
struct IpInfo {
    city: String,
}

// an app to make file to text
#[derive(Parser, Debug)]
#[command(author = "wen", version = "V1.1.0", about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub mode: Config,
}

#[derive(Subcommand, Debug)]
pub enum Config {
    // encode base64
    B64encode,
    // decode base64
    B64decode,
    // weather from wttr.in
    Wttr,
}

impl Base64 for App {
    // need to parse before call
    fn encode(&self) {
        if let Config::B64encode = &self.mode {
            let bytes = general_purpose::STANDARD_NO_PAD.encode(&get_as_byte_vec());
            write_as_byte_vec(bytes.as_bytes());
        }
    }
    // need to parse before call
    fn decode(&self) {
        if let Config::B64decode = &self.mode {
            write_as_byte_vec(
                &general_purpose::STANDARD_NO_PAD
                    .decode(&get_as_byte_vec())
                    .expect("the text is wrong"),
            );
        }
    }
}

impl Wttr for App {
    fn wttr(&self) {
        let json = get("http://ip-api.com/json/")
            .expect("Are you online")
            .text()
            .expect("Are you online");

        let info: IpInfo = serde_json::from_str(json.as_str()).unwrap();
        let city = info.city;
        println!("{city}");
        println!(
            "{}",
            Client::builder()
                .build()
                .unwrap()
                .get(format!("https://wttr.in/{}", city))
                .header("User-Agent", "curl/7.68.0")
                .send()
                .expect("Are you online")
                .text()
                .expect("Are you online")
                .as_str()
        );
    }
}

impl Run for App {
    fn run(&self) {
        match &self.mode {
            Config::B64encode => self.encode(),
            Config::B64decode => self.decode(),
            Config::Wttr => self.wttr(),
        }
    }
}

fn get_as_byte_vec() -> Vec<u8> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).unwrap();
    buffer
}

fn write_as_byte_vec(buffer: &[u8]) {
    io::stdout().write_all(buffer).unwrap();
}
