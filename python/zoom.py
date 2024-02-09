line = input()
_, k = line.split()
k = int(k)

num_list = input()
zoomed = []

for (i, c) in enumerate(num_list.split(), start=1):
    c = int(c)
    if i % k == 0:
        zoomed.append(c)

print(" ".join(str(x) for x in zoomed))