var multiply = function(num1, num2) {
    if (num1 === '0' || num2 === '0') return '0';
    const result = Array(num1.length + num2.length).fill(0);
    for (let i = num1.length - 1; i >= 0; i--) {
        for (let j = num2.length - 1; j >= 0; j--) {
            const mul = (num1[i] - '0') * (num2[j] - '0');
            const p1 = i + j, p2 = i + j + 1;
            const sum = mul + result[p2];
            result[p2] = sum % 10;
            result[p1] += Math.floor(sum / 10);
        }
    }
    let str = result.join('');
    return str.replace(/^0+/, '') || '0';
};
