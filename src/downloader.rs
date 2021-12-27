/*
 * downloader function for nhdl
 * take a Vec<String> of urls, a String id as folder name and a reqwest Client
 * output downloaded files from given urls asynchronously
 */
use std::sync::Arc;

use regex::Regex;

use futures::future::join_all;

use tokio::sync::Semaphore;
use tokio::task::JoinHandle;

use reqwest::Client;

use tracing::error;

pub async fn downloader(paths: Vec<String>, id: String, client: Client) {
    let sem = Arc::new(Semaphore::new(10)); // limit max amount of threads so downloads don't break
    let mut tasks: Vec<JoinHandle<Result<(), ()>>> = vec![]; // initialize empty vector for list of tasks
    for path in paths {
        // clone path and id to fix ownership problems
        let path = path.clone();
        let id = id.clone();
        let send_fut = client.get(&path).send();
        let permit = Arc::clone(&sem).acquire_owned().await;

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            match send_fut.await { // get html response
                Ok(resp) => match resp.bytes().await { // get bytes as a file stream
                    Ok(stream) => match image::load_from_memory(&stream) { // load bytes into memory
                        Ok(img) => {
                            let page_re = Regex::new(r"(\w+\.)+\w+$").unwrap();
                            let page_caps = page_re.captures(&path).unwrap();
                            let file_name = page_caps.get(0).map_or("", |m| m.as_str());
                            img.save(format!("{}/{}", id, file_name)).unwrap();
                        },
                        Err(e) => error!("cannot write file: {:?}", e)
                    },             
                    Err(e) => error!("cannot get file stream: {:?}", e)
                },
                Err(e) => error!("failed to download file: {:?}", e)
            }
            Ok(())
        }));
    }
    join_all(tasks).await;
}
