#!/usr/bin/env python3
"""Verify the pinned Chrome observations against the independent repeat-count model."""

from __future__ import annotations

import hashlib
import json
import math
import struct
from pathlib import Path


HERE = Path(__file__).resolve().parent
REPOSITORY = HERE.parents[3]
OBSERVATIONS = HERE / "chrome-headless-observed.json"
SUMMARY = HERE / "chrome-headless-summary.html"
SUMMARY_SCREENSHOT = HERE / "chrome-headless-summary.png"
FIXTURE = (
    REPOSITORY
    / "tests/fixtures/css/c11/grid-auto-repeat-fixed-maximum-c114-precomparison.html"
)
EXPECTED_FIXTURE_SHA256 = (
    "6662e72ee62f9d5e0524a90f9092d1adec36dd201271f0f7d1bdc32cff84d7bc"
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def track_values(serialized: str) -> list[float]:
    return [float(value.removesuffix("px")) for value in serialized.split()]


def close(actual: float, expected: float) -> bool:
    return math.isclose(actual, expected, rel_tol=0.0, abs_tol=1e-6)


def resolved_maximum(token: str, basis: float) -> float:
    if token.endswith("px"):
        return float(token.removesuffix("px"))
    if token == "30%":
        return basis * 0.30
    if token == "calc(15px + 25%)":
        return 15.0 + basis * 0.25
    raise AssertionError(f"unplanned maximum breadth: {token}")


def single_track_count(basis: float, maximum: float, gap: float) -> int:
    return max(1, math.floor((basis + gap) / (maximum + gap)))


def two_track_count(basis: float, gap: float) -> int:
    count = 0
    for repetitions in range(1, 100):
        used = repetitions * (40.0 + 20.0) + (2 * repetitions - 1) * gap
        if used <= basis:
            count = repetitions
    return max(1, count)


def expected_ids() -> set[str]:
    result: set[str] = set()
    for axis in ("columns", "rows"):
        for minimum in ("auto", "min-content", "max-content"):
            for maximum_id in (
                "px40",
                "percent30",
                "calc15-plus-25-percent",
            ):
                for mode in ("auto-fill", "auto-fit"):
                    result.add(f"{axis}-{mode}-{minimum}-{maximum_id}-130")
            result.add(f"{axis}-overflow-{minimum}-max10-basis10")
        result.add(f"{axis}-auto-fill-px40-below-boundary-129")
    result.add("columns-auto-fill-fixed-minimum-control")
    for axis in ("columns", "rows"):
        for mode in ("auto-fill", "auto-fit"):
            result.add(f"{axis}-{mode}-two-track-pattern-135")
    return result


def main() -> None:
    assert sha256(FIXTURE) == EXPECTED_FIXTURE_SHA256
    scenes = json.loads(OBSERVATIONS.read_text())
    ids = [scene["id"] for scene in scenes]
    assert len(ids) == 49, len(ids)
    assert len(set(ids)) == 49, "duplicate scene ids"
    assert set(ids) == expected_ids(), "missing or unexpected scene id"
    by_id = {scene["id"]: scene for scene in scenes}

    for scene in scenes:
        axis = scene["axis"]
        assert axis in ("columns", "rows"), scene["id"]
        root = scene["root"]
        item = scene["item"]
        expected_root = (
            (scene["basis"], 40.0)
            if axis == "columns"
            else (40.0, scene["basis"])
        )
        assert close(root["width"], expected_root[0]), scene["id"]
        assert close(root["height"], expected_root[1]), scene["id"]
        assert close(root["x"], 0.0), scene["id"]
        assert close(item["x"], 0.0), scene["id"]
        assert close(item["y"], root["y"]), scene["id"]
        assert close(item["width"], 20.0), scene["id"]
        assert close(item["height"], 20.0), scene["id"]

        actual = track_values(
            scene["gridTemplateColumns"]
            if axis == "columns"
            else scene["gridTemplateRows"]
        )
        if "overflow-" in scene["id"]:
            expected = [20.0]
        elif scene["pattern"] == "two-track":
            repetitions = two_track_count(scene["basis"], gap=5.0)
            expected = (
                [40.0] + [0.0] * (2 * repetitions - 1)
                if scene["mode"] == "auto-fit"
                else [40.0, 20.0] * repetitions
            )
        else:
            maximum = resolved_maximum(scene["maximum"], scene["basis"])
            repetitions = single_track_count(scene["basis"], maximum, gap=5.0)
            expected = (
                [maximum] + [0.0] * (repetitions - 1)
                if scene["mode"] == "auto-fit"
                else [maximum] * repetitions
            )

        assert len(actual) == len(expected), (scene["id"], actual, expected)
        assert all(close(a, e) for a, e in zip(actual, expected)), (
            scene["id"],
            actual,
            expected,
        )

    summary = SUMMARY.read_text()
    summary_ids = (
        "columns-auto-fill-auto-px40-130",
        "columns-auto-fit-auto-px40-130",
        "columns-auto-fill-auto-percent30-130",
        "columns-auto-fit-auto-percent30-130",
        "columns-auto-fill-auto-calc15-plus-25-percent-130",
        "columns-auto-fit-auto-calc15-plus-25-percent-130",
    )
    for scene_id in summary_ids:
        cssom = by_id[scene_id]["gridTemplateColumns"]
        marker = f'data-observation-id="{scene_id}" data-cssom="{cssom}"'
        assert marker in summary, (scene_id, cssom)

    screenshot = SUMMARY_SCREENSHOT.read_bytes()
    assert screenshot[:8] == b"\x89PNG\r\n\x1a\n", "invalid summary PNG signature"
    width, height = struct.unpack(">II", screenshot[16:24])
    assert (width, height) == (1280, 633), (width, height)

    print(
        "PASS: 49/49 scenes and 6/6 summary CSSOM values; "
        "repeat counts, track sizes, frames, and 1280x633 PNG verified"
    )


if __name__ == "__main__":
    main()
