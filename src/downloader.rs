use std::sync::Arc;
use regex::Regex;
use futures::future::join_all;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use reqwest::Client;

pub async fn downloader(paths: Vec<String>, id: String, client: Client) {
    let sem = Arc::new(Semaphore::new(10));
    let mut tasks: Vec<JoinHandle<Result<(), ()>>> = vec![];
    for path in paths {
        let path = path.clone();
        let id = id.clone();
        let send_fut = client.get(&path).send();
        let permit = Arc::clone(&sem).acquire_owned().await;

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            match send_fut.await {
                Ok(resp) => match resp.bytes().await {
                    Ok(stream) => match image::load_from_memory(&stream) {
                        Ok(img) => {
                            let page_re = Regex::new(r"(\w+\.)+\w+$").unwrap();
                            let page_caps = page_re.captures(&path).unwrap();
                            let file_name = page_caps.get(0).map_or("", |m| m.as_str());
                            img.save(format!("{}/{}", id, file_name)).unwrap();
                        },
                        Err(e) => println!("[error] cannot write file: {:?}", e)
                    },             
                    Err(e) => println!("[error] cannot get file stream: {:?}", e)
                },
                Err(e) => println!("[error] failed to download file: {:?}", e)
            }
            Ok(())
        }));
    }
    join_all(tasks).await;
}
