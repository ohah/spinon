import json
from pathlib import Path

ROOT = Path(__file__).parent
EPSILON = 1 / 64

expectations = json.loads((ROOT / "spec-derived-expectations.json").read_text())
observation = json.loads((ROOT / "chrome156-observation.json").read_text())
scenes = {scene["id"]: scene for scene in observation["scenes"]}


def close_lists(actual, expected, scene_id, label):
    assert len(actual) == len(expected), (scene_id, label, actual, expected)
    assert all(abs(a - e) <= EPSILON for a, e in zip(actual, expected)), (
        scene_id,
        label,
        actual,
        expected,
    )


def derive_normative_tracks(case):
    inputs = case["inputs"]
    content = inputs["contentAxisPx"]
    if "outerAxisPx" in inputs:
        border = inputs["borderEachSidePx"]
        padding = inputs["paddingEachSidePx"]
        derived_content = inputs["outerAxisPx"] - 2 * (border + padding)
        assert derived_content == content, (
            case["id"],
            derived_content,
            content,
        )
    gap = inputs["gapPx"]
    minimum = inputs["repeatMinimumPx"]
    prefix_percent = inputs["prefixPercent"]
    suffix_percent = inputs["suffixPercent"]
    prefix = content * prefix_percent / 100 if prefix_percent is not None else None
    suffix = content * suffix_percent / 100 if suffix_percent is not None else None
    neighbor_count = int(prefix is not None) + int(suffix is not None)

    def minimum_required(repeat_count):
        track_count = neighbor_count + repeat_count
        gutter_count = max(0, track_count - 1)
        return (prefix or 0) + (suffix or 0) + repeat_count * minimum + gutter_count * gap

    fitting_counts = [
        count for count in range(1, 513) if minimum_required(count) <= content + EPSILON
    ]
    repeat_count = max(fitting_counts) if fitting_counts else 1

    occupied = set(inputs["occupiedRepeatIndexes"])
    is_fit = inputs["repeatMode"] == "auto-fit"
    collapsed = [
        is_fit and index not in occupied for index in range(repeat_count)
    ]

    slots = []
    if prefix is not None:
        slots.append((prefix, False, "prefix"))
    slots.extend(
        (0 if collapsed[index] else minimum, collapsed[index], "repeat")
        for index in range(repeat_count)
    )
    if suffix is not None:
        slots.append((suffix, False, "suffix"))

    surviving_gutters = sum(
        not slots[index][1] and not slots[index + 1][1]
        for index in range(len(slots) - 1)
    )
    active_repeats = sum(not value for value in collapsed)
    base_tracks = sum(value for value, _, _ in slots)
    free_space = max(0, content - base_tracks - surviving_gutters * gap)
    flex_growth = free_space / active_repeats if active_repeats else 0

    tracks = []
    for value, is_collapsed, kind in slots:
        tracks.append(
            value + flex_growth if kind == "repeat" and not is_collapsed else value
        )
    return repeat_count, tracks, surviving_gutters, prefix


def derive_first_item_frame(case, prefix):
    inputs = case["inputs"]
    inset = (
        expectations["fixtureGeometry"]["borderEachSidePx"]
        + expectations["fixtureGeometry"]["paddingEachSidePx"]
    )
    item_size = expectations["fixtureGeometry"]["itemSizePx"]
    before_first_repeat = (prefix or 0) + (
        inputs["gapPx"] if inputs["prefixPercent"] is not None else 0
    )
    if case["axis"] == "columns":
        return [inset + before_first_repeat, inset, item_size, item_size]
    return [inset, inset + before_first_repeat, item_size, item_size]


assert len(expectations["cases"]) == 13
assert len(scenes) == 13
assert observation["devicePixelRatio"] == 1
assert "HeadlessChrome/156.0.0.0" in observation["userAgent"]

divergence_ids = []
for case in expectations["cases"]:
    scene = scenes[case["id"]]
    normative_count, normative_tracks, surviving_gutters, prefix = derive_normative_tracks(
        case
    )
    assert normative_count == case["repeatCount"], (
        case["id"],
        normative_count,
        case["repeatCount"],
    )
    close_lists(normative_tracks, case["usedTracksPx"], case["id"], "normative tracks")

    axis_values = scene["columns"] if case["axis"] == "columns" else scene["rows"]
    observed_tracks = [float(value.removesuffix("px")) for value in axis_values]
    observed_count = len(observed_tracks) - case["prefixTrackCount"] - case["suffixTrackCount"]
    assert observed_count == normative_count, (
        case["id"],
        observed_count,
        normative_count,
    )

    chrome_expected = case.get("chromeObservedTracksPx", case["usedTracksPx"])
    close_lists(observed_tracks, chrome_expected, case["id"], "Chrome observation")
    if any(abs(a - e) > EPSILON for a, e in zip(observed_tracks, normative_tracks)):
        divergence_ids.append(case["id"])

    if "itemFrameOuterPx" in case:
        derived_frame = derive_first_item_frame(case, prefix)
        assert derived_frame == case["itemFrameOuterPx"], (
            case["id"],
            derived_frame,
            case["itemFrameOuterPx"],
        )
        item = scene["item"]
        observed_frame = [item["x"], item["y"], item["width"], item["height"]]
        assert observed_frame == derived_frame, (
            case["id"],
            observed_frame,
            derived_frame,
        )

assert divergence_ids == ["col-fit-first", "row-fit-first"], divergence_ids

probe = json.loads((ROOT / "auto-fit-end-alignment-probe-2026-10-24.json").read_text())
for axis, observed_frame, expected_frame in [
    (
        "columns",
        probe["chrome156Observation"][0]["endAlignedItemFrameFromGridBorderPx"],
        probe["specDerivedExpectation"][0]["endAlignedItemFrameFromGridBorderPx"],
    ),
    (
        "rows",
        probe["chrome156Observation"][1]["endAlignedItemFrameFromGridBorderPx"],
        probe["specDerivedExpectation"][1]["endAlignedItemFrameFromGridBorderPx"],
    ),
]:
    chrome_track = 196
    normative_track = 200
    start = 2 + 8 + 24 + 4
    item_size = 2
    if axis == "columns":
        calculated_chrome = [start + chrome_track - item_size, 10, 2, 2]
        calculated_normative = [start + normative_track - item_size, 10, 2, 2]
        suffix_chrome = 10 + 24 + 4 + chrome_track + 4
        suffix_normative = 10 + 24 + 4 + normative_track
    else:
        calculated_chrome = [10, start + chrome_track - item_size, 2, 2]
        calculated_normative = [10, start + normative_track - item_size, 2, 2]
        suffix_chrome = 10 + 24 + 4 + chrome_track + 4
        suffix_normative = 10 + 24 + 4 + normative_track
    assert observed_frame == calculated_chrome, (axis, observed_frame, calculated_chrome)
    assert expected_frame == calculated_normative, (
        axis,
        expected_frame,
        calculated_normative,
    )
    assert suffix_chrome == suffix_normative == 238, (
        axis,
        suffix_chrome,
        suffix_normative,
    )

assert probe["suffixItemDiscriminatorCheck"]["chromeColumnItemFrameFromGridBorderPx"] == [
    238,
    10,
    2,
    2,
]
assert probe["suffixItemDiscriminatorCheck"]["chromeRowItemFrameFromGridBorderPx"] == [
    10,
    238,
    2,
    2,
]
print(
    "13/13 normative calculations and Chrome observations verified; "
    f"browser differences isolated to {', '.join(divergence_ids)}; "
    "end-alignment and non-discriminating suffix geometry verified"
)
