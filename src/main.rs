use std::error::Error;
use std::path::Path;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;

use reqwest::Client;

#[macro_use]
extern crate clap;
use clap::App;

mod downloader;
mod nhentai;
mod html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    // get url from user input
    let input = matches.value_of("INPUT").unwrap().to_string();
    let mut list = vec![];
    loop {
        match &input.trim().parse::<u32>() {
            Ok(id) => {
                list.push(format!("https://nhentai.net/g/{}/", id));
                break;
            },
            Err(_) => {
                match &input.trim().contains("https://nhentai.net/g/") {
                    true => {
                        list.push(input);
                        break;
                    },
                    false => println!("[error] invalid url or id!")
                }
            }
        }
    }

    // get html source from reqwest
    let client = Client::builder().build()?;
    for doujin in list {
        let document = html::get_document(doujin, client.clone()).await;

        // get metadata 
        let mut title: nhentai::NH = Default::default();
        title.get_title(document.clone());
        title.get_id(document.clone());
        title.get_tags(document.clone());
        title.get_page(document.clone());
        title.get_gallery(document.clone());
        title.print_status();

        // initialize download directory
        let dir = format!("./{}", title.id);
        if !Path::new(&dir).exists() {
            fs::create_dir(dir)?;
        }

        // initialize download links vector
        let mut paths = vec![];
        for i in 1..=title.pages.parse::<u8>().unwrap() {
            paths.push(format!("https://i.nhentai.net/galleries/{}/{}.jpg", title.gallery_id, i));
        }
        downloader::downloader(paths, title.id, client.clone()).await;
    }

    Ok(())
}
