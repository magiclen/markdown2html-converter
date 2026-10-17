use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Write},
    fs,
    path::Path,
    sync::{Arc, Mutex, PoisonError},
};

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use comrak::nodes::{Node, NodeValue};
use yaml_rust2::YamlLoader;

use crate::{ConvertOptions, HighlightLanguages, resources::HIGHLIGHT_LANGUAGES};

/// Which optional assets a document needs.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct UsedAssets {
    pub(crate) highlight: bool,
    pub(crate) math:      bool,
    pub(crate) mermaid:   bool,
}

// `context.user` is whether the output is going to be minified.
comrak::create_formatter!(HtmlFormatter<bool>, {
    NodeValue::Math(ref math) => |context, node, entering| {
        if !context.user || !math.dollar_math {
            return comrak::html::render_math(context, node, entering, math);
        }

        if entering {
            context.write_str(if math.display_math {
                "<span data-math-style=\"display\">"
            } else {
                "<span data-math-style=\"inline\">"
            })?;

            escape_math(context, &math.literal)?;
            context.write_str("</span>")?;
        }
    },
});

fn escape_math(context: &mut comrak::html::Context<bool>, literal: &str) -> fmt::Result {
    let mut start = 0;

    // Character references keep the HTML minifier from changing whitespace in TeX.
    for (index, byte) in literal.bytes().enumerate() {
        let escaped = match byte {
            b' ' => "&#32;",
            b'\t' => "&#9;",
            b'\n' => "&#10;",
            b'\r' => "&#13;",
            b'\x0C' => "&#12;",
            _ => continue,
        };

        context.escape(&literal[start..index])?;
        context.write_str(escaped)?;
        start = index + 1;
    }

    context.escape(&literal[start..])
}

/// Build the **comrak** options matching the given conversion options.
pub(crate) fn build_comrak_options(options: &ConvertOptions) -> comrak::Options<'static> {
    let mut comrak_options = comrak::Options::default();

    comrak_options.render.r#unsafe = options.allow_unsafe;
    comrak_options.render.hardbreaks = options.hardbreaks;
    comrak_options.render.gfm_quirks = true;
    comrak_options.render.tasklist_classes = true;

    comrak_options.extension.alerts = true;
    comrak_options.extension.autolink = true;
    comrak_options.extension.cjk_friendly_emphasis = true;
    comrak_options.extension.description_lists = true;
    comrak_options.extension.footnotes = true;
    comrak_options.extension.front_matter_delimiter = Some(String::from("---"));
    comrak_options.extension.header_id_prefix = Some(String::new());
    comrak_options.extension.multiline_block_quotes = true;
    comrak_options.extension.strikethrough = true;
    comrak_options.extension.subscript = true;
    comrak_options.extension.superscript = true;
    comrak_options.extension.table = true;
    comrak_options.extension.tasklist = true;

    comrak_options.parse.relaxed_tasklist_matching = true;

    // Without a renderer the math would be shown as raw TeX, so the delimiters are better left alone.
    if options.math.is_some() {
        comrak_options.extension.math_code = true;
        comrak_options.extension.math_dollars = true;
        comrak_options.extension.math_latex = true;
    }

    if options.embed_images {
        // An empty base path stands for the current directory, which `canonicalize` cannot resolve on its own.
        let base_path = match options.base_path {
            Some(path) if !path.as_os_str().is_empty() => path,
            _ => Path::new("."),
        };

        // Resolving the base path once here keeps the check for an image which escapes it down to one syscall per image.
        let real_base_path = fs::canonicalize(base_path).ok();
        let base_path = base_path.to_path_buf();

        // The same image may be referenced many times, and reading and encoding it again for each of them would be wasteful.
        let embedded: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());

        comrak_options.extension.image_url_rewriter = Some(Arc::new(move |url: &str| {
            let mut embedded = embedded.lock().unwrap_or_else(PoisonError::into_inner);

            embedded
                .entry(url.to_string())
                .or_insert_with(|| {
                    real_base_path
                        .as_deref()
                        .and_then(|real_base_path| {
                            embed_image(base_path.as_path(), real_base_path, url)
                        })
                        .unwrap_or_else(|| url.to_string())
                })
                .clone()
        }));
    }

    comrak_options
}

/// Find the title of a document, first in its front matter and then in its first level-1 heading.
pub(crate) fn document_title(root: Node) -> Option<String> {
    for node in root.children() {
        match node.data.borrow().value {
            NodeValue::FrontMatter(ref front_matter) => {
                if let Some(title) = front_matter_title(front_matter) {
                    return Some(title);
                }
            },
            NodeValue::Heading(heading) if heading.level == 1 => {
                let text = node.collect_text();
                let title = text.trim();

                if !title.is_empty() {
                    // A front matter is always the first node, so nothing can override this title any more.
                    return Some(title.to_string());
                }
            },
            _ => (),
        }
    }

    None
}

