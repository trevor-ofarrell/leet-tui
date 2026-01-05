from collections import defaultdict, deque

def alienOrder(words):
    graph = defaultdict(set)
    indegree = {c: 0 for w in words for c in w}

    for i in range(len(words) - 1):
        w1, w2 = words[i], words[i + 1]
        if len(w1) > len(w2) and w1.startswith(w2):
            return ''
        for j in range(min(len(w1), len(w2))):
            if w1[j] != w2[j]:
                if w2[j] not in graph[w1[j]]:
                    graph[w1[j]].add(w2[j])
                    indegree[w2[j]] += 1
                break

    queue = deque([c for c in indegree if indegree[c] == 0])
    result = []
    while queue:
        c = queue.popleft()
        result.append(c)
        for next_c in graph[c]:
            indegree[next_c] -= 1
            if indegree[next_c] == 0:
                queue.append(next_c)

    return ''.join(result) if len(result) == len(indegree) else ''
