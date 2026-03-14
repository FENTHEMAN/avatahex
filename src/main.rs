mod artist;
mod utils;

use clap::{Parser, ValueEnum};
use svg::save;

use crate::utils::{convert, generate_svg, parse, queue_parse, rayon_parse};

#[derive(ValueEnum, Clone, Copy)]
enum Optimisation {
    None,
    Rayon,
    Queue,
}

#[derive(Parser)]
struct App {
    input: String,

    save_to: Option<String>,

    #[arg(long, short, default_value = "none")]
    optimisation: Optimisation,
}

fn main() {
    let app = App::parse();

    let save_to = app.save_to.unwrap_or(format!("{}.svg", app.input));

    let operations = match app.optimisation {
        Optimisation::None => parse(&app.input),
        Optimisation::Queue => queue_parse(&app.input),
        Optimisation::Rayon => rayon_parse(&app.input)
    };
    let commands = convert(&operations);
    let document = generate_svg(commands);
    save(save_to, &document).unwrap();
}
