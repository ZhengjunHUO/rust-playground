use containers_api::conn::tty::TtyChunk;
use docker_api::exec::Exec;
use docker_api::opts::{ContainerFilter, ContainerListOpts, ExecCreateOpts, ExecStartOpts};
use docker_api::Docker;
use futures_util::StreamExt;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let docker = Docker::new("unix:///var/run/docker.sock").unwrap();

    rt.block_on(async {
        /*
                // Print all docker images
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
        */
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
                    let ctn_id = ctn.id.unwrap();
                    println!("Container ID for alpine: {:?}", &ctn_id);
                    let d = Docker::new("unix:///var/run/docker.sock").unwrap();
                    match Exec::create(
                        d,
                        &ctn_id,
                        &ExecCreateOpts::builder()
                            .command(["cat", "/sys/class/net/eth0/iflink"])
                            .attach_stderr(true)
                            .attach_stdout(true)
                            .tty(true)
                            .build(),
                    )
                    .await
                    {
                        Ok(ex) => match ex.start(&ExecStartOpts::builder().build()).await {
                            Ok(mut mx) => {
                                while let Some(chunk) = mx.next().await {
                                    match chunk {
                                        Ok(content) => match content {
                                            TtyChunk::StdOut(v) => println!(
                                                "[stdout]: {}",
                                                String::from_utf8_lossy(&v)
                                            ),
                                            TtyChunk::StdErr(v) => println!(
                                                "[stderr]: {}",
                                                String::from_utf8_lossy(&v)
                                            ),
                                            _ => unreachable!(),
                                        },
                                        Err(e) => {
                                            eprintln!("Error polling from multiplexer: {}", e)
                                        }
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error exec inside container: {}", e),
                        },
                        Err(e) => eprintln!("Error create exec for container: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error retrieving containers: {}", e),
        }
    });
}
