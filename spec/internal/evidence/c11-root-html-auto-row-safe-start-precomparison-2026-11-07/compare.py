import json
from pathlib import Path

root = Path(__file__).parent
observed = json.loads((root / "chrome-observation.json").read_text())
expected = json.loads((root / "expected-values.json").read_text())
observed_by_id = {scene["id"]: scene for scene in observed["scenes"]}
failures = []
coordinate_count = 0
max_delta = 0.0
scene_results = []

for scene in expected["scenes"]:
    actual = observed_by_id.get(scene["id"])
    scene_failures = []
    if actual is None:
        scene_failures.append("missing scene")
    else:
        if actual["alignContent"] != scene["computed_align_content"]:
            scene_failures.append("computed align-content mismatch")
        if actual["rowGap"] != scene["row_gap"]:
            scene_failures.append("computed row-gap mismatch")
        rows = [float(value.removesuffix("px")) for value in actual["gridTemplateRows"].split()]
        if rows != expected["grid_template_rows_px"]:
            scene_failures.append("used row sizes mismatch")
        if len(actual["items"]) != len(scene["item_y_px"]):
            scene_failures.append("item count mismatch")
        else:
            for index, (item, expected_y) in enumerate(zip(actual["items"], scene["item_y_px"])):
                for key, expected_value in (("x", 0), ("y", expected_y), ("width", expected["item_size_px"][0]), ("height", expected["item_size_px"][1])):
                    delta = abs(item[key] - expected_value)
                    coordinate_count += 1
                    max_delta = max(max_delta, delta)
                    if delta > expected["tolerance_css_px"]:
                        scene_failures.append(f"item {index} {key} differs by {delta} CSS px")
    failures.extend(f"{scene['id']}: {failure}" for failure in scene_failures)
    scene_results.append({"id": scene["id"], "passed": not scene_failures, "failures": scene_failures})

result = {
    "passed": not failures,
    "scene_count": len(expected["scenes"]),
    "item_frame_coordinate_count": coordinate_count,
    "max_absolute_delta_css_px": max_delta,
    "tolerance_css_px": expected["tolerance_css_px"],
    "scene_results": scene_results,
    "failures": failures,
}
(root / "comparison.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
if failures:
    raise SystemExit(1)
