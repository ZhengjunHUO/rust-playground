pub mod utils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ProtoClient {
    Reg { room: String },
    Envoy { room: String, content: String },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ProtoServer {
    Envoy { room: String, content: String },
    Error(String),
}

#[test]
fn test_proto_client() {
    let msg = ProtoClient::Envoy {
        room: "Programmer".to_string(),
        content: "Who is rustacean here?".to_string(),
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
