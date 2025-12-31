var reverse = function(x) {
    const sign = x < 0 ? -1 : 1;
    let result = 0;
    x = Math.abs(x);
    while (x > 0) {
        result = result * 10 + (x % 10);
        x = Math.floor(x / 10);
    }
    result *= sign;
    if (result < -(2**31) || result > 2**31 - 1) return 0;
    return result;
};
