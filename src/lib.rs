//! # Markdown to HTML Converter
//! Markdown to HTML Converter is a free tool for converting a Markdown file to a single HTML file with built-in CSS and JS.

extern crate clap;
extern crate html_minifier;
extern crate comrak;

#[macro_use]
extern crate lazy_static_include;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::{Path, PathBuf};
use std::fs;

use clap::{App, Arg};

use comrak::{markdown_to_html, ComrakOptions};

use html_minifier::HTMLMinifier;

lazy_static_include_str!(MarkdownCSS, "resources/github-markdown.css");
lazy_static_include_str!(FontCJK, "resources/font-cjk.css");
lazy_static_include_str!(FontCJKMono, "resources/font-cjk-mono.css");

lazy_static_include_str!(Webfont, "resources/webfont.js");
lazy_static_include_str!(JQuerySlim, "resources/jquery-slim.min.js");
lazy_static_include_str!(WebfontLoader, "resources/webfontloader.min.js");

// TODO -----Config START-----

const APP_NAME: &str = "Markdown to HTML Converter";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");


#[derive(Debug)]
pub struct Config {
    pub markdown_path: String,
    pub html_path: Option<String>,
    pub title: Option<String>,
    pub no_safe: bool,
    pub no_highlight: bool,
    pub no_mathjax: bool,
    pub no_cjk_fonts: bool,
}

impl Config {
    pub fn from_cli() -> Result<Config, String> {
        let arg0 = env::args().next().unwrap();
        let arg0 = Path::new(&arg0).file_stem().unwrap().to_str().unwrap();

        let examples = vec![
            "/path/to/file.md                          # Convert /path/to/file.md to /path/to/file.html, titled \"file\"",
            "/path/to/file.md -o /path/to/output.html  # Convert /path/to/file.md to /path/to/output.html, titled \"output\"",
            "/path/to/file.md -t \"Hello World!\"        # Convert /path/to/file.md to /path/to/file.html, titled \"Hello World!\"",
        ];

        let matches = App::new(APP_NAME)
            .version(CARGO_PKG_VERSION)
            .author(CARGO_PKG_AUTHORS)
            .about(format!("Markdown to HTML Converter is a free tool for converting a Markdown file to a single HTML file with built-in CSS and JS.\n\nEXAMPLES:\n{}", examples.iter()
                .map(|e| format!("  {} {}\n", arg0, e))
                .collect::<Vec<String>>()
                .concat()
            ).as_str()
            )
            .arg(Arg::with_name("TITLE")
                .required(false)
                .long("title")
                .short("t")
                .help("Specifies the title of your HTML file.")
                .takes_value(true)
            )
            .arg(Arg::with_name("MARKDOWN_PATH")
                .required(true)
                .help("Specifies the path of your Markdown file.")
                .takes_value(true)
            )
            .arg(Arg::with_name("HTML_PATH")
                .required(false)
                .long("html-path")
                .short("o")
                .help("Specifies the path of your HTML file.")
                .takes_value(true)
            )
            .arg(Arg::with_name("NO_SAFE")
                .required(false)
                .long("no-safe")
                .help("Allows raw HTML and dangerous URLs.")
            )
            .arg(Arg::with_name("NO_HIGHLIGHT")
                .required(false)
                .long("no-highlight")
                .help("Not allow to use highlight.js.")
            )
            .arg(Arg::with_name("NO_MATHJAX")
                .required(false)
                .long("no-mathjax")
                .help("Not allow to use mathjax.js.")
            )
            .arg(Arg::with_name("NO_CJK_FONTS")
                .required(false)
                .long("no-cjk-fonts")
                .help("Not allow to use CJK fonts.")
            )
            .after_help("Enjoy it! https://magiclen.org")
            .get_matches();

        let title = matches.value_of("TITLE").map(|s| s.to_string());

        let markdown_path = matches.value_of("MARKDOWN_PATH").unwrap().to_string();

        let html_path = matches.value_of("HTML_PATH").map(|s| s.to_string());

        let no_safe = matches.is_present("NO_SAFE");

        let no_highlight = matches.is_present("NO_HIGHLIGHT");

        let no_mathjax = matches.is_present("NO_MATHJAX");

        let no_cjk_fonts = matches.is_present("NO_CJK_FONTS");

        Ok(Config {
            title,
            markdown_path,
            html_path,
            no_safe,
            no_highlight,
            no_mathjax,
            no_cjk_fonts,
        })
    }
}

