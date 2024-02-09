n = int(input())

def calc_pos(idx):
    return 31 - idx

while n != 0:
    out = ["?"] * 32

    for _ in range(n):
        ins, num = input().split(maxsplit=1)

        if ins == "CLEAR":
            i = int(num)
            out[calc_pos(i)] = "0"
        elif ins == "SET":
            i = int(num)
            out[calc_pos(i)] = "1"
        elif ins == "OR":
            i, j = [int(n) for n in num.split()]
            i_val = out[calc_pos(i)]
            j_val = out[calc_pos(j)]

            if i_val == "1" or j_val == "1":
                out[calc_pos(i)] = "1"
            elif i_val == "0" and j_val == "0":
                out[calc_pos(i)] = "0"
            else:
                out[calc_pos(i)] = "?"
        elif ins == "AND":
            i, j = [int(n) for n in num.split()]
            i_val = out[calc_pos(i)]
            j_val = out[calc_pos(j)]

            if i_val == "1" and j_val == "1":
                out[calc_pos(i)] = "1"
            elif i_val == "0" or j_val == "0":
                out[calc_pos(i)] = "0"
            else:
                out[calc_pos(i)] = "?"


    print("".join(out)) 
    n = int(input())
