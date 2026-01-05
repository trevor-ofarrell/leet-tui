var KthLargest = function(k, nums) {
    this.k = k;
    this.heap = nums.sort((a, b) => a - b).slice(-k);
};
KthLargest.prototype.add = function(val) {
    if (this.heap.length < this.k) { this.heap.push(val); this.heap.sort((a, b) => a - b); }
    else if (val > this.heap[0]) { this.heap[0] = val; this.heap.sort((a, b) => a - b); }
    return this.heap[0];
};
