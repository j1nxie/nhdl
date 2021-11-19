use std::error::Error;
use std::io::copy;
use std::fs;
use std::path::Path;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get url from user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("[error] unable to read user input");

    // get html source from url
    let response = reqwest::get(&input).await?;
    let body = response.text().await?;

    // parse body
    let parsed_body = Html::parse_document(&body);

    // set selectors for parsing
    let html_selector = Selector::parse("html").unwrap();
    let head_selector = Selector::parse("head").unwrap();
    let body_selector = Selector::parse("body").unwrap();
    let content_selector = Selector::parse(r#"div[id="content"]"#).unwrap();
    let meta_title = Selector::parse(r#"meta[itemprop="name"]"#).unwrap();
    let meta_tags = Selector::parse(r#"meta[name="twitter:description"#).unwrap();
    // parse for title and tags
    let html = parsed_body.select(&html_selector).next().unwrap();
    let head = html.select(&head_selector).next().unwrap();
    let parsed_title = head.select(&meta_title).next().unwrap();
    let title = parsed_title.value().attr("content").unwrap();
    let parsed_tags = head.select(&meta_tags).next().unwrap();
    let tags = parsed_tags.value().attr("content").unwrap();
    // get id from url
    let id: String = input.chars().filter(|c| c.is_numeric()).collect();

    // download pages
    // initial counters
    let mut page = 1;
    let mut download_response = reqwest::get(format!("https://i.nhentai.net/galleries/{}/{}.jpg", id, page)).await?;
    let dir = format!("./{}/{}.jpg", id, page).to_string();
    while download_response.status() != 404 {
        let content = response.text().await?;
        copy(&mut content.as_bytes(), Path::new(&dir));
        page += 1;
    }

    // print status
    println!("title: {}", title);
    println!("id: {}", id);
    println!("tags: {}", textwrap::fill(tags, 80));
    Ok(())
}
