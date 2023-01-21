#[derive(clap::Parser)]
#[command(version, author, about = "Downloads a webpage")]
pub struct Arguments {
    /// The URL of the webpage to download
    #[arg(required = true)]
    pub url: String,

    /// File to save the downloaded webpage to
    #[arg(short, long)]
    pub output: Option<String>,

    /// Activate syntax highlighting
    #[arg(short, long, default_value_t = false)]
    pub prettify: bool,
}
