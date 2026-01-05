var DetectSquares = function() { this.points = new Map(); };
DetectSquares.prototype.add = function(point) {
    const key = point.join(',');
    this.points.set(key, (this.points.get(key) || 0) + 1);
};
DetectSquares.prototype.count = function(point) {
    const [x1, y1] = point;
    let result = 0;
    for (const [key, cnt] of this.points) {
        const [x2, y2] = key.split(',').map(Number);
        if (Math.abs(x2 - x1) !== Math.abs(y2 - y1) || x1 === x2) continue;
        const p3 = `${x1},${y2}`, p4 = `${x2},${y1}`;
        result += cnt * (this.points.get(p3) || 0) * (this.points.get(p4) || 0);
    }
    return result;
};
