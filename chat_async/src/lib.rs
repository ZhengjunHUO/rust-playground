pub mod utils;

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
