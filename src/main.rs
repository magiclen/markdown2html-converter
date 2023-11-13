mod cli;

use std::{borrow::Cow, fs, io};

use anyhow::{anyhow, Context};
use cli::*;
use comrak::{markdown_to_html, ComrakOptions};
use html_minifier::HTMLMinifier;
use lazy_static_include::lazy_static_include_str;

lazy_static_include_str! {
    MARKDOWN_CSS => "resources/github-markdown.css",
    FONT_CJK => "resources/font-cjk.css",
    FONT_CJK_MONO => "resources/font-cjk-mono.css",
    GITHUB => "resources/github.css",
    WEBFONT => "resources/webfont.js",
    HIGHLIGHT_CODE => "resources/highlight-code.js",
    MATH_JAX => "resources/mathjax.min.js",
    MATH_JAX_CONFIG => "resources/mathjax-config.js",
    HIGHLIGHT => "resources/highlight.min.js",
}

fn main() -> anyhow::Result<()> {
    let args = get_args();

    if args.markdown_path.metadata().with_context(|| anyhow!("{:?}", args.markdown_path))?.is_dir()
    {
        return Err(anyhow!("{:?} is a directory!", args.markdown_path));
    }

    let file_ext = args
        .markdown_path
        .extension()
        .map(|ext| ext.to_string_lossy())
        .unwrap_or_else(|| "".into());

    match file_ext.to_ascii_lowercase().as_str() {
        "md" | "markdown" => (),
        _ => {
            return Err(anyhow!("{:?} is not a Markdown file.", args.markdown_path));
        },
    }

    let file_stem = args
        .markdown_path
        .file_stem()
        .map(|ext| ext.to_string_lossy())
        .unwrap_or_else(|| "".into());

    let html_path = match args.html_path {
        Some(html_path) => Cow::from(html_path),
        None => {
            let folder_path = args.markdown_path.parent().unwrap();

            Cow::from(folder_path.join(format!("{file_stem}.html")))
        },
    };

    match html_path.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() || !args.force {
                return Err(anyhow!("`{html_path:?}` exists!"));
            }
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => (),
        Err(error) => return Err(error).with_context(|| anyhow!("{html_path:?}")),
    }

    let title = match args.title {
        Some(title) => Cow::from(title),
        None => file_stem,
    };

    let markdown = fs::read_to_string(args.markdown_path.as_path())
        .with_context(|| anyhow!("{:?}", args.markdown_path))?;

    let markdown_html = {
        let mut options = ComrakOptions::default();

        if args.no_safe {
            options.render.unsafe_ = true;
        }

        options.extension.autolink = true;
        options.extension.description_lists = true;
        options.extension.footnotes = true;
        options.extension.strikethrough = true;
        options.extension.superscript = true;
        options.extension.table = true;
        options.extension.tagfilter = true;
        options.extension.tasklist = true;
        options.render.hardbreaks = true;

        markdown_to_html(&markdown, &options)
    };

    let mut html_minifier = HTMLMinifier::new();

    html_minifier.digest("<!DOCTYPE html>")?;
    html_minifier.digest("<html>").unwrap();

    html_minifier.digest("<head>").unwrap();
    html_minifier.digest("<meta charset=UTF-8>").unwrap();
    html_minifier
        .digest(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, \
             shrink-to-fit=no\">",
        )
        .unwrap();
    html_minifier
        .digest(format!(
            "<meta name=\"generator\" content=\"{} {} by magiclen.org\"/>",
            APP_NAME, CARGO_PKG_VERSION,
        ))
        .unwrap();
    html_minifier.digest("<title>").unwrap();
    html_minifier.digest(html_escape::encode_text(title.as_ref()).as_ref()).unwrap();
    html_minifier.digest("</title>").unwrap();

    html_minifier.digest("<style>").unwrap();
    match args.css_path {
        Some(with_css_path) => {
            let with_css = fs::read_to_string(with_css_path.as_path())
                .with_context(|| anyhow!("{with_css_path:?}"))?;

            html_minifier
                .digest(html_escape::encode_style(&with_css).as_ref())
                .with_context(|| anyhow!("{with_css_path:?}"))?;
        },
        None => {
            html_minifier.digest(*MARKDOWN_CSS).unwrap();
        },
    }
    html_minifier.digest("</style>").unwrap();

    let has_code = {
        if args.no_highlight {
            false
        } else {
            markdown_html.contains("</code></pre>")
        }
    };

    let has_mathjax = {
        if args.no_mathjax {
            false
        } else {
            markdown_html.contains("#{{")
        }
    };

    if !args.no_cjk_fonts {
        html_minifier.digest("<style>").unwrap();
        html_minifier.digest(*FONT_CJK).unwrap();
        html_minifier.digest("</style>").unwrap();

        html_minifier.digest("<style>").unwrap();
        html_minifier.digest(*FONT_CJK_MONO).unwrap();
        html_minifier.digest("</style>").unwrap();
    }

    if has_code {
        html_minifier.digest("<script>").unwrap();
        match args.highlight_js_path {
            Some(with_highlight_js_path) => {
                let with_highlight_js = fs::read_to_string(with_highlight_js_path.as_path())
                    .with_context(|| anyhow!("{with_highlight_js_path:?}"))?;

                html_minifier
                    .digest(html_escape::encode_script(&with_highlight_js).as_ref())
                    .with_context(|| anyhow!("{with_highlight_js_path:?}"))?;
            },
            None => unsafe {
                html_minifier.indigest(*HIGHLIGHT);
            },
        }
        html_minifier.digest("</script>").unwrap();

        html_minifier.digest("<style>").unwrap();
        match args.highlight_css_path {
            Some(with_highlight_css_path) => {
                let with_highlight_css = fs::read_to_string(with_highlight_css_path.as_path())
                    .with_context(|| anyhow!("{with_highlight_css_path:?}"))?;

                html_minifier
                    .digest(html_escape::encode_style(&with_highlight_css).as_ref())
                    .with_context(|| anyhow!("{with_highlight_css_path:?}"))?;
            },
            None => {
                html_minifier.digest(*GITHUB).unwrap();
            },
        }
        html_minifier.digest("</style>").unwrap();
    }

    if has_mathjax {
        html_minifier.digest("<script>").unwrap();
        html_minifier.digest(*MATH_JAX_CONFIG).unwrap();
        html_minifier.digest("</script>").unwrap();

        html_minifier.digest("<script>").unwrap();
        match args.mathjax_js_path {
            Some(with_mathjax_js_path) => {
                let with_mathjax_js = fs::read_to_string(with_mathjax_js_path.as_path())
                    .with_context(|| anyhow!("{with_mathjax_js_path:?}"))?;

                html_minifier
                    .digest(html_escape::encode_script(&with_mathjax_js).as_ref())
                    .with_context(|| anyhow!("{with_mathjax_js_path:?}"))?;
            },
            None => unsafe {
                html_minifier.indigest(*MATH_JAX);
            },
        }
        html_minifier.digest("</script>").unwrap();
    }

    html_minifier.digest("</head>").unwrap();

    html_minifier.digest("<body>").unwrap();

    html_minifier.digest("<article class=\"markdown-body\">").unwrap();
    html_minifier.digest(&markdown_html).unwrap();
    html_minifier.digest("</article>").unwrap();

    if !args.no_cjk_fonts {
        html_minifier.digest("<script>").unwrap();
        html_minifier.digest(*WEBFONT).unwrap();
        html_minifier.digest("</script>").unwrap();
    }

    if has_code {
        html_minifier.digest("<script>").unwrap();
        html_minifier.digest(*HIGHLIGHT_CODE).unwrap();
        html_minifier.digest("</script>").unwrap();
    }

    html_minifier.digest("</body>").unwrap();

    html_minifier.digest("</html>").unwrap();

    let minified_html = html_minifier.get_html();

    fs::write(html_path.as_ref(), minified_html).with_context(|| anyhow!("{html_path:?}"))?;

    Ok(())
}
