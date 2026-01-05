var encode = function(strs) {
    return strs.map(s => s.length + '#' + s).join('');
};
var decode = function(s) {
    const result = [];
    let i = 0;
    while (i < s.length) {
        let j = i;
        while (s[j] !== '#') j++;
        const len = parseInt(s.substring(i, j));
        result.push(s.substring(j + 1, j + 1 + len));
        i = j + 1 + len;
    }
    return result;
};
