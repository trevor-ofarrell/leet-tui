var minWindow = function(s, t) {
    if (t.length === 0) return '';
    const need = new Map();
    for (const c of t) need.set(c, (need.get(c) || 0) + 1);
    let have = 0, required = need.size;
    let left = 0, minLen = Infinity, minStart = 0;
    const window = new Map();
    for (let right = 0; right < s.length; right++) {
        const c = s[right];
        window.set(c, (window.get(c) || 0) + 1);
        if (need.has(c) && window.get(c) === need.get(c)) have++;
        while (have === required) {
            if (right - left + 1 < minLen) { minLen = right - left + 1; minStart = left; }
            const lc = s[left];
            window.set(lc, window.get(lc) - 1);
            if (need.has(lc) && window.get(lc) < need.get(lc)) have--;
            left++;
        }
    }
    return minLen === Infinity ? '' : s.substring(minStart, minStart + minLen);
};
