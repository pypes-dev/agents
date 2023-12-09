pub mod docker {
    use docker_api::{Docker, Result};

    #[cfg(unix)]
    pub fn new_docker() -> Result<Docker> {
        Ok(Docker::unix("/var/run/docker.sock"))
    }

    #[cfg(not(unix))]
    pub fn new_docker() -> Result<Docker> {
        Docker::new("tcp://127.0.0.1:8080")
    }
}
