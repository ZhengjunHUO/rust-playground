use docker_api::opts::{ContainerFilter, ContainerListOpts};
use docker_api::Docker;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let docker = Docker::new("unix:///var/run/docker.sock").unwrap();

    rt.block_on(async {
        println!("### Image list ###");
        match docker.images().list(&Default::default()).await {
            Ok(images) => {
                for image in images {
                    println!("{0:?}", image.repo_tags);
                }
            }
            Err(e) => eprintln!("Error retrieving images: {}", e),
        }
        println!("### End of image list ###");

        match docker
            .containers()
            .list(
                &ContainerListOpts::builder()
                    .filter(vec![ContainerFilter::Name("alpine".to_string())])
                    .build(),
            )
            .await
        {
            Ok(ctns) => {
                for ctn in ctns {
                    println!("Container ID for alpine: {:?}", ctn.id.unwrap());
                }
            }
            Err(e) => eprintln!("Error retrieving containers: {}", e),
        }
    });
}
