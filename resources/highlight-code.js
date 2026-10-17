hljs.configure({ languageDetectRe: /\blang(?:uage)?-([^\s]+)/i });

document.querySelectorAll('pre code[class^="language-"]:not(.language-math)').forEach(function (element) {
    hljs.highlightElement(element);
});
