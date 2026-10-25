#!/usr/bin/env python3
"""Validate Chrome observations against independently calculated Ahem grid sizes."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
OBSERVATION = ROOT / "chrome156-agent-browser-dom-geometry.json"
FONT_SIZE = 10
IMAGE_HEIGHT = 16
SCENES = (
    ("single-token-min-content", "ABC", "min-content", 30, True),
    ("single-token-max-content", "ABC", "max-content", 30, True),
    ("two-words-min-content", "A BC", "min-content", 20, True),
    ("two-words-max-content", "A BC", "max-content", 40, True),
    ("three-words-min-content", "AB CDE F", "min-content", 30, True),
    ("three-words-max-content", "AB CDE F", "max-content", 80, True),
    ("collapsed-whitespace-min-content", "  A   9Z  ", "min-content", 20, True),
    ("collapsed-whitespace-max-content", "  A   9Z  ", "max-content", 40, True),
    ("spaces-only-min-content", "   ", "min-content", 16, True),
    ("spaces-only-max-content", "   ", "max-content", 16, True),
    ("spaces-only-no-image-min-content", "   ", "min-content", 0, False),
    ("spaces-only-no-image-max-content", "   ", "max-content", 0, False),
)


def expected_row(text: str, track: str, has_image: bool) -> int:
    # The accepted input is ASCII alphanumeric plus U+0020 only.
    normalized = " ".join(token for token in text.split(" ") if token)
    words = normalized.split(" ") if normalized else []
    min_content_inline = max((len(word) for word in words), default=0) * FONT_SIZE
    max_content_inline = len(normalized) * FONT_SIZE
    text_contribution = (
        min_content_inline if track == "min-content" else max_content_inline
    )
    return max(text_contribution, IMAGE_HEIGHT if has_image else 0)


def main() -> None:
    observation = json.loads(OBSERVATION.read_text())
    assert "HeadlessChrome/156." in observation["ua"]
    assert observation["viewport"] == {"width": 1280, "height": 633, "dpr": 1}
    assert observation["fontStatus"] == "loaded"
    assert observation["fontLoaded"] is True

    scenes = observation["scenes"]
    assert len(scenes) == len(SCENES) == 12
    assert len({scene["id"] for scene in scenes}) == len(scenes)
    assert {scene["id"] for scene in scenes} == {case[0] for case in SCENES}
    by_id = {scene["id"]: scene for scene in scenes}
    for index, (scene_id, text, track, expected, has_image) in enumerate(SCENES):
        scene = by_id[scene_id]
        assert scene["text"] == text, scene_id
        assert text and all(
            character == " "
            or (character.isascii() and character.isalnum())
            for character in text
        ), scene_id
        assert expected_row(text, track, has_image) == expected, scene_id
        row = expected
        assert scene["rows"] == f"{row}px", (scene["id"], scene["rows"], row)
        assert scene["columns"] == "50px 24px", scene["id"]
        assert scene["grid"]["x"] == 20, scene_id
        assert scene["grid"]["y"] == 36 + index * 92, scene_id
        assert scene["grid"]["width"] == 74, scene["id"]
        assert scene["grid"]["height"] == row, scene["id"]
        assert scene["textRect"]["x"] == 20, scene_id
        assert scene["textRect"]["y"] == 36 + index * 92, scene_id
        assert scene["textRect"]["width"] == 50, scene["id"]
        assert scene["textRect"]["height"] == row, scene["id"]
        if has_image:
            assert scene["imageRect"]["x"] == 70, scene_id
            assert scene["imageRect"]["y"] == 36 + index * 92, scene_id
            assert scene["imageRect"]["width"] == 24, scene_id
            assert scene["imageRect"]["height"] == IMAGE_HEIGHT, scene_id
            assert scene["imageComplete"] is True, scene_id
            assert scene["natural"] == {"width": 24, "height": IMAGE_HEIGHT}, scene_id
        else:
            assert scene["imageRect"] is None, scene_id
            assert scene["imageComplete"] is None, scene_id
            assert scene["natural"] is None, scene_id
        style = scene["textStyle"]
        assert style["writingMode"] == "vertical-rl", scene["id"]
        assert style["direction"] == "ltr", scene["id"]
        assert style["textOrientation"] == "mixed", scene["id"]
        assert style["fontFamily"] == "Ahem", scene["id"]
        assert style["fontSize"] == "10px", scene["id"]
        assert style["fontWeight"] == "400", scene["id"]
        assert style["fontStyle"] == "normal", scene["id"]
        assert style["fontVariantLigatures"] == "normal", scene["id"]
        assert style["lineHeight"] == "10px", scene["id"]
        assert style["whiteSpace"] == "normal", scene["id"]
        assert style["whiteSpaceCollapse"] == "collapse", scene["id"]
        assert style["textWrapMode"] == "wrap", scene["id"]
        assert style["wordBreak"] == "normal", scene["id"]
        assert style["overflowWrap"] == "normal", scene["id"]
        assert style["lineBreak"] == "normal", scene["id"]

    print("PASS: 12/12 Chrome 156 scenes match independent Ahem/track calculations (10 target + 2 no-image controls)")


if __name__ == "__main__":
    main()