/// Check which optional assets a document needs.
pub(crate) fn used_assets(root: Node, options: &ConvertOptions) -> UsedAssets {
    let wants_highlight = options.highlight;
    let wants_math = options.math.is_some();
    // A ```mermaid block is a diagram only when Mermaid is going to draw it, and an ordinary code block otherwise.
    let wants_mermaid = options.mermaid.is_some();

    let mut used_assets = UsedAssets::default();

    if !wants_highlight && !wants_math && !wants_mermaid {
        return used_assets;
    }

    for node in root.descendants() {
        match node.data.borrow().value {
            NodeValue::CodeBlock(ref code_block) => {
                let lang = code_block.info.split_whitespace().next().unwrap_or("");

                if lang == "math" {
                    used_assets.math = true;
                } else if wants_mermaid && lang == "mermaid" {
                    used_assets.mermaid = true;
                } else if wants_highlight
                    && !used_assets.highlight
                    && is_highlighted(options.highlight_languages, lang)
                {
                    // Code blocks without a language, or with one highlight.js cannot handle, are left alone.
                    used_assets.highlight = true;
                }
            },
            NodeValue::Math(..) => used_assets.math = true,
            _ => (),
        }

        if (used_assets.highlight || !wants_highlight)
            && (used_assets.math || !wants_math)
            && (used_assets.mermaid || !wants_mermaid)
        {
            break;
        }
    }

    used_assets
}

/// Check whether **highlight.js** is expected to handle a code block language.
fn is_highlighted(languages: HighlightLanguages, lang: &str) -> bool {
    if lang.is_empty() {
        return false;
    }

    match languages {
        HighlightLanguages::BuiltIn => HIGHLIGHT_LANGUAGES
            .lines()
            .filter(|line| !line.starts_with('#'))
            .any(|name| name.eq_ignore_ascii_case(lang)),
        HighlightLanguages::Any => true,
        HighlightLanguages::Only(languages) => {
            languages.iter().any(|name| name.eq_ignore_ascii_case(lang))
        },
    }
}

/// Read the `title` entry out of a YAML front matter.
fn front_matter_title(front_matter: &str) -> Option<String> {
    // The front matter still carries its `---` delimiters, so the title is in the first document.
    let documents = YamlLoader::load_from_str(front_matter).ok()?;
    let title = documents.first()?["title"].as_str()?.trim();

    (!title.is_empty()).then(|| title.to_string())
}

/// Turn a local image into a `data` URL. Only the images which stay inside the base path are embedded.
///
/// `real_base_path` is the canonical form of `base_path`. The two are kept apart because a canonical path is verbatim on Windows, where `..` is then no longer resolved.
fn embed_image(base_path: &Path, real_base_path: &Path, url: &str) -> Option<String> {
    if url.is_empty() || url.starts_with(['#', '/', '\\']) || has_scheme(url) {
        return None;
    }

    let (path_url, fragment) =
        url.split_once('#').map_or((url, None), |(path, fragment)| (path, Some(fragment)));
    let path_url = path_url.split_once('?').map_or(path_url, |(path, _)| path);
    let relative_path = percent_decode(path_url);
    let relative_path = Path::new(relative_path.as_ref());

    // A Markdown file should not be able to pull in a file from anywhere on the disk.
    if relative_path.is_absolute() {
        return None;
    }

    let mime = image_mime(relative_path.extension()?.to_str()?)?;
    let path = fs::canonicalize(base_path.join(relative_path)).ok()?;

    // `..` could still climb out of the base path, and a symbolic link could point anywhere.
    if !path.starts_with(real_base_path) {
        return None;
    }

    let image = fs::read(path).ok()?;

    let mut data_url = String::with_capacity(
        mime.len()
            + 13
            + image.len().div_ceil(3) * 4
            + fragment.map_or(0, |fragment| fragment.len() + 1),
    );

    data_url.push_str("data:");
    data_url.push_str(mime);
    data_url.push_str(";base64,");

    BASE64.encode_string(image, &mut data_url);

    if let Some(fragment) = fragment {
        data_url.push('#');
        data_url.push_str(fragment);
    }

    Some(data_url)
}

/// Check whether a URL starts with a scheme. A single letter is not treated as one, so that a Windows drive letter is left to the absolute path check.
fn has_scheme(url: &str) -> bool {
    match url.find(':') {
        Some(index) => {
            let scheme = &url[..index];

            scheme.len() > 1
                && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
                && scheme.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
        },
        None => false,
    }
}

