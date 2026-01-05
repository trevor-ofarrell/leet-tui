var dailyTemperatures = function(temperatures) {
    const n = temperatures.length, result = Array(n).fill(0), stack = [];
    for (let i = 0; i < n; i++) {
        while (stack.length && temperatures[i] > temperatures[stack[stack.length - 1]]) {
            const idx = stack.pop();
            result[idx] = i - idx;
        }
        stack.push(i);
    }
    return result;
};
