use std::path::PathBuf;

use clap::{CommandFactory, FromArgMatches, Parser};
use concat_with::concat_line;
use terminal_size::terminal_size;

pub const APP_NAME: &str = "Markdown to HTML Converter";
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const AFTER_HELP: &str = "Enjoy it! https://magiclen.org";

const APP_ABOUT: &str = concat!(
    "A simple tool for converting Simple Chinese to Traditional Chinese(TW).\n\nEXAMPLES:\n",
    concat_line!(prefix "markdown2html-converter ",
            "/path/to/file.md                           # Convert /path/to/file.md to /path/to/file.html, titled \"file\"",
            "/path/to/file.md -o /path/to/output.html   # Convert /path/to/file.md to /path/to/output.html, titled \"output\"",
            "/path/to/file.md -t 'Hello World!'         # Convert /path/to/file.md to /path/to/file.html, titled \"Hello World!\"",
    )
);

#[derive(Debug, Parser)]
#[command(name = APP_NAME)]
#[command(term_width = terminal_size().map(|(width, _)| width.0 as usize).unwrap_or(0))]
#[command(version = CARGO_PKG_VERSION)]
#[command(author = CARGO_PKG_AUTHORS)]
#[command(after_help = AFTER_HELP)]
pub struct CLIArgs {
    #[arg(short, long)]
    #[arg(help = "Specify the title of your HTML file")]
    pub title: Option<String>,

    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Specify the path of your Markdown file")]
    pub markdown_path: PathBuf,

    #[arg(short = 'o', long)]
    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Specify the path of your HTML file")]
    pub html_path: Option<PathBuf>,

    #[arg(short, long)]
    #[arg(help = "Force to output if the HTML file exists")]
    pub force: bool,

    #[arg(long)]
    #[arg(help = "Allow raw HTML and dangerous URLs")]
    pub no_safe: bool,

    #[arg(long)]
    #[arg(help = "Not allow to use highlight.js")]
    pub no_highlight: bool,

    #[arg(long)]
    #[arg(help = "Not allow to use mathjax.js")]
    pub no_mathjax: bool,

    #[arg(long)]
    #[arg(help = "Not allow to use CJK fonts")]
    pub no_cjk_fonts: bool,

    #[arg(long)]
    #[arg(help = "Specify the path of your custom CSS file")]
    pub css_path: Option<PathBuf>,

    #[arg(long)]
    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Specify the path of your custom highlight.js file")]
    pub highlight_js_path: Option<PathBuf>,

    #[arg(long)]
    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Specify the path of your custom CSS file for highlight.js code blocks")]
    pub highlight_css_path: Option<PathBuf>,

    #[arg(long)]
    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Specify the path of your custom single MATH_JAX.js file")]
    pub mathjax_js_path: Option<PathBuf>,
}

pub fn get_args() -> CLIArgs {
    let args = CLIArgs::command();

    let about = format!("{APP_NAME} {CARGO_PKG_VERSION}\n{CARGO_PKG_AUTHORS}\n{APP_ABOUT}");

    let args = args.about(about);

    let matches = args.get_matches();

    match CLIArgs::from_arg_matches(&matches) {
        Ok(args) => args,
        Err(err) => {
            err.exit();
        },
    }
}
