#!/usr/bin/env python
# assume the current pwd is "./user"
import os

base_address = 0x8040_0000
step = 0x2_0000

linker_file = "src/linker.ld"
apps = os.listdir("src/bin")
apps.sort()

for app_id, app in enumerate(apps):
    lines = []
    with open(linker_file, "r") as f:
        for line in f.readlines():
            line = line.replace(
                hex(base_address + (app_id - 1) * step),
                hex(base_address + app_id * step),
            )
            lines.append(line)
    with open(linker_file, "w+") as f:
        f.writelines(lines)
    print("cargo build --bin %s --release" % app[: app.find(".")])
    os.system("cargo build --bin %s --release" % app[: app.find(".")])

# recover
lines = []
with open(linker_file, "r") as f:
    for line in f.readlines():
        line = line.replace(
            hex(base_address + (len(apps) - 1) * step),
            hex(base_address),
        )
        lines.append(line)
with open(linker_file, "w+") as f:
    f.writelines(lines)
