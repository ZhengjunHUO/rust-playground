use clap::Parser;
use serde_json::json;

#[derive(Parser)]
struct Params {
    #[arg(short = 'j', long = "json")]
    json: bool,
}

fn main() {
    let args = Params::parse();
    if args.json {
        println!(
            "{}",
            json!({
                "type": "info",
                "message": "This is rust speaking",
            })
        );
    } else {
        println!("This is rust speaking");
    }
}
