use std::fs::File;
use std::io::prelude::*;

use clap::Parser;
use highlight::pretty_print_html;

mod highlight;

#[derive(clap::Parser)]
#[clap(version, author, about = "Downloads a webpage")]
struct Args {
    /// The URL of the webpage to download
    #[clap(required = true)]
    url: String,

    /// File to save the downloaded webpage to
    #[clap(short, long)]
    output: Option<String>,

    /// Activate syntax highlighting
    #[clap(short, long, default_value_t = false)]
    prettify: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let response = reqwest::blocking::get(&args.url)?;
    let text = response.text()?;

    let text = if args.prettify {
        pretty_print_html(&text).unwrap()
    } else {
        text
    };

    if let Some(output_file) = args.output {
        let mut file = File::create(output_file)?;
        file.write_all(text.as_bytes())?;
    } else {
        println!("{text}");
    };
    Ok(())
}
