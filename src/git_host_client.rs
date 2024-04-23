use std::env;
use reqwest::Method;
use reqwest;
use core::fmt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};


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
            _ => {panic!("USER_AGENT, GIT_HOST_URL and GIT_HOST_TOKEN must be defined in environment.");}
        }
        
    }

    pub async fn request<T>(&self, method: Method, endpoint: &str, _data: Option<String>) -> reqwest::Result<T>
    where T: DeserializeOwned + Serialize + fmt::Debug
    {
        let client = reqwest::Client::new();
        let request_url = format!("{}{}", &self.url, endpoint);
        let token = format!("{}{}", "Bearer ", &self.token);
        let user = match self.user.as_ref() {
            None => "none",
            Some(user) => user
        };

        let body = match method {
            Method::GET => {
                client
                    .get(request_url)
                    .header("User-Agent", user)
                    .header::<&str, &str>("Authorization", &token)
                    .send().await
            },
            _ => { todo!() }
        };

        let json: reqwest::Result<T> = match body {
            Ok(res) => {
                res.json().await
            },
            Err(err) => { Err(err) }
        };

        match json {
            Ok(value) => {
                Ok(value)
            },
            Err(err) => {
                println!("Error decoding JSON: {:?}", err);
                Err(err)
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubRepoResponse {
    pub full_name: String,
    pub ssh_url: String
}