Markdown to HTML Converter
====================

[![CI](https://github.com/magiclen/markdown2html-converter/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/markdown2html-converter/actions/workflows/ci.yml)

Markdown to HTML Converter is a free tool for converting a Markdown file to a single HTML file with built-in CSS and JS.

## Help

```
EXAMPLES:
markdown2html-converter /path/to/file.md                           # Convert /path/to/file.md to /path/to/file.html, titled "file"
markdown2html-converter /path/to/file.md -o /path/to/output.html   # Convert /path/to/file.md to /path/to/output.html, titled "output"
markdown2html-converter /path/to/file.md -t 'Hello World!'         # Convert /path/to/file.md to /path/to/file.html, titled "Hello World!"

Usage: markdown2html-converter [OPTIONS] <MARKDOWN_PATH>

Arguments:
  <MARKDOWN_PATH>  Specify the path of your Markdown file

Options:
  -t, --title <TITLE>                            Specify the title of your HTML file
  -o, --html-path <HTML_PATH>                    Specify the path of your HTML file
  -f, --force                                    Force to output if the HTML file exists
      --no-safe                                  Allow raw HTML and dangerous URLs
      --no-highlight                             Not allow to use highlight.js
      --no-mathjax                               Not allow to use mathjax.js
      --no-cjk-fonts                             Not allow to use CJK fonts
      --css-path <CSS_PATH>                      Specify the path of your custom CSS file
      --highlight-js-path <HIGHLIGHT_JS_PATH>    Specify the path of your custom highlight.js file
      --highlight-css-path <HIGHLIGHT_CSS_PATH>  Specify the path of your custom CSS file for highlight.js code blocks
      --mathjax-js-path <MATHJAX_JS_PATH>        Specify the path of your custom single MATH_JAX.js file
  -h, --help                                     Print help
  -V, --version                                  Print version
```

## Dependency

Markdown is converted to HTML by the [comrak](https://crates.io/crates/comrak) crate. The default stylesheet (the CSS file) is from [sindresorhus/github-markdown-css](https://github.com/sindresorhus/github-markdown-css). 

If ` ``` ` is used in the input Markdown file, the [highlight.js](https://highlightjs.org/) will be automatically embedded in the output HTML file. The preset supported languages are listed below.

* Apache
* Bash
* C
* C#
* C++
* CSS
* Diff
* Dockerfile
* Go
* HTML, XML
* JSON
* Java
* JavaScript
* Kotlin
* Less
* Lua
* Makefile
* Markdown
* Nginx
* Objective-C
* PHP
* PHP Template
* Perl
* Python
* Python REPL
* R
* Ruby
* Rust
* SCSS
* SQL
* Shell
* Swift
* TOML, INI
* TypeScript
* Visual Basic .NET
* YAML

If `#{{` - `}}#` or `#{{{` - `}}}#` is used in the input Markdown file, the [mathjax.js](https://www.mathjax.org/) will be automatically embedded in the output HTML file. `#{{` and `}}#` are `inlineMath` delimiters. `#{{{` and `}}}#` are `displayMath` delimiters. The default **mathjax.js** are using the [tex-mml-chtml](http://docs.mathjax.org/en/latest/web/components/combined.html#tex-mml-chtml) configuration file.

## A Markdown Example

[The Markdown File](https://github.com/magiclen/markdown2html-converter/blob/master/example.md)

[The HTML File](https://jsfiddle.net/magiclen/jgs324w0/latest)

## License

[MIT](LICENSE)