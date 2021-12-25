use select::document::Document;

use reqwest::Client;

pub async fn get_document(input: String, client: Client) -> Document {
    let response = client.get(input).send().await;
    let body = response.unwrap().text().await;
    let document = Document::from(body.unwrap().as_str());
    return document;
}
