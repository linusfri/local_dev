use std::env;
use reqwest::Method;
use reqwest::*;
use serde_json;

pub struct GitHostClient {
    url: String,
    token: String,
    user: Option<String>
}

impl GitHostClient {
    pub fn new() -> Self {
        let user_agent = env::var("USER_AGENT");
        let url = env::var("GIT_HOST_URL");
        let token = env::var("GIT_HOST_TOKEN");

        match (user_agent, url, token) {
            (Ok(user_agent), Ok(url), Ok(token)) => GitHostClient {url, token, user: Some(String::from(user_agent))},
            _ => {panic!(".env problems");}
        }
        
    }

    pub async fn request(&self, method: Method, endpoint: &str, _data: Option<String>) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let mut request_url = self.url.clone();

        request_url.push_str(endpoint);

        let body = match method {
            Method::GET => {
                client
                    .get(request_url)
                    .header("User-Agent", self.user.as_ref().unwrap())
                    .send().await
            },
            _ => { todo!() }
        };

        let json: serde_json::Value = match body {
            Ok(res) => {
                res.json().await.unwrap()
            },
            _ => { serde_json::Value::Null }
        };

        // println!("{:#?}", json[0].get("full_name"));
        Ok(json)
    }
}