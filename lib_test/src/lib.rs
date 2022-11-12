use crate::frontend::recv;

fn decode() {}

mod frontend {
    pub mod recv {
        pub struct Session {
            pub username: String,
            session_id: u64,
        }

        impl Session {
            pub fn authentication(username: &str) -> Session {
                Session {
                    username: String::from(username),
                    session_id: 8,
                }
            }
        }

        fn queueing() {}
        fn authorization() {
            apply_rules();
            super::super::decode();
        }
        fn apply_rules() {}
    }

    mod analyse {
        fn routing() {}
        fn forword() {}
    }

    mod send {
        fn encode() {}
        fn deliver() {}
    }
}

pub fn call_service() {
    //let mut s = crate::frontend::recv::Session::authentication("fufu");
    let mut s = recv::Session::authentication("fufu");
    println!("Login as {} ...", s.username);

    s.username = String::from("huo");
    println!("Not working, try login as {} ...", s.username);
}
