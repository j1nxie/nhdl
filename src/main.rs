#[macro_use]
extern crate tracing;

#[macro_use]
extern crate lazy_static;

use std::{
    error::Error,
    path::Path,
    fs,
};

use reqwest::Client;

mod settings;
mod init;
mod downloader;
mod nhentai;
mod html;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new()
        .expect("config file not found");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize logger
    tracing_subscriber::fmt::init();
    info!("tracing initialized.");

    // get user input - either an id, url or file
    let list = init::get_input();

    // get html source from reqwest
    let client: Client =
    if CONFIG.proxy.is_empty() {
        Client::builder().build()?
    } else {
        let proxy = reqwest::Proxy::all(&CONFIG.proxy)?
            .basic_auth(&CONFIG.proxy_username, &CONFIG.proxy_password);
        Client::builder()
            .proxy(proxy)
            .build()?
    };

    // cycles through each doujin and download them
    for doujin in list {
        // get html document for parsing
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
        let dir = format!("{}{}", CONFIG.path, title.id);
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
