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
        // let headers_registry = 


        return ("".to_string(), "".to_string(), "".to_string());
    }

    // fn get_username(&self) -> (String, String) {
    //     (self.username.clone(),self.password.clone())
    // }
}
