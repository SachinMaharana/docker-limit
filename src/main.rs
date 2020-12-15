extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate config;
#[macro_use]
extern crate lazy_static;
use std::sync::RwLock;

use anyhow::Result;
use config::*;
// use log::{error, info};
use reqwest::header::HeaderValue;
use serde::Deserialize;
use std::env;

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings
            .merge(File::with_name("Config.toml"))
            .expect("Config file missing");
        settings
    });
}

fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let username = env::var("DOCKERHUB_USERNAME").unwrap_or_default();
    let password = env::var("DOCKERHUB_PASSWORD").unwrap_or_default();

    let docker_client = DockerHub::new(username, password);

    let token = match docker_client.get_token() {
        Ok(t) => t,
        Err(e) => {
            if let Some(err) = e.downcast_ref::<reqwest::Error>() {
                error!("Request Error: {}", err);
            }
            if let Some(err) = e.downcast_ref::<config::ConfigError>() {
                error!("Config Error: {}", err);
            }
            std::process::exit(1);
        }
    };

    match docker_client.get_docker_limits(token) {
        Ok((limit, remaining)) => info!("Limit: {:?}, Remaining: {:?}", limit, remaining),
        Err(e) => {
            if let Some(err) = e.downcast_ref::<reqwest::Error>() {
                error!("Request Error: {}", err);
            }
            if let Some(err) = e.downcast_ref::<config::ConfigError>() {
                error!("Config Error: {}", err);
            }
            std::process::exit(1);
        }
    };
}

struct DockerHub {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct Token {
    token: String,
}

impl DockerHub {
    fn new(username: String, password: String) -> DockerHub {
        let username = username.clone();
        let password = password.clone();
        DockerHub { username, password }
    }

    fn get_token(&self) -> Result<Token, Box<dyn std::error::Error>> {
        let token_url = CONFIG.read()?.get::<String>("token_url")?.clone();

        if self.username != "" && self.password != "" {
            info!("Using Authenticated Token");
            let t_r: Token = reqwest::blocking::Client::new()
                .get(&token_url)
                .basic_auth(&self.username, Some(&self.password))
                .send()?
                .json()?;
            return Ok(t_r);
        }

        info!("Using Anonymous Token");
        let token_response: Token = reqwest::blocking::get(&token_url)?.json()?;
        Ok(token_response)
    }

    fn get_docker_limits(
        &self,
        token: Token,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let registry_url = CONFIG.read()?.get::<String>("registry_url")?.clone();

        let response = reqwest::blocking::Client::new()
            .head(&registry_url)
            .bearer_auth(token.token)
            .send()?;

        let lm: String = response
            .headers()
            .get("ratelimit-limit")
            .map(|x| x.to_str())
            .unwrap_or(Ok(""))?
            .into();

        let rm = response
            .headers()
            .get("ratelimit-remaining")
            .unwrap_or(&HeaderValue::from_static(""))
            .to_str()?
            .into();

        Ok((lm, rm))
    }
}
