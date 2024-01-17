import sys

genes = sys.argv[1]

ATT = "" + genes[0] + genes[1]
DEF = "" + genes[2] + genes[3]
HP = "" + genes[4] + genes[5]

print("SLIME GENES:{", end='')
print("ATT = " + str(int(ATT, 16)), end='')
print(", DEF = " + str(int(DEF, 16)), end='')
print(", HP = " + str(int(HP, 16)), end='')
print(", COLOR #" + genes[6:],end='}\n')