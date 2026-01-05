from collections import defaultdict

class DetectSquares:
    def __init__(self):
        self.points = defaultdict(int)

    def add(self, point):
        self.points[tuple(point)] += 1

    def count(self, point):
        x1, y1 = point
        result = 0
        for (x2, y2), cnt in self.points.items():
            if abs(x2 - x1) != abs(y2 - y1) or x1 == x2:
                continue
            result += cnt * self.points.get((x1, y2), 0) * self.points.get((x2, y1), 0)
        return result
