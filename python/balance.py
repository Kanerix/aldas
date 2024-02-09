b_o = 0
b_c = 0
c_o = 0
c_c = 0

for c in input():
    if c == "(":
        b_o += 1
    elif c == ")" and b_o > b_c:
        b_c += 1
    elif c == "[":
        c_o += 1
    elif c == "]" and c_o > c_c:
        c_c += 1

if b_o == b_c and c_o == c_c: 
    print("1")
else:
    print("0")
