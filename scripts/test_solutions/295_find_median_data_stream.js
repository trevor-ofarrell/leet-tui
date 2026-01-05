var MedianFinder = function() { this.lo = []; this.hi = []; };
MedianFinder.prototype.addNum = function(num) {
    this.lo.push(num); this.lo.sort((a, b) => b - a);
    this.hi.push(this.lo.shift()); this.hi.sort((a, b) => a - b);
    if (this.hi.length > this.lo.length) {
        this.lo.push(this.hi.shift());
        this.lo.sort((a, b) => b - a);
    }
};
MedianFinder.prototype.findMedian = function() {
    return this.lo.length > this.hi.length ? this.lo[0] : (this.lo[0] + this.hi[0]) / 2;
};
