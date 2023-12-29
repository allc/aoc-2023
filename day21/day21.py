with open('input.txt') as f:
    grid = [list(l.strip()) for l in f.readlines()]
    for row in grid:
        print(''.join(row))
for y in range(len(grid)):
    for x in range(len(grid[y])):
        if grid[y][x] == 'S':
            start = (y, x)
            break
print(start)

stack = [(start, 0)]
visited = set()
exact = set()

while stack:
    node = stack.pop()
    if node in visited:
        continue
    visited.add(node)
    (y, x), distance = node
    if distance == 64:
        exact.add((y, x))
        continue
    for dy, dx in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        ny, nx = y + dy, x + dx
        if 0 <= ny < len(grid) and 0 <= nx < len(grid[ny]) and grid[ny][nx] != '#':
            stack.append(((ny, nx), distance + 1))
print(len(exact))
