import os
import sys

if len(sys.argv) != 2:
    print("Usage: python createday.py <day>")
    exit(1)
i = int(sys.argv[1])

# Ensure we don't overwrite
dest = "day" + f"{i:02}" + ".rs"
if os.path.isfile(dest):
    print(f"File {dest} already exists. Exiting.")
    exit(1)

# Read the code template
f_template = open("template.rs", "r")
content = f_template.read()
f_template.close()

# Add mod dayXX; to to mod.rs
f = open(f"mod.rs", "a")
f.write("pub mod day" + f"{i:02}" + ";\n")
f.close()

# Write out the code file for the day: dayXX.rs
dest = "day" + f"{i:02}" + ".rs"
newContent = content.replace("_test.txt", f"{i:02}" + "_test.txt")
f_day = open(dest, "w")
f_day.write(newContent)
f_day.close()
