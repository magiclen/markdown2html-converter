document.querySelectorAll('pre > code.language-mermaid').forEach(function (element) {
    var diagram = document.createElement('pre');

    diagram.className = 'mermaid';
    diagram.textContent = element.textContent;

    // A ```mermaid block becomes `<pre><code>`, and the whole `<pre>` has to be replaced.
    element.parentNode.parentNode.replaceChild(diagram, element.parentNode);
});

// A fixed theme wins, and without one the diagrams follow the same media feature as the rest of the page.
var theme = document.documentElement.getAttribute('data-theme');
var dark = theme === 'dark'
    || (theme === null && window.matchMedia('(prefers-color-scheme: dark)').matches);

// `redux-color` is the new default theme of Mermaid 12, and `redux-dark-color` is its dark version.
mermaid.initialize({ startOnLoad: false, theme: dark ? 'redux-dark-color' : 'redux-color' });

// `querySelector` has to be repeated here, because its default only applies when `run` is given no argument at all.
// `suppressErrors` keeps a single broken diagram from stopping the rest of the document from being drawn.
mermaid.run({ querySelector: '.mermaid', suppressErrors: true });