/// Decode the percent-encoded octets of a URL. A URL which has nothing to decode is borrowed as-is.
fn percent_decode(url: &str) -> Cow<'_, str> {
    let bytes = url.as_bytes();

    // `%` is ASCII, so this byte index never falls inside a multi-byte character.
    let Some(start) = url.find('%') else {
        return Cow::Borrowed(url);
    };

    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = start;

    decoded.extend_from_slice(&bytes[..start]);

    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                match hex_octet(bytes[index + 1], bytes[index + 2]) {
                    Some(byte) => {
                        decoded.push(byte);
                        index += 3;
                    },
                    None => {
                        decoded.push(b'%');
                        index += 1;
                    },
                }
            },
            byte => {
                decoded.push(byte);
                index += 1;
            },
        }
    }

    match String::from_utf8(decoded) {
        Ok(decoded) => Cow::Owned(decoded),
        Err(_) => Cow::Borrowed(url),
    }
}

/// Decode a pair of hexadecimal digits into a byte.
fn hex_octet(high: u8, low: u8) -> Option<u8> {
    Some((hex_digit(high)? << 4) | hex_digit(low)?)
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn image_mime(extension: &str) -> Option<&'static str> {
    const MIMES: [(&str, &str); 12] = [
        ("png", "image/png"),
        ("apng", "image/apng"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("gif", "image/gif"),
        ("svg", "image/svg+xml"),
        ("webp", "image/webp"),
        ("avif", "image/avif"),
        ("bmp", "image/bmp"),
        ("ico", "image/x-icon"),
        ("tif", "image/tiff"),
        ("tiff", "image/tiff"),
    ];

    MIMES.into_iter().find(|(name, _)| extension.eq_ignore_ascii_case(name)).map(|(_, mime)| mime)
}

#[cfg(test)]
mod tests {
    use std::{env, process};

    use super::*;

    #[test]
    fn a_url_without_percent_encoding_is_borrowed() {
        assert_eq!("images/pic.png", percent_decode("images/pic.png"));
        assert!(matches!(percent_decode("images/pic.png"), Cow::Borrowed(_)));
    }

    #[test]
    fn percent_encoded_octets_are_decoded() {
        assert_eq!("a b.png", percent_decode("a%20b.png"));
        assert_eq!("a/b.png", percent_decode("a%2Fb.png"));
        assert_eq!("圖.png", percent_decode("%E5%9C%96.png"));
    }

    #[test]
    fn a_percent_which_is_not_an_escape_is_kept() {
        // A `%` followed by a non-ASCII character used to be sliced at a byte which is not a character boundary.
        assert_eq!("%圖.png", percent_decode("%圖.png"));
        assert_eq!("%zz.png", percent_decode("%zz.png"));
        // `from_str_radix` used to accept a sign, which made this an escape.
        assert_eq!("%+A.png", percent_decode("%+A.png"));
        assert_eq!("100%", percent_decode("100%"));
        assert_eq!("100%2", percent_decode("100%2"));
    }

    #[test]
    fn only_real_schemes_are_detected() {
        assert!(has_scheme("https://magiclen.org/pic.png"));
        assert!(has_scheme("data:image/png;base64,AAAA"));
        assert!(has_scheme("mailto:len@magiclen.org"));
        assert!(!has_scheme("images/pic.png"));
        assert!(!has_scheme("C:/images/pic.png"));
    }

    #[test]
    fn only_relative_images_are_embedded() {
        let dir = env::temp_dir()
            .join(format!("markdown2html-converter-unit-embed-image-{}", process::id()));
        let sub_dir = dir.join("sub");
        let path = dir.join("pic.png");

        fs::create_dir_all(sub_dir.as_path()).unwrap();
        fs::write(path.as_path(), b"an image").unwrap();

        let real_dir = fs::canonicalize(dir.as_path()).unwrap();
        let real_sub_dir = fs::canonicalize(sub_dir.as_path()).unwrap();

        let from_dir = |url: &str| embed_image(dir.as_path(), real_dir.as_path(), url);
        let from_sub_dir = |url: &str| embed_image(sub_dir.as_path(), real_sub_dir.as_path(), url);

        let data_url = Some(String::from("data:image/png;base64,YW4gaW1hZ2U="));

        assert_eq!(data_url, from_dir("pic.png"));
        // A path which leaves the base path and comes back is still inside it.
        assert_eq!(data_url, from_dir("sub/../pic.png"));
        // The file is there, but it may only be reached relatively to the base path.
        assert_eq!(None, from_sub_dir("../pic.png"));
        assert_eq!(None, from_dir(path.to_str().unwrap()));
        assert_eq!(None, from_dir("https://magiclen.org/pic.png"));
        assert_eq!(None, from_dir("//magiclen.org/pic.png"));
        assert_eq!(None, from_dir("#anchor"));

        let _ = fs::remove_dir_all(dir.as_path());
    }
}
