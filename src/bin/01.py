with open("../../data/examples/01.txt") as f:
    lines = f.readlines()

lst1 = []
lst2 = []
for line in lines:
    print(line)
    toks = line.split(" ")
    lst1.append(int(toks[0]))
    lst2.append(int(toks[1]))

lst1 = sorted(lst1)
lst2 = sorted(lst2)

print(
    sum(
        abs(a - b) for a,b in zip(lst1, lst2)
    )
)
