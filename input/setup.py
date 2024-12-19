for i in range(1, 26):
    f = open(f"{i:02}" + ".txt", "x")
    f.close()
    f = open(f"{i:02}" + "_test.txt", "x")
    f.close()
