import json
import sys


document = json.load(sys.stdin)
scenes = document.pop("scenes")
print("{")
for key, value in document.items():
    encoded_key = json.dumps(key, ensure_ascii=False)
    encoded_value = json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    print(f"  {encoded_key}: {encoded_value},")
print('  "scenes": [')
for index, scene in enumerate(scenes):
    encoded_scene = json.dumps(scene, ensure_ascii=False, separators=(",", ":"))
    suffix = "," if index + 1 < len(scenes) else ""
    print(f"    {encoded_scene}{suffix}")
print("  ]")
print("}")
