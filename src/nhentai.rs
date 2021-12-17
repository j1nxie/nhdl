/*
 * nhentai library for nhdl
 */
use select::predicate::{Attr, Name};
use select::document::Document;
use regex::Regex;

#[derive(Clone)]
pub struct NH {
    pub romaji: String,
    pub original: String,
    pub id: String,
    pub tags: String,
    pub pages: String,
    pub gallery_id: String,
}

impl Default for NH {
    fn default() -> NH {
        NH {
            romaji: String::new(),
            original: String::new(),
            id: String::new(),
            tags: String::new(),
            pages: String::new(),
            gallery_id: String::new(),
        }
    }
}

pub fn metadata(document: Document) -> NH {
    let mut title = NH::default();
    // get titles
    for node in document.find(Attr("id", "info")) {
        title.romaji = node.find(Name("h1")).next().unwrap().text();
        match node.find(Name("h2")).next() {
            Some(ok) => title.original = ok.text(),
            None => break
        }
    }

    // get id
    title.id = document.find(Name("h3")).next().unwrap().text();

    // get tags
    for node in document.find(Attr("name", "twitter:description")) {
        title.tags = node.attr("content").unwrap().to_string();
    }

    // get page count
    for node in document.find(Attr("id", "tags")) {
        for a in node.find(Name("a")) {
            if a.attr("href").unwrap().contains("pages") {
                title.pages = a.first_child().unwrap().text();
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
    title.gallery_id = id_caps.get(0).map_or("", |m| m.as_str()).to_string();

    return title;
}

pub fn print_status(title: NH) {
    println!("romaji title: {}", title.romaji);
    if !title.original.is_empty() {
        println!("original title: {}", title.original);
    }
    println!("id: {}", title.id);
    println!("tags: {}", textwrap::fill(&title.tags, 80));
    println!("pages: {}", title.pages);
}
