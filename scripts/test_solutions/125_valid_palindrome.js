var isPalindrome = function(s) {
    s = s.toLowerCase().replace(/[^a-z0-9]/g, '');
    let left = 0, right = s.length - 1;
    while (left < right) {
        if (s[left++] !== s[right--]) return false;
    }
    return true;
};
