use std::{
    env, fs,
    io::Write,
    ops::Deref,
    path::PathBuf,
    process::{self, Command, Output, Stdio},
};

const EXECUTABLE: &str = env!("CARGO_BIN_EXE_markdown2html-converter");

/// A temporary directory which is removed when it is dropped.
struct TempDir(PathBuf);

impl Deref for TempDir {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(self.0.as_path());
    }
}

fn temp_dir(name: &str) -> TempDir {
    let path = env::temp_dir().join(format!("markdown2html-converter-{name}-{}", process::id()));

    let _ = fs::remove_dir_all(path.as_path());
    fs::create_dir_all(path.as_path()).unwrap();

    TempDir(path)
}

fn convert_stdin(markdown: &str, args: &[&str]) -> Output {
    let mut child = Command::new(EXECUTABLE)
        .arg("-")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    // A rejected argument makes the binary exit before it reads anything, and the pipe is then already closed.
    let _ = child.stdin.take().unwrap().write_all(markdown.as_bytes());

    child.wait_with_output().unwrap()
}

#[test]
fn markdown_from_stdin_goes_to_stdout() {
    let mut child = Command::new(EXECUTABLE)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.take().unwrap().write_all(b"# Hello world!\n").unwrap();

    let output = child.wait_with_output().unwrap();
    let html = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(html.contains("<title>Hello world!</title>"));
}

#[test]
fn the_html_file_is_written_next_to_the_markdown_file() {
    let dir = temp_dir("next-to");
    let markdown_path = dir.join("readme.md");

    fs::write(markdown_path.as_path(), "Just a paragraph.").unwrap();

    let status = Command::new(EXECUTABLE).arg(markdown_path.as_path()).status().unwrap();

    let html = fs::read_to_string(dir.join("readme.html")).unwrap();

    assert!(status.success());
    assert!(html.contains("<title>readme</title>"));
}

#[test]
fn the_output_file_name_is_used_as_the_fallback_title() {
    let dir = temp_dir("output-title");
    let markdown_path = dir.join("input.md");
    let html_path = dir.join("output.html");

    fs::write(markdown_path.as_path(), "Just a paragraph.").unwrap();

    let status = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .arg("-o")
        .arg(html_path.as_path())
        .status()
        .unwrap();

    let html = fs::read_to_string(html_path.as_path()).unwrap();

    assert!(status.success());
    assert!(html.contains("<title>output</title>"));
}

#[test]
fn the_markdown_file_cannot_be_overwritten() {
    let dir = temp_dir("protect-input");
    let markdown_path = dir.join("readme.md");

    fs::write(markdown_path.as_path(), "# Original Markdown").unwrap();

    let status = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .arg("-o")
        .arg(markdown_path.as_path())
        .arg("--force")
        .stderr(Stdio::null())
        .status()
        .unwrap();

    assert!(!status.success());
    assert_eq!("# Original Markdown", fs::read_to_string(markdown_path.as_path()).unwrap());
}

#[test]
fn a_hard_link_to_the_markdown_file_cannot_be_overwritten() {
    let dir = temp_dir("protect-input-hard-link");
    let markdown_path = dir.join("readme.md");
    let html_path = dir.join("readme.html");

    fs::write(markdown_path.as_path(), "# Original Markdown").unwrap();
    fs::hard_link(markdown_path.as_path(), html_path.as_path()).unwrap();

    let status = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .arg("-o")
        .arg(html_path.as_path())
        .arg("--force")
        .stderr(Stdio::null())
        .status()
        .unwrap();

    assert!(!status.success());
    assert_eq!("# Original Markdown", fs::read_to_string(markdown_path.as_path()).unwrap());
}

#[test]
fn an_existing_html_file_is_only_overwritten_with_force() {
    let dir = temp_dir("force");
    let markdown_path = dir.join("readme.md");
    let html_path = dir.join("readme.html");

    fs::write(markdown_path.as_path(), "Just a paragraph.").unwrap();
    fs::write(html_path.as_path(), "old").unwrap();

    let status = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .stderr(Stdio::null())
        .status()
        .unwrap();

    assert!(!status.success());
    assert_eq!("old", fs::read_to_string(html_path.as_path()).unwrap());

    let status = Command::new(EXECUTABLE).arg(markdown_path.as_path()).arg("-f").status().unwrap();

    assert!(status.success());
    assert!(fs::read_to_string(html_path.as_path()).unwrap().contains("<!DOCTYPE html>"));
}

