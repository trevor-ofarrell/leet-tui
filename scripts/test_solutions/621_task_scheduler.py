def leastInterval(tasks, n):
    freq = [0] * 26
    for t in tasks:
        freq[ord(t) - 65] += 1
    max_freq = max(freq)
    max_count = sum(1 for f in freq if f == max_freq)
    return max(len(tasks), (max_freq - 1) * (n + 1) + max_count)
