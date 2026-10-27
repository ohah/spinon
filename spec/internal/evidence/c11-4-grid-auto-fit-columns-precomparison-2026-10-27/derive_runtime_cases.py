#!/usr/bin/env python3
"""Derive the opt-in runtime fixture input table from the pinned WPT source."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from html.parser import HTMLParser
from pathlib import Path
from typing import Any


class ElementTree(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.stack: list[dict[str, Any]] = []
        self.roots: list[dict[str, Any]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag != "div":
            return
        node: dict[str, Any] = {"attrs": dict(attrs), "children": []}
        if self.stack:
            self.stack[-1]["children"].append(node)
        self.stack.append(node)

    def handle_endtag(self, tag: str) -> None:
        if tag == "div" and self.stack:
            node = self.stack.pop()
            if "grid" in (node["attrs"].get("class") or "").split():
                self.roots.append(node)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cssom_track_count(value: str, label: str) -> int:
    tokens = re.compile(r"\[[^\]]*\]|(?:\d+(?:\.\d+)?|\.\d+)px")
    cursor = 0
    count = 0
    for match in tokens.finditer(value):
        if value[cursor : match.start()].strip():
            raise SystemExit(f"unsupported Chrome {label} serialization: {value}")
        if match.group().endswith("px"):
            count += 1
        cursor = match.end()
    if value[cursor:].strip() or count == 0:
        raise SystemExit(f"invalid Chrome {label} serialization: {value}")
    return count


def rules(css: str) -> list[tuple[str, str]]:
    return [(selector.strip(), body.strip()) for selector, body in re.findall(r"([^{}]+)\{([^{}]*)\}", css)]


def matches(selector: str, classes: set[str], child_index: int) -> bool:
    if selector == ".item:nth-child(2n)":
        return "item" in classes and child_index % 2 == 0
    if selector.startswith(".") and re.fullmatch(r"\.[A-Za-z0-9_-]+", selector):
        return selector[1:] in classes
    return False


def declarations(
    stylesheets: list[list[tuple[str, str]]], classes: set[str], child_index: int
) -> list[str]:
    result: list[str] = []
    for stylesheet in stylesheets:
        for selector, body in stylesheet:
            if any(matches(part.strip(), classes, child_index) for part in selector.split(",")):
                for declaration in body.split(";"):
                    declaration = declaration.strip()
                    if not declaration:
                        continue
                    property_name, separator, value = declaration.partition(":")
                    if not separator:
                        raise SystemExit(f"invalid pinned CSS declaration: {declaration}")
                    if property_name.strip() == "background":
                        if value.strip() not in {"cyan", "green"}:
                            raise SystemExit(f"unsupported WPT background shorthand: {value.strip()}")
                        property_name = "background-color"
                    result.append(f"{property_name.strip()}: {value.strip()}")
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--wpt-root", type=Path, required=True)
    parser.add_argument("--evidence-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    manifest = json.loads((args.evidence_dir / "source-manifest.json").read_text())
    repeat_expectations_path = args.evidence_dir / "repeat-count-expected.json"
    expected_repeat_sha256 = "b21913bda7c84d3ea804533b1eb58da2eb36dac0cb1f1d4791686871ee0a2cd0"
    if sha256(repeat_expectations_path) != expected_repeat_sha256:
        raise SystemExit("pinned independent repeat-count oracle hash mismatch")
    repeat_expectations = json.loads(repeat_expectations_path.read_text())
    repeat_by_scene = {entry["sceneId"]: entry for entry in repeat_expectations["cases"]}
    if len(repeat_by_scene) != 35:
        raise SystemExit(f"expected 35 repeat-count expectations, found {len(repeat_by_scene)}")
    chrome_observation_path = args.evidence_dir / "chrome-observation.json"
    expected_chrome_observation_sha256 = (
        "0e5d1263138c22788e0f62e522657ac7c025791822ff710ab3eae43b204995d0"
    )
    if sha256(chrome_observation_path) != expected_chrome_observation_sha256:
        raise SystemExit("pinned Chrome used-track observation hash mismatch")
    chrome_observation = json.loads(chrome_observation_path.read_text())
    chrome_roots = chrome_observation.get("roots")
    if not isinstance(chrome_roots, list) or len(chrome_roots) != 35:
        raise SystemExit("expected 35 roots in the pinned Chrome observation")
    chrome_by_index = {root.get("index"): root for root in chrome_roots}
    if len(chrome_by_index) != 35:
        raise SystemExit("Chrome observation root indexes are not unique")
    wpt_root = args.wpt_root
    for entry in manifest["files"]:
        path = wpt_root / entry["path"]
        actual = sha256(path)
        if actual != entry["sha256"]:
            raise SystemExit(f"pinned input hash mismatch: {entry['path']}: {actual}")

    source_rel = "css/css-grid/grid-definition/grid-auto-fit-columns-001.html"
    source_path = wpt_root / source_rel
    html = source_path.read_text()
    page_rules = re.search(r"<style>\s*(.*?)\s*</style>", html, re.S)
    if not page_rules:
        raise SystemExit("inline stylesheet is missing")
    sheets = [
        rules((wpt_root / "css/support/grid.css").read_text()),
        rules(page_rules.group(1)),
    ]
    tree = ElementTree()
    tree.feed(html)
    roots = tree.roots
    if len(roots) != 35:
        raise SystemExit(f"expected 35 grid roots, found {len(roots)}")

    scenes: list[dict[str, Any]] = []
    total_items = 0
    for wpt_index, root in enumerate(roots, start=1):
        root_attrs = root["attrs"]
        root_classes = (root_attrs.get("class") or "").split()
        item_nodes = [child for child in root["children"] if "item" in (child["attrs"].get("class") or "").split()]
        root_style = ";".join(declarations(sheets, set(root_classes), 1))
        items: list[dict[str, Any]] = []
        for item_index, item in enumerate(item_nodes, start=1):
            attrs = item["attrs"]
            item_classes = (attrs.get("class") or "").split()
            expected_keys = ("data-offset-x", "data-offset-y", "data-expected-width", "data-expected-height")
            if any(key not in attrs for key in expected_keys):
                raise SystemExit(f"missing WPT geometry: case {wpt_index}, item {item_index}")
            item_style = declarations(sheets, set(item_classes), item_index)
            inline_style = attrs.get("style")
            if inline_style:
                item_style.extend(part.strip() for part in inline_style.split(";") if part.strip())
            items.append(
                {
                    "id": f"c114-grid-auto-fit-wpt-{wpt_index:02}-item-{item_index}",
                    "classes": item_classes,
                    "style": ";".join(item_style),
                    "authoredStyle": inline_style or "",
                    "expected": {
                        "x": int(attrs["data-offset-x"]),
                        "y": int(attrs["data-offset-y"]),
                        "width": int(attrs["data-expected-width"]),
                        "height": int(attrs["data-expected-height"]),
                    },
                }
            )
        if not items:
            raise SystemExit(f"grid root {wpt_index} has no direct item")
        scene_id = f"grid-auto-fit-columns-{wpt_index:02}"
        repeat = repeat_by_scene.get(scene_id)
        if repeat is None or repeat["rootClass"] != " ".join(root_classes):
            raise SystemExit(f"missing or mismatched independent repeat oracle: {scene_id}")
        chrome_root = chrome_by_index.get(wpt_index)
        if chrome_root is None or chrome_root.get("className") != " ".join(root_classes):
            raise SystemExit(f"missing or mismatched pinned Chrome root: {scene_id}")
        chrome_style = chrome_root.get("style")
        if not isinstance(chrome_style, dict):
            raise SystemExit(f"missing Chrome computed style: {scene_id}")
        columns = chrome_style.get("gridTemplateColumns")
        rows = chrome_style.get("gridTemplateRows")
        if not isinstance(columns, str) or not isinstance(rows, str):
            raise SystemExit(f"missing Chrome used track lists: {scene_id}")
        column_track_count = cssom_track_count(columns, "gridTemplateColumns")
        row_track_count = cssom_track_count(rows, "gridTemplateRows")
        authored_track_count = (
            repeat["prefixTrackCount"]
            + len(repeat["repeatedPatternCountSizesPx"])
            * repeat["expectedRepeatCount"]
            + repeat["suffixTrackCount"]
        )
        if column_track_count < authored_track_count:
            raise SystemExit(f"Chrome used fewer tracks than the authored explicit grid: {scene_id}")
        if re.search(r"(?:^|;)\s*grid-template-rows\s*:", root_style):
            raise SystemExit(f"unexpected authored explicit rows in pinned WPT adapter: {scene_id}")
        used_track_observation = {
            "source": "Chrome 156 CSSOM observation; not a normative expectation",
            "gridTemplateColumns": columns,
            "gridTemplateRows": rows,
            "columnGap": chrome_style.get("columnGap"),
            "rowGap": chrome_style.get("rowGap"),
            "explicitColumnTrackCount": authored_track_count,
            "negativeImplicitColumnTrackCount": 0,
            "positiveImplicitColumnTrackCount": column_track_count - authored_track_count,
            "explicitRowTrackCount": 0,
            "negativeImplicitRowTrackCount": 0,
            "positiveImplicitRowTrackCount": row_track_count,
        }
        width_match = re.search(r"(?:^|;)\s*width:\s*(\d+)px", root_style)
        border_match = re.search(r"(?:^|;)\s*border:\s*(\d+)px\s+solid", root_style)
        padding_match = re.search(r"(?:^|;)\s*padding:\s*0\s+(\d+)px", root_style)
        if not width_match or not border_match:
            raise SystemExit(f"cannot derive root frame from authored CSS: {scene_id}")
        content_width = int(width_match.group(1))
        border = int(border_match.group(1))
        horizontal_padding = 2 * int(padding_match.group(1)) if padding_match else 0
        root_content_height = max(
            item["expected"]["y"] + item["expected"]["height"] for item in items
        )
        expected_root_frame = {
            "x": 0,
            "y": 0,
            "width": content_width + horizontal_padding + 2 * border,
            "height": root_content_height + 2 * border,
        }
        total_items += len(items)
        scenes.append(
            {
                "wptIndex": wpt_index,
                "sceneId": scene_id,
                "rootId": f"c114-grid-auto-fit-wpt-{wpt_index:02}-root",
                "rootClasses": root_classes,
                "rootStyle": root_style,
                "expectedRootFrame": expected_root_frame,
                "repeatExpectation": repeat,
                "usedTrackObservation": used_track_observation,
                "items": items,
            }
        )

    if total_items != 37:
        raise SystemExit(f"expected 37 direct items, found {total_items}")
    output = {
        "source": {
            "wptRevision": manifest["wptRevision"],
            "path": source_rel,
            "sha256": sha256(source_path),
        },
        "caseCount": len(scenes),
        "itemCount": total_items,
        "scenes": scenes,
    }
    args.output.write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n")


if __name__ == "__main__":
    main()
