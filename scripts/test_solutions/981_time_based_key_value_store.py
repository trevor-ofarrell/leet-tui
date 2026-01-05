from collections import defaultdict

class TimeMap:
    def __init__(self):
        self.store = defaultdict(list)

    def set(self, key, value, timestamp):
        self.store[key].append((timestamp, value))

    def get(self, key, timestamp):
        if key not in self.store:
            return ''
        arr = self.store[key]
        lo, hi = 0, len(arr) - 1
        result = ''
        while lo <= hi:
            mid = (lo + hi) // 2
            if arr[mid][0] <= timestamp:
                result = arr[mid][1]
                lo = mid + 1
            else:
                hi = mid - 1
        return result
