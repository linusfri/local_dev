use reqwest::*;

pub struct GitHostClient {
    url: String,
    token: String
}

impl GitHostClient {
    pub fn new(url: String, token: String) -> Self {
        GitHostClient {url, token}
    }

    pub async fn request(&self, method: &str, data: Option<String>) -> Result<String> {
        let body = reqwest::get(&self.url)
            .await?
            .text()
            .await?;

        Ok(body)
    }
}

pub enum Endpoints {

}