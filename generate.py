import random

n = 1_000_000
m = 1_000_000
print(n, m)
for i in range(m):
    op = ["?", "="][random.randrange(2)]
    print(op, random.randint(0, n-1), random.randint(0, n-1))