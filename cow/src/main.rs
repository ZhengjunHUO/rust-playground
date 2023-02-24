use std::borrow::Cow;
use std::env::var;

fn from_env(s: &str) -> String {
    var(s).unwrap_or("unknown".to_string())
}

// If environment variable is set, returns the resulting String as a Cow::Owned
// If not exist, returns its &str as a Cow::Borrowed
fn from_env_cow(s: &str) -> Cow<'_, str> {
    var(s)
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("unknown"))
}

fn main() {
    println!("Hello {} from {}!", from_env("USER"), from_env("HOME"));
    println!(
        "Hello again {} from {}!",
        from_env_cow("USER"),
        from_env_cow("HOME")
    );
}
