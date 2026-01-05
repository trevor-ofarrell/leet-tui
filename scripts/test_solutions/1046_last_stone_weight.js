var lastStoneWeight = function(stones) {
    while (stones.length > 1) {
        stones.sort((a, b) => b - a);
        const diff = stones[0] - stones[1];
        stones = stones.slice(2);
        if (diff > 0) stones.push(diff);
    }
    return stones.length ? stones[0] : 0;
};
