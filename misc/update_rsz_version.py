import sys
import re
import os

mismatchregex = re.compile(".*Mismatch CRC (.*) for (.*)")

mismatchlist = open(sys.argv[1])
newversion = sys.argv[2]

for line in mismatchlist.readlines():
    m = mismatchregex.match(line)
    if m is None:
        continue
    crc = m.group(1)
    name = m.group(2)
    print(f"{name} -> {crc}")

    for source_file in os.listdir("src/rsz"):
        source_file = "src/rsz/" + source_file
        content = ""
        source = open(source_file)
        changed = False
        for line in source.readlines():
            content += line
            if f"#[rsz(\"{name}\"" in line:
                content += f"        0x{crc} = {newversion},\n"
                changed = True
        source.close()
        source = open(source_file, "w")
        source.write(content)
        source.close()
