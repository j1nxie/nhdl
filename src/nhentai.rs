/*
 * nhentai library for nhdl
 */
use select::predicate::{Attr, Name};
use select::document::Document;
use regex::Regex;

#[derive(Clone, Default)]
pub struct NH {
    pub romaji: String,
    pub original: String,
    pub id: String,
    pub tags: String,
    pub pages: String,
    pub gallery_id: String,
}

impl NH {
    pub fn get_title(&mut self, document: Document) {
        for node in document.find(Attr("id", "info")) {
            self.romaji = node.find(Name("h1")).next().unwrap().text();
            match node.find(Name("h2")).next() {
                Some(ok) => self.original = ok.text(),
                None => break
            }
        }
    }

    pub fn get_id(&mut self, document: Document) {
        self.id = document.find(Name("h3")).next().unwrap().text();
    }

    pub fn get_page(&mut self, document: Document) {
        for node in document.find(Attr("id", "tags")) {
            for a in node.find(Name("a")) {
                if a.attr("href").unwrap().contains("pages") {
                    self.pages = a.first_child().unwrap().text();
                }
            }
        }
    }

    pub fn get_gallery(&mut self, document: Document) {
        let mut gallery_link = String::new();
        for node in document.find(Name("head")) {
            for meta in node.find(Attr("property", "og:image")) {
                gallery_link = meta.attr("content").unwrap().to_string();
            }
        }
        let id_re = Regex::new(r"[0-9]+").unwrap();
        let id_caps = id_re.captures(&gallery_link).unwrap();
        self.gallery_id = id_caps.get(0).map_or("", |m| m.as_str()).to_string();
    }

    pub fn print_status(&self) {
        println!("romaji title: {}", self.romaji);
        if !self.original.is_empty() {
            println!("original title: {}", self.original);
        }
        println!("id: {}", self.id);
        println!("tags: {}", textwrap::fill(&self.tags, 80));
        println!("pages: {}", self.pages);
    }
}
