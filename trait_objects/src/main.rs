use trait_objects::{Project, K8s, Baremetal};

fn main() {
    let p = Project {
        envs: vec![
            Box::new(K8s {
                nodes: 16,
                version: String::from("1.22"),
                cni: String::from("cilium"),
            }),
            Box::new(Baremetal {
                os: String::from("Fedora 37"),
                platform: String::from("amd64"),
                release: String::from("6.0.10"),
            }),
        ],
    };

    p.exec();
}
