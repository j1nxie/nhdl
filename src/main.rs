use std::error::Error;
use std::path::Path;
use std::fs;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Name};
use reqwest::Client;

mod downloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get url from user input
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input)
            .expect("[error] unable to read user input");
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
        input = String::new(); // resets input
   }

    // get html source from reqwest
    let client = Client::builder().build()?;
    let response = client.get(&input).send().await?;
    let body = response.text().await?;
    let document = Document::from(body.as_str());

    // get titles
    for node in document.find(Attr("id", "info")) {
        let romaji = node.find(Name("h1")).next().unwrap().text();
        println!("romaji title: {}", romaji);
        match node.find(Name("h2")).next() {
            Some(ok) => {
                let original = ok.text();
                println!("original title: {}", original);
            },
            None => break
        }
    }

    // get id
    let id = document.find(Name("h3")).next().unwrap().text();

    // print id
    println!("id: {}", id);

    // get tags
    for node in document.find(Attr("name", "twitter:description")) {
        let tags = node.attr("content").unwrap();
        println!("tags: {}", textwrap::fill(tags, 80));
    }

    // get page count
    let mut pages = String::new();
    for node in document.find(Attr("id", "tags")) {
        for a in node.find(Name("a")) {
            if a.attr("href").unwrap().contains("pages") {
                pages = a.first_child().unwrap().text();
                println!("pages: {}", pages);
            }
        }
    }

    // parse gallery id
    let mut gallery_link = String::new();
    for node in document.find(Name("head")) {
        for meta in node.find(Attr("property", "og:image")) {
            gallery_link = meta.attr("content").unwrap().to_string();
        }
    }
    let id_re = Regex::new(r"[0-9]+").unwrap();
    let id_caps = id_re.captures(&gallery_link).unwrap();
    let gallery_id = id_caps.get(0).map_or("", |m| m.as_str());

    // download
    let dir = format!("./{}", id);
    if !Path::new(&dir).exists() {
        fs::create_dir(dir)?;
    }
    // initialize download links vector
    let mut paths = vec![];
    for i in 1..=pages.parse::<u8>().unwrap() {
        paths.push(format!("https://i.nhentai.net/galleries/{}/{}.jpg", gallery_id, i));
    }
    downloader::downloader(paths, id, client).await;

    Ok(())
}
