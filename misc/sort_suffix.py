# First, run ShowCCallsScript.java on RegisterFormat, save the output
# Then send the output to the stdin of this script

from sys import stdin, stderr
import re

pattern = re.compile(r'.*RegisterFormat\(L\"(.*)\",(.*)\).*')

list = []

for line in stdin:
    match = pattern.match(line)
    if match == None:
        print(f"Skipped line: {line}", file=stderr)
        continue

    nv = match.group(1, 2)
    list.append(nv)

list.sort(key = lambda nv: nv[0])

for nv in list:
    print(f'("{nv[0]}", &[{nv[1]}][..]),')



