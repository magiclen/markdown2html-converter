$('code[class^=\'language-\']').each(function () {
    var t = $(this);
    var c = t.attr('class');
    t.attr('class', c.replace(/language-/g, ''));
});

hljs.initHighlightingOnLoad();