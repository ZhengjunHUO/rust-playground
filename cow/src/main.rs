use rand::{thread_rng, Rng};
use std::borrow::Cow;
use std::env::var;

fn affinity<'a>() -> Option<&'a str> {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..10);
    if n > 7 {
        return Some("my dear");
    }
    if n > 3 {
        return Some("my friend");
    }
    None
}

fn from_env(s: &str) -> String {
    var(s).unwrap_or("unknown".to_string())
}

// If environment variable is set, returns the resulting String as a Cow::Owned
// If not exist, returns its &str as a Cow::Borrowed
fn from_env_cow(s: &str) -> Cow<'_, str> {
    var(s)
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("unknown"))
}

fn main() {
    println!("Hello {} from {}!", from_env("USER"), from_env("HOME"));
    println!(
        "Hello again {} from {}!",
        from_env_cow("USER"),
        from_env_cow("HOME")
    );

    let mut username = from_env_cow("USER");
    if let Some(aff) = affinity() {
        // if username is borrowed, will keep as borrowed until need to be modified
        username += " ";
        username.to_mut().push_str(aff);
    }
    println!("Welcome {} !", username);
}
