use std::error::Error;
use select::document::Document;
use select::predicate::{Attr, Name, Class};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get url from user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("[error] unable to read user input");

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
    for node in document.find(Attr("id", "tags")) {
        for a in node.find(Name("a")) {
            if a.attr("href").unwrap().contains("pages") {
                let pages = a.first_child().unwrap().text();
                println!("pages: {}", pages);
            }
        }
    }

    Ok(())
}
