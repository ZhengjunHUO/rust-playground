use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ProtoClient {
    Reg {
        room: Arc<String>,
    },
    Envoy {
        room: Arc<String>,
        content: Arc<String>,
    },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ProtoServer {
    Envoy {
        room: Arc<String>,
        content: Arc<String>,
    },
    Error(String),
}

#[test]
fn test_proto_client() {
    let msg = ProtoClient::Envoy {
        room: Arc::new("Programmer".to_string()),
        content: Arc::new("Who is rustacean here?".to_string()),
    };

    let marshalled = serde_json::to_string(&msg).unwrap();
    assert_eq!(
        marshalled,
        r#"{"Envoy":{"room":"Programmer","content":"Who is rustacean here?"}}"#
    );
    assert_eq!(
        serde_json::from_str::<ProtoClient>(&marshalled).unwrap(),
        msg
    );
}

#[test]
fn test_race() {
    use async_std::future;
    use async_std::prelude::*;
    use async_std::task::block_on;

    block_on(async {
        let a = future::pending();
        let b = future::ready(1u8);
        let c = future::ready(2u8);

        // Awaits multiple futures simultaneously, returning the output of the first future that completes.
        let f = a.race(c).race(b);
        let rslt = f.await;
        assert_eq!(rslt, 2u8);
    });

    block_on(async {
        let a = future::pending();
        let b = future::ready(1u8);
        let c = future::ready(2u8);

        let f = a.race(b).race(c);
        let rslt = f.await;
        assert_eq!(rslt, 1u8);
    });
}
