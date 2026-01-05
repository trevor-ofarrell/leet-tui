var mergeTriplets = function(triplets, target) {
    const good = new Set();
    for (const [a, b, c] of triplets) {
        if (a <= target[0] && b <= target[1] && c <= target[2]) {
            if (a === target[0]) good.add(0);
            if (b === target[1]) good.add(1);
            if (c === target[2]) good.add(2);
        }
    }
    return good.size === 3;
};
