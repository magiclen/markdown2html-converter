document.querySelectorAll('code[class^="language-"]').forEach(function(element) {
    var c = element.getAttribute('class');
    element.setAttribute('class', c.replace(/language-/g, ''));

    hljs.highlightElement(element);
});