// TODO -----Config END-----

pub fn run(config: Config) -> Result<i32, String> {
    let (markdown_path, html_path, title) = {
        let markdown_path = Path::new(&config.markdown_path);

        if !markdown_path.exists() {
            return Err(format!("`{}` does not exist.", config.markdown_path));
        }

        if !markdown_path.is_file() {
            return Err(format!("`{}` is not a file.", config.markdown_path));
        }

        let file_name = markdown_path.file_name().unwrap().to_str().unwrap();
        let file_name_len = file_name.len();


        let file_stem = markdown_path.file_stem().unwrap().to_str().unwrap();
        let file_stem_len = file_stem.len();

        let file_ext = if file_name_len > file_stem_len {
            &file_name[(file_stem_len + 1)..]
        } else {
            ""
        };

        if file_ext.ne("md") && file_ext.ne("markdown") {
            return Err(format!("`{}` is not a Markdown file.", config.markdown_path));
        }

        let html_path = match &config.html_path {
            Some(html_path) => {
                let html_path = PathBuf::from(html_path);

                html_path
            }
            None => {
                let folder_path = markdown_path.parent().unwrap();

                let html_path = Path::join(folder_path, format!("{}.html", file_stem));

                html_path
            }
        };

        if html_path.exists() {
            if !html_path.is_file() {
                return Err(format!("`{}` exists and it is not a file.", html_path.to_str().unwrap()));
            }
        }

        let title = match &config.title {
            Some(title) => title,
            None => file_stem
        };

        (markdown_path, html_path, title)
    };

    let markdown = fs::read_to_string(markdown_path).map_err(|err| err.to_string())?;

    let markdown_html = {
        let mut options = ComrakOptions::default();

        if !config.no_safe {
            options.safe = true;
        }

        markdown_to_html(&markdown, &options)
    };

    let mut minifier = HTMLMinifier::new();

    minifier.digest("<!DOCTYPE html>").map_err(|err| err.to_string())?;
    minifier.digest("<html>").map_err(|err| err.to_string())?;

    minifier.digest("<head>").map_err(|err| err.to_string())?;
    minifier.digest("<meta charset=UTF-8>").map_err(|err| err.to_string())?;
    minifier.digest("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\">").map_err(|err| err.to_string())?;

    minifier.digest("<title>").map_err(|err| err.to_string())?;
    minifier.digest(title).map_err(|err| err.to_string())?;
    minifier.digest("</title>").map_err(|err| err.to_string())?;

    minifier.digest("<style>").map_err(|err| err.to_string())?;
    minifier.digest(&MarkdownCSS).map_err(|err| err.to_string())?;
    minifier.digest("</style>").map_err(|err| err.to_string())?;

    if !config.no_cjk_fonts {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&JQuerySlim).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;
    }

    if !config.no_cjk_fonts {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&WebfontLoader).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;

        minifier.digest("<style>").map_err(|err| err.to_string())?;
        minifier.digest(&FontCJK).map_err(|err| err.to_string())?;
        minifier.digest("</style>").map_err(|err| err.to_string())?;

        minifier.digest("<style>").map_err(|err| err.to_string())?;
        minifier.digest(&FontCJKMono).map_err(|err| err.to_string())?;
        minifier.digest("</style>").map_err(|err| err.to_string())?;
    }

    minifier.digest("</head>").map_err(|err| err.to_string())?;

    minifier.digest("<body>").map_err(|err| err.to_string())?;

    minifier.digest("<article class=\"markdown-body\">").map_err(|err| err.to_string())?;
    minifier.digest(&markdown_html).map_err(|err| err.to_string())?;
    minifier.digest("</article>").map_err(|err| err.to_string())?;

    if !config.no_cjk_fonts {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&Webfont).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;
    }

    minifier.digest("</body>").map_err(|err| err.to_string())?;

    minifier.digest("</html>").map_err(|err| err.to_string())?;

    let minified_html = minifier.get_html();

    fs::write(html_path, minified_html).map_err(|err| err.to_string())?;

    Ok(0)
}