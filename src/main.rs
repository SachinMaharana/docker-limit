#![allow(dead_code)]

use std::env;

fn main() {
    let username = env::var("DOCKERHUB_USERNAME").unwrap_or_default();
    let password = env::var("DOCKERHUB_PASSWORD").unwrap_or_default();

    let docker_client = DockerHub::new(username, password);

    let (limit, remaining, reset) = docker_client.get_docker_limits();
}

struct DockerHub {
    username: String,
    password: String,
}

impl DockerHub {
    fn new(username: String, password: String) -> DockerHub {
        let username = username.clone();
        let password = password.clone();

        DockerHub { username, password }
    }

    fn get_docker_limits(&self) -> (String, String, String) {
        let token_url = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:ratelimitpreview/test:pull";

        let token_resp = ureq::get(token_url).call().into_json().unwrap();
        let token = &token_resp["token"].to_string();

        let client = reqwest::blocking::Client::new();

        let resp = client
            .head("https://registry-1.docker.io/v2/ratelimitpreview/test/manifests/latest")
            .bearer_auth(token)
            .send();

        println!("{:?}", resp);

        return ("".to_string(), "".to_string(), "".to_string());
    }

    // fn get_username(&self) -> (String, String) {
    //     (self.username.clone(),self.password.clone())
    // }
}
