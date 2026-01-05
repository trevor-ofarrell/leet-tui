var maxProfit = function(prices) {
    let sold = 0, held = -Infinity, rest = 0;
    for (const p of prices) {
        const prevSold = sold;
        sold = held + p;
        held = Math.max(held, rest - p);
        rest = Math.max(rest, prevSold);
    }
    return Math.max(sold, rest);
};
