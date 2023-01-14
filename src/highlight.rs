use anyhow::Context;
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

pub fn pretty_print_html(html: &str) -> anyhow::Result<String> {
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
        theme,
    )?)
}

#[allow(unused)]
pub fn get_theme_names() -> Vec<String> {
    let theme_set = ThemeSet::load_defaults();
    theme_set.themes.keys().cloned().collect()
}
