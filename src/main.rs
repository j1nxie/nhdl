use std::error::Error;
use std::path::Path;

use std::fs;

use reqwest::Client;

mod downloader;
mod nhentai;
mod html;
mod init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let list = init::get_input();
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
