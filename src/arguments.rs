#[derive(clap::Parser)]
#[clap(version, author, about = "Downloads a webpage")]
pub struct Arguments {
    /// The URL of the webpage to download
    #[clap(required = true)]
    pub url: String,

    /// File to save the downloaded webpage to
    #[clap(short, long)]
    pub output: Option<String>,

    /// Activate syntax highlighting
    #[clap(short, long, default_value_t = false)]
    pub prettify: bool,
}
