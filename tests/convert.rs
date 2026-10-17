use markdown2html_converter::{
    ConvertOptions, HighlightLanguages, MathMode, MermaidMode, Theme, convert,
};

const MERMAID_MARKDOWN: &str = "```mermaid\ngraph TD; A-->B;\n```\n";

fn convert_to_string(markdown: &str, options: &ConvertOptions) -> String {
    String::from_utf8(convert(markdown, options).unwrap()).unwrap()
}

fn title_of(html: &str) -> String {
    let start = html.find("<title>").unwrap() + "<title>".len();
    let length = html[start..].find("</title>").unwrap();

    html[start..(start + length)].to_string()
}

#[test]
fn title_comes_from_the_first_heading() {
    let html =
        convert_to_string("# Hello world!\n\n# Another heading\n", &ConvertOptions::default());

    assert_eq!("Hello world!", title_of(html.as_str()));
}

#[test]
fn the_heading_title_includes_math() {
    for (markdown, expected) in [("# About $x$", "About x"), ("# $E=mc^2$", "E=mc^2")] {
        let html = convert_to_string(markdown, &ConvertOptions::default());

        assert_eq!(expected, title_of(html.as_str()));
    }
}

#[test]
fn title_comes_from_the_front_matter() {
    let markdown = "---\ntitle: \"From the front matter\"\n---\n\n# Hello world!\n";

    let html = convert_to_string(markdown, &ConvertOptions::default());

    assert_eq!("From the front matter", title_of(html.as_str()));
    assert!(!html.contains("From the front matter</h"));
}

#[test]
fn a_folded_title_comes_from_the_front_matter() {
    let markdown = "---\ntitle: >-\n  A title with\n  multiple lines\n---\n\n# Hello world!\n";

    let html = convert_to_string(markdown, &ConvertOptions::default());

    assert_eq!("A title with multiple lines", title_of(html.as_str()));
}

#[test]
fn the_given_title_wins() {
    let options = ConvertOptions {
        title: Some("The given title"),
        default_title: Some("The default title"),
        ..ConvertOptions::default()
    };

    let html = convert_to_string("# Hello world!", &options);

    assert_eq!("The given title", title_of(html.as_str()));
}

#[test]
fn title_falls_back_to_the_default_title() {
    let options = ConvertOptions {
        default_title: Some("The default title"),
        ..ConvertOptions::default()
    };

    let html = convert_to_string("Just a paragraph.", &options);

    assert_eq!("The default title", title_of(html.as_str()));
}

fn math_html(math_mode: MathMode) -> String {
    let options = ConvertOptions {
        math: Some(math_mode),
        ..ConvertOptions::default()
    };

    convert_to_string("The famous $E = mc^2$ equation.", &options)
}

fn code_html(highlight_languages: HighlightLanguages, lang: &str) -> String {
    let options = ConvertOptions {
        highlight_languages,
        ..ConvertOptions::default()
    };

    convert_to_string(format!("```{lang}\nsomething\n```\n").as_str(), &options)
}

#[test]
fn math_assets_are_only_added_when_the_document_has_math() {
    let options = ConvertOptions::default();

    let without_math = convert_to_string("It costs $5 and $10.", &options);
    let with_math = convert_to_string("The famous $E = mc^2$ equation.", &options);

    assert!(!without_math.contains("npm/katex"));
    assert!(with_math.contains("npm/katex"));
    assert!(with_math.contains("data-math-style=\"inline\""));
}

#[test]
fn an_embedded_math_mode_carries_the_whole_library() {
    let mathjax = math_html(MathMode::MathJaxEmbedded);
    let katex = math_html(MathMode::KatexEmbedded);

    assert!(mathjax.contains("MathJax"));
    assert!(!mathjax.contains("<script src="));

    assert!(katex.contains("katex.render"));
    assert!(!katex.contains("npm/katex"));
    // The fonts have to travel with the file for it to work offline.
    assert!(katex.contains("data:font/woff2;base64,"));
}

