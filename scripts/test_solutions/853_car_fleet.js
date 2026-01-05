var carFleet = function(target, position, speed) {
    const cars = position.map((p, i) => [p, (target - p) / speed[i]]).sort((a, b) => b[0] - a[0]);
    let fleets = 0, maxTime = 0;
    for (const [, time] of cars) {
        if (time > maxTime) { fleets++; maxTime = time; }
    }
    return fleets;
};
