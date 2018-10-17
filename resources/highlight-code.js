$('code[class^=\'language-\']').each(function (i, block) {
    var t = $(this);
    var c = t.attr('class');
    t.attr('class', c.replace(/language-/g, ''));
    hljs.highlightBlock(block);
});