pub mod vectordb {
    use crate::resource::docker::docker::new_docker;
    use anyhow::Error;
    use docker_api::opts::PullOpts;
    use docker_api::Docker;
    use futures::StreamExt;
    #[tokio::main]
    pub async fn add_vectordb(_name: &Option<String>) -> Result<(), Error> {
        let docker = new_docker()?;
        pull_image_if_not_exists("qdrant/qdrant", &docker).await;
        Ok(())
    }

    pub async fn pull_image_if_not_exists(image_name: &str, docker: &Docker) {
        let images = docker.images();
        let opts = PullOpts::builder().image(image_name).build();
        let mut stream = images.pull(&opts);
        while let Some(pull_result) = stream.next().await {
            match pull_result {
                Ok(output) => {
                    println!("{output:?}");
                }
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}
