def findCheapestPrice(n, flights, src, dst, k):
    prices = [float('inf')] * n
    prices[src] = 0

    for _ in range(k + 1):
        temp = prices[:]
        for u, v, p in flights:
            if prices[u] != float('inf'):
                temp[v] = min(temp[v], prices[u] + p)
        prices = temp

    return -1 if prices[dst] == float('inf') else prices[dst]
