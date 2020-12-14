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

        let token = ureq::get(token_url).call().into_json();

        println!("{:?}", token.unwrap()["token"]);

        // let path = format!("{}", "https://registry-1.docker.io/v2/".to_owned() + "ratelimitpreview/test" + "/manifests/latest");
        // // let headers_registry = 
        // let registry = ureq::head(path.as_str()).set("Authorization", "Bearer ")


        return ("".to_string(), "".to_string(), "".to_string());
    }

    // fn get_username(&self) -> (String, String) {
    //     (self.username.clone(),self.password.clone())
    // }
}
