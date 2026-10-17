pub(crate) const PAGE_CSS: &str = include_str!("../resources/page.css");
pub(crate) const PAGE_CSS_LIGHT: &str = include_str!("../resources/page-light.css");
pub(crate) const PAGE_CSS_DARK: &str = include_str!("../resources/page-dark.css");

pub(crate) const MARKDOWN_CSS: &str = include_str!("../resources/github-markdown.css");
pub(crate) const MARKDOWN_CSS_LIGHT: &str = include_str!("../resources/github-markdown-light.css");
pub(crate) const MARKDOWN_CSS_DARK: &str = include_str!("../resources/github-markdown-dark.css");

pub(crate) const FONT_CJK_CSS: &str = include_str!("../resources/font-cjk.css");
pub(crate) const FONT_CJK_MONO_CSS: &str = include_str!("../resources/font-cjk-mono.css");

pub(crate) const HIGHLIGHT_CSS_LIGHT: &str = include_str!("../resources/highlight-light.min.css");
pub(crate) const HIGHLIGHT_CSS_DARK: &str = include_str!("../resources/highlight-dark.min.css");
pub(crate) const HIGHLIGHT_JS: &str = include_str!("../resources/highlight.min.js");
pub(crate) const HIGHLIGHT_CODE_JS: &str = include_str!("../resources/highlight-code.js");
pub(crate) const HIGHLIGHT_LANGUAGES: &str = include_str!("../resources/highlight-languages.txt");

pub(crate) const MATH_JAX_JS: &str = include_str!("../resources/mathjax.min.js");
pub(crate) const MATH_JAX_CONFIG_JS: &str = include_str!("../resources/mathjax-config.js");

/// The `resources/katex.min.css` file carries its fonts as `data` URLs, so it is much bigger than the one of **KaTeX** itself.
pub(crate) const KATEX_CSS: &str = include_str!("../resources/katex.min.css");
pub(crate) const KATEX_JS: &str = include_str!("../resources/katex.min.js");
pub(crate) const KATEX_RENDER_JS: &str = include_str!("../resources/katex-render.js");

/// The `resources/mermaid.min.js` file is the IIFE build, which carries every diagram type instead of loading them on demand.
pub(crate) const MERMAID_JS: &str = include_str!("../resources/mermaid.min.js");
pub(crate) const MERMAID_RENDER_JS: &str = include_str!("../resources/mermaid-render.js");

macro_rules! mathjax_cdn_root {
    () => {
        "https://cdn.jsdelivr.net/npm/mathjax@4.1.3"
    };
}

pub(crate) const MATH_JAX_CDN_ROOT: &str = mathjax_cdn_root!();
pub(crate) const MATH_JAX_CDN_JS: &str = concat!(mathjax_cdn_root!(), "/tex-mml-svg.js");
pub(crate) const KATEX_CDN_CSS: &str =
    "https://cdn.jsdelivr.net/npm/katex@0.19.0/dist/katex.min.css";
pub(crate) const KATEX_CDN_JS: &str = "https://cdn.jsdelivr.net/npm/katex@0.19.0/dist/katex.min.js";
pub(crate) const MERMAID_CDN_JS: &str =
    "https://cdn.jsdelivr.net/npm/mermaid@12.2.0/dist/mermaid.min.js";