#[test]
fn a_client_math_mode_points_at_a_cdn() {
    let mathjax = math_html(MathMode::MathJaxClient);
    let katex = math_html(MathMode::KatexClient);

    assert!(mathjax.contains("npm/mathjax"));
    assert!(katex.contains("npm/katex"));
    assert!(katex.contains("katex.render"));

    assert!(mathjax.len() < math_html(MathMode::MathJaxEmbedded).len());
    assert!(katex.len() < math_html(MathMode::KatexEmbedded).len());
}

#[test]
fn math_is_left_alone_when_there_is_no_math_mode() {
    let options = ConvertOptions {
        math: None,
        ..ConvertOptions::default()
    };

    let html = convert_to_string("The famous $E = mc^2$ equation.", &options);

    assert!(!html.contains("npm/katex"));
    assert!(!html.contains("MathJax"));
    assert!(!html.contains("data-math-style"));
}

fn mermaid_html(mermaid_mode: MermaidMode) -> String {
    let options = ConvertOptions {
        mermaid: Some(mermaid_mode),
        ..ConvertOptions::default()
    };

    convert_to_string(MERMAID_MARKDOWN, &options)
}

#[test]
fn mermaid_assets_are_only_added_when_the_document_has_a_diagram() {
    let options = ConvertOptions::default();

    let without_diagram = convert_to_string("```rust\nfn main() {}\n```\n", &options);
    let with_diagram = convert_to_string(MERMAID_MARKDOWN, &options);

    assert!(!without_diagram.contains("npm/mermaid"));
    assert!(with_diagram.contains("npm/mermaid"));
    assert!(with_diagram.contains("mermaid.run"));
}

#[test]
fn an_embedded_mermaid_mode_carries_the_whole_library() {
    let html = mermaid_html(MermaidMode::Embedded);

    assert!(html.contains("mermaid.run"));
    assert!(!html.contains("npm/mermaid"));
    assert!(html.len() > mermaid_html(MermaidMode::Client).len());
}

#[test]
fn a_client_mermaid_mode_points_at_a_cdn() {
    let html = mermaid_html(MermaidMode::Client);

    assert!(html.contains("npm/mermaid"));
    assert!(html.contains("mermaid.run"));
}

#[test]
fn a_mermaid_block_stays_a_code_block_when_there_is_no_mermaid_mode() {
    let options = ConvertOptions {
        mermaid: None,
        highlight_languages: HighlightLanguages::Only(&["mermaid"]),
        ..ConvertOptions::default()
    };

    let html = convert_to_string(MERMAID_MARKDOWN, &options);

    assert!(!html.contains("npm/mermaid"));
    assert!(!html.contains("mermaid.run"));
    // Without Mermaid the block is ordinary code, so highlight.js may take it.
    assert!(html.contains("hljs"));
}

#[test]
fn highlight_is_embedded_only_when_a_code_block_has_a_language() {
    let options = ConvertOptions::default();

    let without_language = convert_to_string("```\nplain text\n```\n", &options);
    let with_language = convert_to_string("```rust\nfn main() {}\n```\n", &options);

    assert!(!without_language.contains("hljs"));
    assert!(with_language.contains("hljs"));
    assert!(with_language.contains("language-rust"));
}

#[test]
fn highlight_is_embedded_only_for_a_language_it_supports() {
    assert!(code_html(HighlightLanguages::BuiltIn, "rust").contains("hljs"));
    assert!(!code_html(HighlightLanguages::BuiltIn, "plantuml").contains("hljs"));
}

#[test]
fn the_built_in_languages_cover_the_aliases_and_ignore_the_case() {
    for lang in ["js", "c++", "sh", "Rust", "YAML"] {
        assert!(code_html(HighlightLanguages::BuiltIn, lang).contains("hljs"), "{lang}");
    }
}

#[test]
fn the_highlight_languages_are_configurable() {
    assert!(code_html(HighlightLanguages::Any, "plantuml").contains("hljs"));
    assert!(code_html(HighlightLanguages::Only(&["plantuml"]), "plantuml").contains("hljs"));
    assert!(!code_html(HighlightLanguages::Only(&["plantuml"]), "rust").contains("hljs"));
}

#[test]
fn the_libraries_are_loaded_after_the_article() {
    let options = ConvertOptions {
        highlight_js: Some("window.customHighlight = true;"),
        ..ConvertOptions::default()
    };

    let html = convert_to_string("$E = mc^2$\n\n```rust\nfn main() {}\n```\n", &options);
    let article_end = html.find("</article>").unwrap();

    assert!(html.find("katex.min.js").unwrap() > article_end);
    assert!(html.find("window.customHighlight").unwrap() > article_end);
}

