n, m = map(lambda x: int(x), input().split())

seats = [0] * n
votes = [float(input()) for _ in range(n)] 
quotient = votes.copy()

for i in range(m):
    i_max = quotient.index(max(quotient))
    quotient[i_max] = (votes[i_max] / (seats[i_max] + 2))
    seats[i_max] += 1
    
for s in seats:
    print(s)