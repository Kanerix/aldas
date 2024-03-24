def opposite_color(color):
    return 0 if color == 1 else 1

def is_harmonious_2_coloring(graph):
    num_vertices = len(graph)
    colors = [-1] * num_vertices

    stack = [(0, 0)]
    while stack:
        v, color = stack.pop()
        colors[v] = color

        for neighbor, edge in graph[v]:
            if colors[neighbor] == color and edge == 1:
                return "0"

            if colors[neighbor] == opposite_color(color) and edge == 0:
                return "0"

            if colors[neighbor] == -1:
                if edge == 1:
                    stack.append((neighbor, opposite_color(color)))
                else:
                    stack.append((neighbor, color))

    return "1"


n, m = map(int, input().split())

graph = {i: [] for i in range(n)}

for _ in range(m):
    u, v, c = map(int, input().split())
    graph[u].append((v, c))
    graph[v].append((u, c))

print(is_harmonious_2_coloring(graph))