#[test]
fn a_fixed_theme_is_written_to_the_html_element() {
    let options = ConvertOptions {
        theme: Theme::Dark,
        ..ConvertOptions::default()
    };

    let html = convert_to_string("# Hello world!", &options);

    assert!(html.contains("<html lang=\"en\" data-theme=\"dark\">"));
}

#[test]
fn the_lang_attribute_is_configurable() {
    let options = ConvertOptions {
        lang: "zh-TW",
        ..ConvertOptions::default()
    };

    let html = convert_to_string("# 你好，世界！", &options);

    assert!(html.contains("<html lang=\"zh-TW\">"));
}

#[test]
fn alerts_and_task_lists_are_rendered() {
    let markdown = "> [!NOTE]\n> Take note.\n\n- [x] Done\n- [ ] Not yet\n";

    let html = convert_to_string(markdown, &ConvertOptions::default());

    assert!(html.contains("markdown-alert-note"));
    assert!(html.contains("markdown-alert-title"));
    assert!(html.contains("task-list-item"));
}

#[test]
fn headings_get_an_anchor() {
    let html = convert_to_string("# Hello world!", &ConvertOptions::default());

    assert!(html.contains("<h1 id=\"hello-world\">"));
    assert!(html.contains("href=\"#hello-world\""));
}

#[test]
fn raw_html_is_only_kept_when_it_is_allowed() {
    let markdown = "<b>bold</b>\n\n<script>window.raw = true;</script>\n";
    let safe = convert_to_string(markdown, &ConvertOptions::default());

    let options = ConvertOptions {
        allow_unsafe: true,
        ..ConvertOptions::default()
    };
    let unsafe_ = convert_to_string(markdown, &options);

    assert!(!safe.contains("<b>bold</b>"));
    assert!(!safe.contains("window.raw"));
    assert!(unsafe_.contains("<b>bold</b>"));
    assert!(unsafe_.contains("<script>window.raw"));
}

#[test]
fn unclosed_raw_html_can_still_be_minified() {
    let options = ConvertOptions {
        allow_unsafe: true,
        ..ConvertOptions::default()
    };

    let html = convert_to_string("Use the <code> element.\n", &options);

    assert!(html.contains("<p>Use the <code> element.</p>"));
}

#[test]
fn code_whitespace_is_preserved() {
    for minify in [false, true] {
        let options = ConvertOptions {
            minify,
            ..ConvertOptions::default()
        };

        let html = convert_to_string("`a  b`\n\n```\n  a  b\n```\n", &options);

        assert!(html.contains("<code>a  b</code>"));
        assert!(html.contains("<pre><code>  a  b\n</code></pre>"));
    }
}

#[test]
fn math_whitespace_is_preserved() {
    let literal = "\\verb|a  b|";

    for minify in [false, true] {
        let options = ConvertOptions {
            minify,
            ..ConvertOptions::default()
        };

        for markdown in [
            format!("${literal}$"),
            format!("$${literal}$$"),
            format!("\\({literal}\\)"),
            format!("\\[{literal}\\]"),
            format!("$`{literal}`$"),
            format!("```math\n{literal}\n```\n"),
        ] {
            let html = convert_to_string(markdown.as_str(), &options);
            let start = html.find("data-math-style=\"").unwrap();
            let content = &html[start + html[start..].find('>').unwrap() + 1..];
            let content = &content[..content.find('<').unwrap()];

            assert_eq!(
                literal,
                html_escape::decode_html_entities(content).trim_end_matches('\n'),
                "{markdown}"
            );
        }
    }
}

#[test]
fn the_output_can_be_left_unminified() {
    let options = ConvertOptions {
        minify: false,
        ..ConvertOptions::default()
    };

    let minified = convert_to_string("# Hello world!", &ConvertOptions::default());
    let plain = convert_to_string("# Hello world!", &options);

    assert!(plain.len() > minified.len());
    assert!(plain.contains("<article class=\"markdown-body\">"));
}
