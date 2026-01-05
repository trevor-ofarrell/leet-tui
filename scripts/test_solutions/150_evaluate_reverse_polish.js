var evalRPN = function(tokens) {
    const stack = [];
    for (const t of tokens) {
        if ('+-*/'.includes(t)) {
            const b = stack.pop(), a = stack.pop();
            if (t === '+') stack.push(a + b);
            else if (t === '-') stack.push(a - b);
            else if (t === '*') stack.push(a * b);
            else stack.push(Math.trunc(a / b));
        } else stack.push(parseInt(t));
    }
    return stack[0];
};
