import sys

genes = sys.argv[1]

ATT = "" + genes[0] + genes[1]
DEF = "" + genes[2] + genes[3]
HP = "" + genes[4] + genes[5]

print("SLIME GENES:")
print("ATT = " + str(int(ATT, 16)))
print("DEF = " + str(int(DEF, 16)))
print("HP = " + str(int(HP, 16)))
if len(genes) == 12:
    print("COLOR #" + genes[6:])
else:
    print("COLOR #" + genes[6:12])
    print("TIME OF BIRTH = " + str(int(genes[12:20])))
    print("MOTHER ID = " + str(int(genes[20:24])))
    print("FATHER ID = " + str(int(genes[24:28])))
    print("NUMBER OF CHILDREN = " + str(int(genes[28:30])))
    print("GENERATION = " + str(int(genes[30:32])))