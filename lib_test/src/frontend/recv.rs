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
