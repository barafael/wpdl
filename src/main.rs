use std::fs::File;
use std::io::prelude::*;

use arguments::Arguments;
use clap::Parser;
use highlight::pretty_print_html;

mod arguments;
mod highlight;

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();

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
