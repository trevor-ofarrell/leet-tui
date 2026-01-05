var isNStraightHand = function(hand, groupSize) {
    if (hand.length % groupSize !== 0) return false;
    const count = new Map();
    for (const card of hand) count.set(card, (count.get(card) || 0) + 1);
    const sorted = [...count.keys()].sort((a, b) => a - b);
    for (const start of sorted) {
        const cnt = count.get(start);
        if (cnt > 0) {
            for (let i = start; i < start + groupSize; i++) {
                if ((count.get(i) || 0) < cnt) return false;
                count.set(i, count.get(i) - cnt);
            }
        }
    }
    return true;
};
