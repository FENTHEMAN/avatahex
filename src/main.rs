mod artist;
mod utils;

use clap::Parser;

#[derive(Parser)]
struct App {
    input: String,
    save_to: Option<String>,
}

fn main() {
    let app = App::parse();

    let save_to = app.save_to.unwrap_or(app.input + ".svg");
}
