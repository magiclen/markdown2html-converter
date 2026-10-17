use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    str::FromStr,
};

/// The color theme of the output HTML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Follow the `prefers-color-scheme` media feature.
    #[default]
    Auto,
    /// Always use the light theme.
    Light,
    /// Always use the dark theme.
    Dark,
}

impl Theme {
    /// The value of the `data-theme` attribute of the `<html>` element, if the theme is fixed.
    #[inline]
    pub const fn attribute_value(self) -> Option<&'static str> {
        match self {
            Self::Auto => None,
            Self::Light => Some("light"),
            Self::Dark => Some("dark"),
        }
    }
}

impl Display for Theme {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        })
    }
}

impl FromStr for Theme {
    type Err = UnknownTheme;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err(UnknownTheme),
        }
    }
}

/// The error of parsing a [`Theme`] from a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownTheme;

impl Display for UnknownTheme {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("the theme should be one of `auto`, `light` and `dark`")
    }
}

impl std::error::Error for UnknownTheme {}

/// How the math in a Markdown file is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathMode {
    /// Embed the **MathJax** bundle; extra fonts and TeX extensions may still need the network.
    MathJaxEmbedded,
    /// Load **MathJax** from a CDN.
    MathJaxClient,
    /// Embed the whole **KaTeX** bundle, its fonts included.
    KatexEmbedded,
    /// Load **KaTeX** from a CDN.
    #[default]
    KatexClient,
}

impl MathMode {
    /// Whether this mode renders the math with **KaTeX**.
    #[inline]
    pub const fn is_katex(self) -> bool {
        matches!(self, Self::KatexEmbedded | Self::KatexClient)
    }
}

impl Display for MathMode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MathJaxEmbedded => "mathjax-embedded",
            Self::MathJaxClient => "mathjax-client",
            Self::KatexEmbedded => "katex-embedded",
            Self::KatexClient => "katex-client",
        })
    }
}

impl FromStr for MathMode {
    type Err = UnknownMathMode;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mathjax-embedded" => Ok(Self::MathJaxEmbedded),
            "mathjax-client" => Ok(Self::MathJaxClient),
            "katex-embedded" => Ok(Self::KatexEmbedded),
            "katex-client" => Ok(Self::KatexClient),
            _ => Err(UnknownMathMode),
        }
    }
}

/// The error of parsing a [`MathMode`] from a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownMathMode;

impl Display for UnknownMathMode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(
            "the math mode should be one of `mathjax-embedded`, `mathjax-client`, \
             `katex-embedded` and `katex-client`",
        )
    }
}

impl std::error::Error for UnknownMathMode {}

/// How the **Mermaid** diagrams in a Markdown file are rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MermaidMode {
    /// Embed the whole **Mermaid** bundle.
    Embedded,
    /// Load **Mermaid** from a CDN.
    #[default]
    Client,
}

impl Display for MermaidMode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Embedded => "embedded",
            Self::Client => "client",
        })
    }
}

impl FromStr for MermaidMode {
    type Err = UnknownMermaidMode;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "embedded" => Ok(Self::Embedded),
            "client" => Ok(Self::Client),
            _ => Err(UnknownMermaidMode),
        }
    }
}

/// The error of parsing a [`MermaidMode`] from a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownMermaidMode;

impl Display for UnknownMermaidMode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("the Mermaid mode should be either `embedded` or `client`")
    }
}

impl std::error::Error for UnknownMermaidMode {}

/// Which code block languages make **highlight.js** be embedded. Languages are matched case-insensitively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HighlightLanguages<'a> {
    /// The languages which the built-in **highlight.js** supports.
    #[default]
    BuiltIn,
    /// Every language.
    Any,
    /// Only the given languages.
    Only(&'a [&'a str]),
}

/// Options for [`convert`](crate::convert).
#[derive(Debug, Clone)]
pub struct ConvertOptions<'a> {
    /// The title of the HTML file. When it is `None`, the title is looked up in the front matter and then in the first level-1 heading.
    pub title:               Option<&'a str>,
    /// The title to use when no title can be found in the Markdown file.
    pub default_title:       Option<&'a str>,
    /// The `lang` attribute of the `<html>` element. An empty string omits the attribute.
    pub lang:                &'a str,
    /// The color theme.
    pub theme:               Theme,
    /// Allow raw HTML and dangerous URLs.
    pub allow_unsafe:        bool,
    /// Treat a single line break as a `<br>`.
    pub hardbreaks:          bool,
    /// Embed **highlight.js** when the Markdown file has code blocks with a supported language.
    pub highlight:           bool,
    /// Which code block languages make **highlight.js** be embedded.
    pub highlight_languages: HighlightLanguages<'a>,
    /// How the math is rendered. `None` leaves the math alone.
    pub math:                Option<MathMode>,
    /// How the **Mermaid** diagrams are rendered. `None` leaves a ```` ```mermaid ```` block as a code block.
    pub mermaid:             Option<MermaidMode>,
    /// Embed the CJK fonts.
    pub cjk_fonts:           bool,
    /// Minify the output HTML.
    pub minify:              bool,
    /// Embed local images as `data` URLs.
    pub embed_images:        bool,
    /// The directory that relative paths in the Markdown file are resolved against.
    pub base_path:           Option<&'a Path>,
    /// A stylesheet which replaces the built-in one.
    pub css:                 Option<&'a str>,
    /// A stylesheet which is appended after all the other stylesheets.
    pub extra_css:           Option<&'a str>,
    /// A script which replaces the built-in **highlight.js**.
    pub highlight_js:        Option<&'a str>,
    /// A stylesheet which replaces the built-in **highlight.js** theme.
    pub highlight_css:       Option<&'a str>,
    /// A script which replaces the built-in **MathJax**.
    pub mathjax_js:          Option<&'a str>,
    /// A script which replaces the built-in **KaTeX**.
    pub katex_js:            Option<&'a str>,
    /// A stylesheet which replaces the built-in **KaTeX** one.
    pub katex_css:           Option<&'a str>,
    /// A script which replaces the built-in **Mermaid**.
    pub mermaid_js:          Option<&'a str>,
}

impl Default for ConvertOptions<'_> {
    #[inline]
    fn default() -> Self {
        Self {
            title:               None,
            default_title:       None,
            lang:                "en",
            theme:               Theme::default(),
            allow_unsafe:        false,
            hardbreaks:          true,
            highlight:           true,
            highlight_languages: HighlightLanguages::default(),
            math:                Some(MathMode::default()),
            mermaid:             Some(MermaidMode::default()),
            cjk_fonts:           true,
            minify:              true,
            embed_images:        false,
            base_path:           None,
            css:                 None,
            extra_css:           None,
            highlight_js:        None,
            highlight_css:       None,
            mathjax_js:          None,
            katex_js:            None,
            katex_css:           None,
            mermaid_js:          None,
        }
    }
}
