use std::fs::File;
use std::io::Write;
use trait_objects::fqs::{Cat, Feline, Mao, Neko, Tiger};
use trait_objects::newtype::Envelope;
use trait_objects::operator::{Binome, Offset};
use trait_objects::runner::{Baremetal, K8s, Project};
use trait_objects::state::Service;
use trait_objects::supertrait::{Couple, WrappedPrint};

// trait object, dynmaic dispatch
fn say_sth_to(to: &mut dyn Write, sth: &str) -> std::io::Result<()> {
    to.write_all(sth.as_bytes())?;
    to.flush()
}

// generic, monomorphization
fn say_sth_to_gen<W: Write>(to: &mut W, sth: &str) -> std::io::Result<()> {
    to.write_all(sth.as_bytes())?;
    to.flush()
}

fn main() {
    // test #1
    let sth = "Rust rocks!";
    let mut f = File::create("test").expect("Create file failed: ");
    say_sth_to(&mut f, sth).expect("Write to file failed: ");
    let mut buf = vec![];
    say_sth_to_gen(&mut buf, sth).expect("Write to vec failed: ");
    assert_eq!(buf, sth.as_bytes().to_vec());

    // test #2
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
    assert_eq!(
        Binome { a: 3, b: 4 } * Binome { a: 8, b: 6 },
        Binome { a: 24, b: 24 }
    );
    assert_eq!(Binome { a: 4, b: 9 } + Offset(1), Binome { a: 5, b: 10 });

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

    // wraped type
    let e = Envelope(vec![String::from("Rust"), String::from("Rocks")]);
    println!("wrapped type e: {}", e);
}
