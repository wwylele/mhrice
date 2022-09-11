##
# Import TDB info

# ################################################################################################
# How to import into Ghidra:
#  1. Run the game and create a minidump (full dump).
#  2. Run `mhrice read-dmp-tdb --dmp minidump.dmp --json-split OUTPUT_FOLDER`.`
#  3. Run Ghidra. Make sure you have 8GB RAM available for it.
#  4. Load minidump into Ghidra.
#      - You can first convert the minidump into memory chunks using some other tool.
#      - Then load the memory chunk as raw memory into Ghidra.
#      - The most important one is the main program at 0x0000000140000000.
#  4. Run this script with OUTPUT_FOLDER specified above.
#  5. Enjoy
#
#################################################################################################


import json
import os

from ghidra.program.model.data import *
from ghidra.app.services import *
from ghidra.program.model.symbol.SourceType import *
from ghidra.program.model.listing.Function.FunctionUpdateType import *
from ghidra.program.model.listing import *

builtInTypeManager = state.getTool().getService(DataTypeManagerService).getBuiltInDataTypesManager()
undefined = builtInTypeManager.getDataType("/undefined1")
void = builtInTypeManager.getDataType("/void")
t_int = builtInTypeManager.getDataType("/int")

folder = askDirectory("Give me the folder output from `mhrice read-dmp-tdb --json-split", "Go!")

chunk = 1000
i = 0
files = []
while True:
    try:
        f = open(os.path.join(folder.absolutePath, str(i) + ".json"), "rb")
    except:
        i -= chunk
        break
    files.append(f)
    i += chunk

last = json.load(files[-1])
typeCount = i + len(last["types"])

print("typeCount = " + str(typeCount))

def types():
    for f in files:
        f.seek(0, 0)
        j = json.load(f)
        start = j["start_index"]
        print("    " + str(start))
        for i, t in enumerate(j["types"]):
            yield (start + i, t)


isValType = [False] * typeCount
typeHandle = [None] * typeCount
isPrimitive = [True] * typeCount
typeSize = [0] * typeCount
isPlaceholder = [False] * typeCount
typeManager = currentProgram.getDataTypeManager()
isTemplate = [False] * typeCount
base = [None] * typeCount
rawFields = [None] * typeCount
fullName = [None] * typeCount
dearray = [None] * typeCount
headerLen = [0] * typeCount

print("First scan, add type def label")

for (i, t) in types():
    try:
        createLabel(toAddr(t["type_def_address"]), "TypeDef:"+ t["full_name"], False, USER_DEFINED)
    except:
        print("Couldn't create type def label for type " + t["full_name"])

    isValType[i] = t["via.clr.VMObjType"] == 5
    if isValType[i]:
        typeSize[i] = t["len"]
    else:
        typeSize[i] = t["runtime_len"]
        headerLen[i] = typeSize[i] - t["len"]
    isPlaceholder[i] = "!" in t["full_name"]
    isTemplate[i] = t["generics"] is not None and t["generics"].has_key("Template")
    base[i] = t["ti_base"]
    fullName[i] = t["full_name"]
    dearray[i] = t["ti_dearray"]

    rawFields[i] = []

    for field in t["fields"]:
        if (field["via.clr.FieldFlag"] & 0x10) != 0:
            continue
        rawFields[i].append((field["name"], field["ti"], field["position"]))

    st = t["via.clr.SystemType"]
    if st == 13:
        typeHandle[i] = builtInTypeManager.getDataType("/byte")
    elif st == 14:
        typeHandle[i] = builtInTypeManager.getDataType("/sbyte")
    elif st == 15:
        typeHandle[i] = builtInTypeManager.getDataType("/ushort")
    elif st == 16:
        typeHandle[i] = builtInTypeManager.getDataType("/short")
    elif st == 17:
        typeHandle[i] = builtInTypeManager.getDataType("/ushort")
    elif st == 18:
        typeHandle[i] = builtInTypeManager.getDataType("/int")
    elif st == 19:
        typeHandle[i] = builtInTypeManager.getDataType("/uint")
    elif st == 20:
        typeHandle[i] = builtInTypeManager.getDataType("/longlong")
    elif st == 21:
        typeHandle[i] = builtInTypeManager.getDataType("/ulonglong")
    elif st == 22:
        typeHandle[i] = builtInTypeManager.getDataType("/float")
    elif st == 23:
        typeHandle[i] = builtInTypeManager.getDataType("/double")
    elif st == 24:
        typeHandle[i] = builtInTypeManager.getDataType("/longlong")
    elif st == 25:
        typeHandle[i] = builtInTypeManager.getDataType("/ulonglong")
    elif st == 26:
        typeHandle[i] = builtInTypeManager.getDataType("/bool")
    elif st == 31:
        typeHandle[i] = void
    else:
        s = StructureDataType(CategoryPath("/TDB"), fullName[i], typeSize[i])
        typeHandle[i] = typeManager.addDataType(s, DataTypeConflictHandler.REPLACE_HANDLER)
        isPrimitive[i] = False
    if typeHandle[i] is None:
        raise Exception("Failed to create type for " + fullName[i])

