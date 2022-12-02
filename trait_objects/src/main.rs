use trait_objects::{Project, K8s, Baremetal, Service};

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


    // state pattern
    let mut service = Service::new();
    service.add_entrypoint("find ./ -name *rs");
    assert_eq!("", service.entrypoint());

    service.provision_container();
    assert_eq!("", service.entrypoint());

    service.exec();
    assert_eq!("find ./ -name *rs", service.entrypoint());
}
