use trait_objects::runner::{Project, K8s, Baremetal};
use trait_objects::state::Service;
use trait_objects::operator::{Binome, Offset};
use trait_objects::fqs::{Feline, Cat, Tiger, Mao, Neko};
use trait_objects::supertrait::{WrappedPrint, Couple};

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

    // trait with associated type
    assert_eq!(Binome {a: 3, b: 4} * Binome {a: 8, b: 6}, Binome {a: 24, b: 24});
    assert_eq!(Binome {a: 4, b: 9} + Offset(1), Binome {a: 5, b: 10});

    // fully qualified syntax
    let f = Feline;
    f.talk();
    Cat::talk(&f);
    Tiger::talk(&f);

    println!("My cat's name: {}", Mao::nickname());
    println!("My cat's name: {}", <Mao as Neko>::nickname());

    // supertrait
    let c = Couple { a: 6, b: 6 };
    c.wrapped_print();
}