#[test]
fn the_math_mode_can_be_chosen_on_the_command_line() {
    let output = convert_stdin("$E = mc^2$", &["--math-mode", "mathjax-embedded"]);
    let html = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(html.contains("MathJax"));
    assert!(!html.contains("npm/katex"));
}

#[test]
fn the_math_mode_conflicts_with_no_math() {
    let output = convert_stdin("$E = mc^2$", &["--no-math", "--math-mode", "katex-client"]);

    assert!(!output.status.success());
}

#[test]
fn a_math_asset_needs_the_math_mode_it_belongs_to() {
    let output = convert_stdin("$E = mc^2$", &["--mathjax-js-path", "/dev/null"]);

    assert!(!output.status.success());
}

#[test]
fn the_highlight_languages_can_be_chosen_on_the_command_line() {
    let markdown = "```plantuml\n@startuml\n```\n";

    let by_default = convert_stdin(markdown, &[]);
    let chosen = convert_stdin(markdown, &["--highlight-languages", "plantuml"]);

    assert!(!String::from_utf8(by_default.stdout).unwrap().contains("hljs"));
    assert!(String::from_utf8(chosen.stdout).unwrap().contains("hljs"));
}

#[test]
fn the_mermaid_mode_can_be_chosen_on_the_command_line() {
    let markdown = "```mermaid\ngraph TD; A-->B;\n```\n";

    let by_default = convert_stdin(markdown, &[]);
    let embedded = convert_stdin(markdown, &["--mermaid-mode", "embedded"]);

    assert!(by_default.status.success());
    assert!(embedded.status.success());
    assert!(String::from_utf8(by_default.stdout).unwrap().contains("npm/mermaid"));
    assert!(!String::from_utf8(embedded.stdout).unwrap().contains("npm/mermaid"));
}

#[test]
fn the_mermaid_mode_conflicts_with_no_mermaid() {
    let output = convert_stdin("```mermaid\ngraph TD;\n```\n", &[
        "--no-mermaid",
        "--mermaid-mode",
        "client",
    ]);

    assert!(!output.status.success());
}

#[test]
fn local_images_can_be_embedded() {
    let dir = temp_dir("embed-images");
    let markdown_path = dir.join("readme.md");

    // The bytes only have to be readable, not a valid PNG.
    fs::write(dir.join("pic.png"), b"an image").unwrap();
    fs::write(markdown_path.as_path(), "![a picture](pic.png)").unwrap();

    let status = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .arg("-o")
        .arg(dir.join("out.html"))
        .arg("--embed-images")
        .status()
        .unwrap();

    let html = fs::read_to_string(dir.join("out.html")).unwrap();

    assert!(status.success());
    assert!(html.contains("src=\"data:image/png;base64,"));
}

#[test]
fn stdin_images_can_be_embedded_from_the_base_path() {
    let dir = temp_dir("stdin-base-path");

    fs::write(dir.join("pic.png"), b"an image").unwrap();

    let mut child = Command::new(EXECUTABLE)
        .arg("-")
        .arg("--embed-images")
        .arg("--base-path")
        .arg(dir.as_path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.take().unwrap().write_all(b"![a picture](pic.png)").unwrap();

    let output = child.wait_with_output().unwrap();
    let html = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(html.contains("src=\"data:image/png;base64,"));
}

#[test]
fn a_local_image_with_a_percent_encoded_name_can_be_embedded() {
    let dir = temp_dir("embed-image-percent-encoded");
    let markdown_path = dir.join("readme.md");

    fs::write(dir.join("圖.png"), b"an image").unwrap();
    fs::write(markdown_path.as_path(), "![a picture](%E5%9C%96.png)").unwrap();

    let output = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .arg("-o")
        .arg("-")
        .arg("--embed-images")
        .output()
        .unwrap();
    let html = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(html.contains("src=\"data:image/png;base64,YW4gaW1hZ2U=\""));
}

#[test]
fn local_images_with_queries_and_fragments_can_be_embedded() {
    let dir = temp_dir("embed-image-url-parts");
    let markdown_path = dir.join("readme.md");

    fs::write(dir.join("pic.svg"), b"an image").unwrap();
    fs::write(markdown_path.as_path(), "![a picture](pic.svg?rev=1#icon)").unwrap();

    let output = Command::new(EXECUTABLE)
        .arg(markdown_path.as_path())
        .arg("-o")
        .arg("-")
        .arg("--embed-images")
        .output()
        .unwrap();
    let html = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.success());
    assert!(html.contains("src=\"data:image/svg+xml;base64,YW4gaW1hZ2U=#icon\""));
}
