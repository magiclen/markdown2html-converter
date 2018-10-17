var testStrings = '123abcABC,./繁體简体한글にっぽんご';
var cjkCounter = 0;
var cjkMonoCounter = 0;

WebFont.load({
    custom: {
        families: ['CJK:n4,n7']
    },
    testStrings: testStrings,
    fontactive: function(){
        if(++cjkCounter === 2) {
            $('.markdown-body').css('font-family', 'CJK');
        }
    }
});
WebFont.load({
    custom: {
        families: ['CJK Mono:n4,n7']
    },
    testStrings: testStrings,
    fontactive: function(){
        if(++cjkMonoCounter === 2) {
            alert('123');
        }
    }
});