print("Add all fields...")

for i in range(typeCount):
    if i % 1000 == 0:
        print("    " + str(i))

    if isPrimitive[i] or isPlaceholder[i] or isTemplate[i]:
        continue

    fields = []

    curI = i
    while True:
        for name, ti, raw_position in rawFields[curI]:
            if isValType[ti]:
                fieldType = typeHandle[ti]
                len = typeSize[ti]
            else:
                fieldType = PointerDataType(typeHandle[ti], 8, typeManager)
                len = 8

            position = raw_position + headerLen[i]
            fields.append((position, fieldType, len, name))
        curI = base[curI]
        if curI is None:
            break

    if headerLen[i] >= 16:
        fields.append((0, PointerDataType(void, 8, typeManager), 8, "$vtable"))
        fields.append((8, PointerDataType(void, 8, typeManager), 8, "$lock"))

    def fieldPos(f):
        return f[0]

    fields.sort(key=fieldPos)
    handle = typeHandle[i]

    handle.deleteAll()
    currentPos = 0
    for (pos, type, len, name) in fields:
        if len == 0:
            continue
        if currentPos > pos:
            print("WARNING: skipped field " + name + " for type " + fullName[i] + " because of overlap")
            continue
        if currentPos < pos:
            handle.add(ArrayDataType(undefined, pos - currentPos, 1, typeManager), pos - currentPos, "", "")
            currentPos = pos

        handle.add(type, len, name, "")
        currentPos += len

    if currentPos > typeSize[i]:
        print("WARNING: member overflow size for type " + fullName[i])

    if currentPos < typeSize[i]:
        handle.add(ArrayDataType(undefined, typeSize[i] - currentPos, 1, typeManager), typeSize[i] - currentPos, "", "")
        currentPos = typeSize[i]

    if dearray[i] is not None:
        handle.add(t_int, 4, "$x", "")
        handle.add(t_int, 4, "$y", "")
        handle.add(t_int, 4, "$rank", "")
        handle.add(t_int, 4, "$count", "")

        if isValType[dearray[i]]:
            eleType = typeHandle[dearray[i]]
            eleLen = typeSize[dearray[i]]
        else:
            eleType = PointerDataType(typeHandle[dearray[i]], 8, typeManager)
            eleLen = 8

        dummyArrayLen = 42
        handle.add(ArrayDataType(eleType, dummyArrayLen, eleLen, typeManager), dummyArrayLen * eleLen, "$data", "")


print("Add all methods...")

functionManager = currentProgram.getFunctionManager()

for (i, t) in types():
    for method in t["methods"]:
        name = t["full_name"] + "." + method["name"]
        if method["runtime_address"] == 0:
            continue
        address = toAddr(method["runtime_address"])
        func = functionManager.getFunctionAt(address)
        if func is None:
            func = createFunction(address, name)
            if func is None:
                print("WARNING: could not create function " + name + " at " + address.toString())
                continue
        else:
            func.setName(name, USER_DEFINED)

        params = []

        if isValType[method["ret"]["ti"]]:
            if typeSize[method["ret"]["ti"]] > 8:
                func.setReturnType(void, USER_DEFINED)
                params.append(ParameterImpl("$ret", PointerDataType(typeHandle[method["ret"]["ti"]], 8, typeManager), currentProgram))
            else:
                func.setReturnType(typeHandle[method["ret"]["ti"]], USER_DEFINED)
        else:
            func.setReturnType(PointerDataType(typeHandle[method["ret"]["ti"]]), USER_DEFINED)

        params.append(ParameterImpl("$ctx", PointerDataType(void, 8, typeManager), currentProgram))
        if (method["via.clr.MethodFlag"] & 0x00000010) == 0:
            params.append(ParameterImpl("$this", PointerDataType(typeHandle[i], 8, typeManager), currentProgram))

        for param in method["params"]:
            if isValType[param["ti"]]:
                paramType = typeHandle[param["ti"]]
            else:
                paramType = PointerDataType(typeHandle[param["ti"]], 8, typeManager)
            if (param["via.clr.ParamModifier"] & 1) != 0:
                paramType = PointerDataType(paramType, 8, typeManager)
            params.append(ParameterImpl(param["name"], paramType, currentProgram))

        func.replaceParameters(params, DYNAMIC_STORAGE_ALL_PARAMS, True, USER_DEFINED)

