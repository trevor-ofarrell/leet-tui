var TimeMap = function() { this.store = new Map(); };
TimeMap.prototype.set = function(key, value, timestamp) {
    if (!this.store.has(key)) this.store.set(key, []);
    this.store.get(key).push([timestamp, value]);
};
TimeMap.prototype.get = function(key, timestamp) {
    if (!this.store.has(key)) return '';
    const arr = this.store.get(key);
    let lo = 0, hi = arr.length - 1, result = '';
    while (lo <= hi) {
        const mid = Math.floor((lo + hi) / 2);
        if (arr[mid][0] <= timestamp) { result = arr[mid][1]; lo = mid + 1; }
        else hi = mid - 1;
    }
    return result;
};
