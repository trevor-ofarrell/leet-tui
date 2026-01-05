from collections import deque

def findOrder(numCourses, prerequisites):
    graph = [[] for _ in range(numCourses)]
    indegree = [0] * numCourses
    for a, b in prerequisites:
        graph[b].append(a)
        indegree[a] += 1
    queue = deque([i for i in range(numCourses) if indegree[i] == 0])
    result = []
    while queue:
        course = queue.popleft()
        result.append(course)
        for next_course in graph[course]:
            indegree[next_course] -= 1
            if indegree[next_course] == 0:
                queue.append(next_course)
    return result if len(result) == numCourses else []
