(function () {
    document.querySelectorAll('pre > code.language-mermaid').forEach(function (element) {
        var diagram = document.createElement('pre');

        diagram.className = 'mermaid';
        diagram.textContent = element.textContent;

        // A ```mermaid block becomes `<pre><code>`, and the whole `<pre>` has to be replaced.
        element.parentNode.parentNode.replaceChild(diagram, element.parentNode);
    });

    // Keep the original HTML because Mermaid replaces each diagram with an SVG.
    var diagrams = Array.from(document.querySelectorAll('.mermaid'), function (element) {
        return { element: element, source: element.innerHTML };
    });
    var theme = document.documentElement.getAttribute('data-theme');
    var media = theme === null ? window.matchMedia('(prefers-color-scheme: dark)') : null;
    var rendering = false;
    var pending = false;

    async function render() {
        pending = true;

        // Finish the current render before applying a newer theme.
        if (rendering) {
            return;
        }

        rendering = true;

        try {
            while (pending) {
                pending = false;

                var dark = theme === 'dark' || (media !== null && media.matches);

                // `redux-color` is the new default theme of Mermaid 12, and `redux-dark-color` is its dark version.
                mermaid.initialize({ startOnLoad: false, theme: dark ? 'redux-dark-color' : 'redux-color' });

                var nodes = diagrams.map(function (diagram) {
                    diagram.element.innerHTML = diagram.source;
                    diagram.element.removeAttribute('data-processed');

                    return diagram.element;
                });

                // `suppressErrors` lets the other diagrams render even if one is broken.
                await mermaid.run({ nodes: nodes, suppressErrors: true });
            }
        } finally {
            rendering = false;
        }
    }

    if (media !== null) {
        media.addEventListener('change', render);
    }

    render();
})();
