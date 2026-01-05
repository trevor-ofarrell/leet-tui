var isHappy = function(n) {
    const seen = new Set();
    while (n !== 1 && !seen.has(n)) {
        seen.add(n);
        let sum = 0;
        while (n > 0) { sum += (n % 10) ** 2; n = Math.floor(n / 10); }
        n = sum;
    }
    return n === 1;
};
