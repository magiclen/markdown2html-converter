use html_minifier::{HTMLMinifierError, HTMLMinifierHelper};

/// The output buffer, which either minifies what it is given or keeps it as-is.
pub(crate) struct Output {
    /// The minifier only holds the state of its scanner, so the output buffer stays here and can be given a capacity.
    minifier: Option<HTMLMinifierHelper>,
    buffer:   Vec<u8>,
}

impl Output {
    #[inline]
    pub(crate) fn new(minify: bool, capacity: usize) -> Self {
        Self {
            minifier: minify.then(|| {
                let mut minifier = HTMLMinifierHelper::new();

                minifier.minify_code = false;

                minifier
            }),
            buffer:   Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub(crate) fn digest<S: AsRef<[u8]>>(&mut self, text: S) -> Result<(), HTMLMinifierError> {
        match self.minifier {
            Some(ref mut minifier) => minifier.digest(text, &mut self.buffer),
            None => {
                self.buffer.extend_from_slice(text.as_ref());

                Ok(())
            },
        }
    }

    /// Append text which is already minified. It saves the minifier from scanning big assets again.
    ///
    /// This skips the scanner, so it must cover the whole content of a `style` or a `script` element to keep the scanner state in sync.
    #[inline]
    fn indigest<S: AsRef<[u8]>>(&mut self, text: S) {
        self.buffer.extend_from_slice(text.as_ref());
    }

    #[inline]
    pub(crate) fn style<S: AsRef<[u8]>>(&mut self, css: S) -> Result<(), HTMLMinifierError> {
        self.digest("<style>")?;
        self.digest(css)?;
        self.digest("</style>")
    }

    #[inline]
    pub(crate) fn script<S: AsRef<[u8]>>(&mut self, js: S) -> Result<(), HTMLMinifierError> {
        self.digest("<script>")?;
        self.digest(js)?;
        self.digest("</script>")
    }

    /// Write a `<link>` element which points to an external stylesheet.
    #[inline]
    pub(crate) fn stylesheet_link(&mut self, href: &str) -> Result<(), HTMLMinifierError> {
        self.digest("<link rel=\"stylesheet\" href=\"")?;
        self.digest(html_escape::encode_double_quoted_attribute(href).as_ref())?;
        self.digest("\">")
    }

    /// Write a `<script>` element which points to an external script.
    #[inline]
    pub(crate) fn external_script(&mut self, src: &str) -> Result<(), HTMLMinifierError> {
        self.digest("<script src=\"")?;
        self.digest(html_escape::encode_double_quoted_attribute(src).as_ref())?;
        self.digest("\"></script>")
    }

    /// Write a `<style>` element whose content is already minified.
    #[inline]
    pub(crate) fn minified_style<S: AsRef<[u8]>>(
        &mut self,
        css: S,
    ) -> Result<(), HTMLMinifierError> {
        self.digest("<style>")?;
        self.indigest(css);
        self.digest("</style>")
    }

    /// Write a `<style>` element out of several pieces which are all already minified.
    #[inline]
    pub(crate) fn minified_styles<S: AsRef<[u8]>>(
        &mut self,
        pieces: impl IntoIterator<Item = S>,
    ) -> Result<(), HTMLMinifierError> {
        self.digest("<style>")?;

        for piece in pieces {
            self.indigest(piece);
        }

        self.digest("</style>")
    }

    /// Write a `<script>` element whose content is already minified.
    #[inline]
    pub(crate) fn minified_script<S: AsRef<[u8]>>(
        &mut self,
        js: S,
    ) -> Result<(), HTMLMinifierError> {
        self.digest("<script>")?;
        self.indigest(js);
        self.digest("</script>")
    }

    #[inline]
    pub(crate) fn into_html(mut self) -> Result<Vec<u8>, HTMLMinifierError> {
        if let Some(ref mut minifier) = self.minifier {
            match minifier.finish(&mut self.buffer) {
                // Raw HTML kept by `allow_unsafe` can leave an element open, but the output has already been fully written, just like without minification.
                Ok(()) | Err(HTMLMinifierError::IncompleteInput) => (),
                Err(error) => return Err(error),
            }
        }

        Ok(self.buffer)
    }
}
