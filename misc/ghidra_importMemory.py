import os
from ghidra.util.task import TaskMonitor

folder = askDirectory("Give me the folder that contains all the binaries with file name `<ADDRESS>.bin`", "Go!")

memory = getCurrentProgram().getMemory()

for file in os.listdir(folder.absolutePath):
    print("Importing " + file)
    address = int(file[0:16], 16)
    full_path = os.path.join(folder.absolutePath, file)
    len = os.path.getsize(full_path)
    f = open(full_path, "rb")
    fb = memory.createFileBytes(file, 0, len, f, TaskMonitor.DUMMY)
    block = memory.createInitializedBlock(file, toAddr(address), fb, 0, len, False)
    block.setRead(True)
    block.setWrite(True)
    block.setExecute(True)

print("Done")

