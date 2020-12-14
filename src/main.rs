use reqwest::StatusCode;
#[allow(unused_variables, unused_assignments)]
use serde::Deserialize;
use std::env;

fn main() {
    let username = env::var("DOCKERHUB_USERNAME").unwrap_or_default();
    let password = env::var("DOCKERHUB_PASSWORD").unwrap_or_default();

    let docker_client = DockerHub::new(username, password);

    let (_limit, _remaining, _reset) = docker_client.get_docker_limits();
}

struct DockerHub {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct Token {
    token: String,
}

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "kebab-case")]
// struct RateLimiting {
//     ratelimit-limit: String,
//     ratelimit-remaining: String,
// }

impl DockerHub {
    fn new(username: String, password: String) -> DockerHub {
        let username = username.clone();
        let password = password.clone();
        DockerHub { username, password }
    }

    fn get_docker_limits(&self) -> (String, String, String) {
        let token_url = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:ratelimitpreview/test:pull";
        let token_response: Token = reqwest::blocking::get(token_url).unwrap().json().unwrap();
        let resp = reqwest::blocking::Client::new()
            .head("https://registry-1.docker.io/v2/ratelimitpreview/test/manifests/latest")
            .bearer_auth(token_response.token)
            .send()
            .unwrap();
        let response_code = resp.status();

        match response_code {
            StatusCode::OK => {
                if let Some(rate_limit_limit) = resp.headers().get("ratelimit-limit") {
                    let limit:Vec<&str> = rate_limit_limit.to_str().unwrap().split(";").collect();
                    println!("RateLimit-Limit: {:?}", limit[0]);
                }
                if let Some(rate_limit_remaining) = resp.headers().get("ratelimit-remaining") {
                    let remaining:Vec<&str> = rate_limit_remaining.to_str().unwrap().split(";").collect();
                    println!("RateLimit-Remaining: {:?}", remaining[0]);
                }
            }
            s => println!("Received response status: {:?}", s),
        }

        return ("".to_string(), "".to_string(), "".to_string());
    }
}
