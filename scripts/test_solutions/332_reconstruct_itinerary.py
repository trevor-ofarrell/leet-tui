from collections import defaultdict

def findItinerary(tickets):
    graph = defaultdict(list)
    for src, dst in tickets:
        graph[src].append(dst)
    for src in graph:
        graph[src].sort(reverse=True)

    result = []

    def dfs(airport):
        while graph[airport]:
            dfs(graph[airport].pop())
        result.append(airport)

    dfs('JFK')
    return result[::-1]
