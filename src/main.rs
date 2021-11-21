use std::error::Error;
use std::path::Path;
use std::fs;
use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get url from user input
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input)
            .expect("[error] unable to read user input");
        let test = &input.trim().parse::<u32>();
        match test {
            Ok(ok) => {
                input = format!("https://nhentai.net/g/{}/", ok);
                break;
            },
            Err(_e) => {
                let valid = &input.trim().contains("nhentai.net");
                match valid {
                    true => break,
                    false => println!("[error] invalid url or id!")
                }
            }
        }
        input = String::new(); // resets input
   }

    // get html source from reqwest
    let response = reqwest::get(&input).await?;
    let body = response.text().await?;
    let document = Document::from(body.as_str());

    // get titles
    let romaji = document.find(Name("h1")).next().unwrap().text();
    let japanese = document.find(Name("h2")).next().unwrap().text();

    // get id
    let id = document.find(Name("h3")).next().unwrap().text();

    // print status
    println!("romaji title: {}", romaji);
    println!("japanese title: {}", japanese);
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
    let re = Regex::new(r"[0-9]+").unwrap();
    let caps = re.captures(&gallery_link).unwrap();
    let gallery_id = caps.get(0).map_or("", |m| m.as_str());

    // download
    let dir = format!("./{}", id);
    if !Path::new(&dir).exists() {
        fs::create_dir(dir)?;
    }
    for i in 1..=pages.parse::<u8>().unwrap() {
        let download_response = reqwest::get(format!("https://i.nhentai.net/galleries/{}/{}.jpg", gallery_id, i)).await
            .expect("[error] request failed");
        let file_stream = download_response.bytes().await?;
        println!("[status] downloading page {}/{}", i, pages);
        let image = image::load_from_memory(&file_stream)?;
        image.save(format!("{}/{}.jpg", id, i)).unwrap();
    }

    Ok(())
}
