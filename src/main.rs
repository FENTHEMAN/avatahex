mod artist;
mod utils;

use clap::Parser;
use svg::save;

use crate::utils::{convert, generate_svg, parse};

#[derive(Parser)]
struct App {
    input: String,
    save_to: Option<String>,
}

fn main() {
    let app = App::parse();

    let save_to = app.save_to.unwrap_or(format!("{}.svg", app.input));

    let operations = parse(&app.input);
    let commands = convert(&operations);
    let document = generate_svg(commands);
    save(save_to, &document).unwrap();
}
