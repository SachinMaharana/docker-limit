extern crate pretty_env_logger;
#[macro_use] extern crate log;

use anyhow::Result;
// use log::{error, info};
use reqwest::header::HeaderValue;
use serde::Deserialize;
use std::env;


fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    let username = env::var("DOCKERHUB_USERNAME").unwrap_or_default();
    let password = env::var("DOCKERHUB_PASSWORD").unwrap_or_default();

    let docker_client = DockerHub::new(username, password);

    let token = match docker_client.get_token() {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    };

    match docker_client.get_docker_limits(token) {
        Ok((limit, remaining)) => info!("Limit: {:?}, Remaining: {:?}", limit, remaining),
        Err(e) => error!("{}", e),
    };
    Ok(())
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

    fn get_token(&self) -> Result<Token> {
        let token_url = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:ratelimitpreview/test:pull";
        if self.username != "" && self.password != "" {
            info!("Using Authenticated Token");
            let t_r: Token = reqwest::blocking::Client::new()
                .get(token_url)
                .basic_auth(&self.username, Some(&self.password))
                .send()?
                .json()?;
            Ok(t_r)
        } else {
            info!("Using Anonymous Token");
            let token_response: Token = reqwest::blocking::get(token_url)?.json()?;
            Ok(token_response)
        }
    }

    fn get_docker_limits(&self, token: Token) -> Result<(String, String), anyhow::Error> {
        let response = reqwest::blocking::Client::new()
            .head("https://registry-1.docker.io/v2/ratelimitpreview/test/manifests/latest")
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

        //  let limit: Vec<&str> = rate_limit_limit.to_str().unwrap().split(";").collect();
    }
}
