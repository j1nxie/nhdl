use std::error::Error;
use std::path::Path;
use std::fs;

use select::document::Document;

use reqwest::Client;

#[macro_use]
extern crate clap;
use clap::App;

mod downloader;
mod nhentai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    // get url from user input
    let mut input = matches.value_of("INPUT").unwrap().to_string();
    loop {
        match &input.trim().parse::<u32>() {
            Ok(id) => {
                input = format!("https://nhentai.net/g/{}/", id);
                break;
            },
            Err(_) => {
                match &input.trim().contains("https://nhentai.net/g/") {
                    true => break,
                    false => println!("[error] invalid url or id!")
                }
            }
        }
    }

    // get html source from reqwest
    let client = Client::builder().build()?;
    let response = client.get(&input).send().await?;
    let body = response.text().await?;
    let document = Document::from(body.as_str());

    // get metadata 
    let title = nhentai::metadata(document);
    nhentai::print_status(title.clone());

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
    downloader::downloader(paths, title.id, client).await;

    Ok(())
}
