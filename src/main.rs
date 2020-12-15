use anyhow::Result;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use std::env;

fn main() {
    let username = env::var("DOCKERHUB_USERNAME").unwrap_or_default();
    let password = env::var("DOCKERHUB_PASSWORD").unwrap_or_default();

    let docker_client = DockerHub::new(username, password);

    let token = docker_client.get_token();

    match docker_client.get_docker_limits(token) {
        Ok((limit, remaining)) => println!("Limit: {:?}, Remaining: {:?}", limit, remaining),
        Err(e) => eprintln!("{}", e),
    };
}

#[allow(dead_code)]
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

    fn get_token(&self) -> Token {
        let token_url = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:ratelimitpreview/test:pull";
        if self.username != "" && self.password != "" {
            println!("Taking Authenticated Token");
            let t_r: Token = reqwest::blocking::Client::new()
                .get(token_url)
                .basic_auth(&self.username, Some(&self.password))
                .send()
                .unwrap()
                .json()
                .unwrap();
            t_r
        } else {
            println!("Taking Anonymous Token");
            let token_response: Token = reqwest::blocking::get(token_url).unwrap().json().unwrap();
            token_response
        }
    }

    fn get_docker_limits(&self, token: Token) -> Result<(String, String), anyhow::Error> {
        // let token_url = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:ratelimitpreview/test:pull";
        // let token_response: Token = reqwest::blocking::get(token_url).unwrap().json().unwrap();
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

        // match response_code {
        //     StatusCode::OK => {
        //         let limit = resp.headers().get("ratelimit-limit").ok_or_else(|| "");
        //         let remaining =  resp.headers().get("ratelimit-remaining").ok_or_else(|| "");

        //         return Ok(limit,remaining)
        //         // if let Some(rate_limit_limit) = resp.headers().get("ratelimit-limit") {
        //         //     let limit: Vec<&str> = rate_limit_limit.to_str().unwrap().split(";").collect();
        //         //    limit[0]
        //         // }
        //     }
        //    s => resp.error_for_status_ref()
        // }
    }
}
