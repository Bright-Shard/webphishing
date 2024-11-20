import pefile
import os

target = "winmm"

# 32bit: /usr/lib32/wine/i386-windows/{target}.dll
dll = pefile.PE(f"/usr/lib/wine/x86_64-windows/{target}.dll")
root = f"{os.path.dirname(__file__)}/.."

with open(f"{root}/exports.def", 'w') as f:
    f.write("EXPORTS\n")
    for export in dll.DIRECTORY_ENTRY_EXPORT.symbols:
        if export.name:
            name = export.name.decode()
            f.write(f"\t{name}={target}_orig.{name} @{export.ordinal}\n")
