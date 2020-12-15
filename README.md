# docker-rate-limit

This Project checks rate limits and rate remaining for pull requests on Docker Hub.

[DockerHubLimit](https://docs.docker.com/docker-hub/download-rate-limit)

Both anonymous and autheticated account are supported.

# Installation And Usage

1. Download the latest binary from release.
    ```
    wget -q --show-progress https://github.com/SachinMaharana/docker-rate-limit/releases/download/v0.4.0/drl-linux-amd64
    ```

2. Add execute Permission and move it to /usr/local/bin to execute from anywhere.
    ```
    mv ./drl-linux-amd64 drl
    chmod +x ./drl
    ```
3. Execute the binary.
    ```
    ./drl            
    INFO  drl > Using Anonymous Token
    INFO  drl > Limit: "200;w=21600", Remaining: "198;w=21600"
    ```

# DockerHub Auth

In case you want to use your own username and password credentials for hub.docker.com,
you need to export them into the environment.

```
export DOCKERHUB_USERNAME='xxx'
export DOCKERHUB_PASSWORD='xxx'
```

# Example
```
export DOCKERHUB_USERNAME='xxx'
export DOCKERHUB_PASSWORD='xxx'

./drl
INFO  drl > Using Authenticated Token
INFO  drl > Limit: "200;w=21600", Remaining: "198;w=21600"

```


