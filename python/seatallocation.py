n, m = map(lambda x: int(x), input().split())

seats = [0] * n
votes = [float(input()) for _ in range(n)] 
quotients = votes.copy()

for _ in range(m):
    i_max = quotients.index(max(quotients))
    quotients[i_max] = (votes[i_max] / (seats[i_max] + 2))
    seats[i_max] += 1
    
for s in seats:
    print(s)