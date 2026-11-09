"""Real client/LOD preparation against Aurora's offline `lods` demo."""
import argparse
import json
from pathlib import Path
import sys
import time

import bpy

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import block_live
from block_live import starter, tracking

parser = argparse.ArgumentParser()
parser.add_argument("--port", type=int, default=17655)
parser.add_argument("--hold", type=float, default=1)
parser.add_argument("--save-blend")
args = parser.parse_args(sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else [])
block_live.register()
for obj in list(bpy.data.objects):
    bpy.data.objects.remove(obj, do_unlink=True)
rig, high = starter.create(bpy.context)
high.name = "Shirt_high"
medium = high.copy()
medium.data = high.data.copy()
medium.name = "Shirt_medium"
bpy.context.scene.collection.objects.link(medium)
medium.modifiers.new("Authored Medium", "DECIMATE").ratio = 0.4
medium.hide_viewport = True
tracking.add(bpy.context.scene, [high])
entry = bpy.context.scene.block_live.objects[0]
entry.medium.mode = "EXISTING"
entry.low.mode = "AUTO"
entry.low.budget = "TRIANGLES"
entry.low.triangles = 180
entry.lowest.mode = "FALLBACK"
entry.lowest.ratio = 8
bpy.context.scene.block_live.show_lods = True
bpy.context.scene.block_live.port = args.port
assert bpy.ops.aurora.live_connect() == {"FINISHED"}
connection = block_live._client
prepared = []
send = connection.import_result


def observe(result):
    prepared.append(result)
    send(result)


connection.import_result = observe
try:
    deadline = time.monotonic() + 40
    while time.monotonic() < deadline and not prepared:
        block_live._tick()
        time.sleep(0.1)
    assert prepared, "no import request: " + connection.state()[0]
    result = prepared[0]
    assert "error" not in result, result
    counts = [sum(len(face["indices"]) // 3 for face in level["mesh"]["faces"]) for level in result["levels"]]
    assert counts[0] < counts[1] < counts[2] < counts[3]
    assert result["levels"][2]["origin"] == "Shirt_medium"
    assert medium.hide_viewport and len(bpy.context.scene.block_live.objects) == 1
    print("AURORA_LODS_ROUND_TRIP_PREPARED " + json.dumps({"triangles": counts}), flush=True)
    if args.save_blend:
        bpy.ops.wm.save_as_mainfile(filepath=str(Path(args.save_blend).resolve()))
    deadline = time.monotonic() + max(1, min(args.hold, 60))
    while time.monotonic() < deadline:
        block_live._tick()
        time.sleep(0.1)
    assert not block_live._error, block_live._error
finally:
    block_live._stop()
    block_live.unregister()
