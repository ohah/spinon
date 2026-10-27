#!/usr/bin/env python3
"""Extract every grid/item geometry assertion from pinned WPT source."""

from __future__ import annotations

import hashlib
import json
import sys
from html.parser import HTMLParser
from pathlib import Path


DEFAULT_SOURCE = Path(
    "build/wpt-c124-source/css/css-grid/grid-definition/"
    "grid-auto-fit-columns-001.html"
)
WPT_REVISION = "c999c58338ee1d223df5ad62e48be1202ced0d37"
EXPECTED_SOURCE_SHA256 = (
    "11410caa92c400b653ac781ef7f1103c871cebb108c52b75f2e8308b2531ba63"
)
EXPECTED_FIELDS = {
    "data-offset-x": "offsetLeftPx",
    "data-offset-y": "offsetTopPx",
    "data-expected-width": "offsetWidthPx",
    "data-expected-height": "offsetHeightPx",
}


class GridCaseParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.stack: list[tuple[str, dict]] = []
        self.cases: list[dict] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag != "div":
            return
        values = {key: value or "" for key, value in attrs}
        classes = values.get("class", "").split()
        if "grid" in classes:
            case = {
                "wptIndex": len(self.cases) + 1,
                "sceneId": f"grid-auto-fit-columns-{len(self.cases) + 1:02d}",
                "rootId": values.get("id", ""),
                "rootClass": values.get("class", ""),
                "items": [],
            }
            self.cases.append(case)
            self.stack.append(("root", case))
            return
        if self.stack and self.stack[-1][0] == "root" and "item" in classes:
            case = self.stack[-1][1]
            expected = {}
            for source_name, target_name in EXPECTED_FIELDS.items():
                if source_name not in values:
                    raise ValueError(
                        f"case {case['wptIndex']} item missing {source_name}"
                    )
                expected[target_name] = float(values[source_name])
            case["items"].append(
                {
                    "itemIndex": len(case["items"]) + 1,
                    "class": values.get("class", ""),
                    "style": values.get("style", ""),
                    "expected": expected,
                }
            )
            self.stack.append(("item", case))
            return
        self.stack.append(("other", {}))

    def handle_endtag(self, tag: str) -> None:
        if tag == "div" and self.stack:
            self.stack.pop()


def main() -> None:
    source = Path(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT_SOURCE
    source_bytes = source.read_bytes()
    source_hash = hashlib.sha256(source_bytes).hexdigest()
    if source_hash != EXPECTED_SOURCE_SHA256:
        raise SystemExit(
            f"pinned WPT source hash mismatch: expected {EXPECTED_SOURCE_SHA256}, "
            f"found {source_hash}"
        )
    parser = GridCaseParser()
    parser.feed(source_bytes.decode("utf-8"))
    if len(parser.cases) != 35:
        raise SystemExit(f"expected 35 WPT roots, found {len(parser.cases)}")
    item_count = sum(len(case["items"]) for case in parser.cases)
    if item_count != 37:
        raise SystemExit(f"expected 37 WPT items, found {item_count}")
    for case in parser.cases:
        if not case["items"]:
            raise SystemExit(f"case {case['wptIndex']} has no direct item")
    result = {
        "source": {
            "wptRevision": WPT_REVISION,
            "path": "css/css-grid/grid-definition/grid-auto-fit-columns-001.html",
            "sha256": source_hash,
        },
        "caseCount": len(parser.cases),
        "itemCount": item_count,
        "assertions": parser.cases,
    }
    print(json.dumps(result, indent=2, ensure_ascii=False) + "\n", end="")


if __name__ == "__main__":
    main()
