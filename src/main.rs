use std::fs::File;
use std::io::prelude::*;

use anyhow::Context;
use clap::Parser;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[derive(clap::Parser)]
#[clap(version, author, about = "Downloads a webpage")]
struct Args {
    /// The URL of the webpage to download
    #[clap(required = true)]
    url: String,

    /// File to save the downloaded webpage to
    #[clap(short, long)]
    output: Option<String>,

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

fn pretty_print_html(html: &str) -> anyhow::Result<String> {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let html_syntax = syntax_set
        .find_syntax_by_extension("html")
        .expect("Failed to get syntax");
    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set
        .themes
        .get("base16-ocean.dark")
        .context("Failed to get theme")?;

    Ok(highlighted_html_for_string(
        html,
        &syntax_set,
        html_syntax,
        &theme,
    )?)
}

#[allow(unused)]
fn get_theme_names() -> Vec<String> {
    let theme_set = ThemeSet::load_defaults();
    theme_set
        .themes
        .iter()
        .map(|(name, _theme)| name)
        .cloned()
        .collect()
}
