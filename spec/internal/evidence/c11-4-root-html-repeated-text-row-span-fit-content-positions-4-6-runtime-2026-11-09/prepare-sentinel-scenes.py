#!/usr/bin/env python3
"""Materialize six exact Chrome-capture scenes for default-route simulator smoke."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
CAPTURE = (
    ROOT
    / "spec/internal/evidence/c11-4-root-html-repeated-text-row-span-fit-content-positions-4-6-precomparison-2026-11-09/chrome157-capture.json"
)
TEXT = {
    "A": "A A A A A A A A A A A A A A A A",
    "B": "A A A A A A A A A A A A A A",
    "C": "A A A A A A A A A A A A",
    "D": "A A A A A A A A A A",
    "E": "A A A A A A A",
}


def html_for(scene: dict) -> str:
    tracks = " ".join(scene["trackList"])
    align = scene["align"]
    children = []
    for item in scene["items"]:
        item_id = item["id"]
        if item_id == "empty-0":
            children.append(
                '<div class="item empty" style="grid-row:1 / 2;grid-column:1 / 2"></div>'
            )
        else:
            text = TEXT[item_id]
            children.append(
                f'<div class="item" data-item="{item_id}" '
                f'style="grid-row:{item["rowStart"]} / {item["rowEnd"]}">'
                f'<div class="payload">{text}</div></div>'
            )
    style = f"""
* {{ box-sizing: border-box; }}
html, body {{ margin: 0; padding: 0; }}
#grid {{
  display: grid; width: 40px; height: 220px;
  grid-template-columns: 40px; grid-template-rows: 10px;
  grid-auto-rows: {tracks}; grid-auto-flow: row;
  row-gap: 0; column-gap: 0; align-content: {align};
  align-items: normal; justify-content: start; justify-items: normal;
  direction: ltr; overflow: visible; margin: 0; padding: 0;
}}
.item {{ display: block; width: auto; height: auto; min-height: 0; margin: 0; padding: 0; overflow: visible; grid-column: 1 / 2; }}
.empty {{ height: 10px; background-color: rgb(11, 17, 23); }}
.payload {{ display: block; width: auto; height: auto; margin: 0; padding: 0; font-family: monospace; font-size: 10px; font-weight: 400; font-style: normal; line-height: 10px; letter-spacing: 0; word-spacing: 0; white-space: normal; direction: ltr; }}
.item[data-item="A"] {{ background-color: rgb(31, 41, 51); }}
.item[data-item="B"] {{ background-color: rgb(61, 71, 81); }}
.item[data-item="C"] {{ background-color: rgb(91, 101, 111); }}
.item[data-item="D"] {{ background-color: rgb(101, 111, 121); }}
.item[data-item="E"] {{ background-color: rgb(111, 121, 131); }}
.item[data-item="A"] .payload {{ background-color: rgb(121, 131, 141); }}
.item[data-item="B"] .payload {{ background-color: rgb(151, 161, 171); }}
.item[data-item="C"] .payload {{ background-color: rgb(181, 191, 201); }}
.item[data-item="D"] .payload {{ background-color: rgb(191, 201, 211); }}
.item[data-item="E"] .payload {{ background-color: rgb(201, 211, 221); }}
"""
    return (
        '<!doctype html><html lang="en"><head><meta charset="utf-8">'
        '<meta name="viewport" content="width=device-width, initial-scale=1">'
        f"<style>{style}</style></head><body><div id=\"grid\">"
        f"{' '.join(children)}</div></body></html>"
    )


def main() -> None:
    raw = json.loads(CAPTURE.read_text())
    capture = json.loads(raw) if isinstance(raw, str) else raw
    selected = [
        scene
        for scene in capture["cases"]
        if scene["fitPosition"] in (4, 5, 6)
        and scene["align"] in ("start", "stretch")
        and scene["cap"] == "10px"
        and scene["topology"] == "span2-span3-six-forward"
        and scene["tail"]
        == ["auto", "max-content", "max-content", "min-content", "min-content"]
    ]
    selected.sort(key=lambda scene: (scene["fitPosition"], scene["align"]))
    if len(selected) != 6:
        raise SystemExit(f"expected 6 sentinel scenes, got {len(selected)}")

    sources = EVIDENCE / "sources"
    sources.mkdir(parents=True, exist_ok=True)
    (sources / "root-app.js").write_text(
        'globalThis.__spinonRootHtmlDefaultRouteSmoke = "c11-4-row-span";\n'
    )
    manifest = []
    for ordinal, scene in enumerate(selected, start=1):
        name = f"fit-{scene['fitPosition']}-{scene['align']}"
        html = html_for(scene).replace(
            "</body>",
            f"<!-- SPINON_C11_4_SMOKE={name};{'x' * ordinal} --></body>",
        )
        source_path = sources / f"{name}.html"
        source_path.write_text(html)
        manifest.append(
            {
                "name": name,
                "source": source_path.name,
                "bytes": len(html.encode()),
                "sha256": hashlib.sha256(html.encode()).hexdigest(),
                "trackList": scene["trackList"],
                "rows": scene["rows"],
                "items": scene["items"],
                "topology": scene["topology"],
                "cap": scene["cap"],
                "align": scene["align"],
            }
        )
    (EVIDENCE / "sentinel-scenes.json").write_text(
        json.dumps(selected, ensure_ascii=False, indent=2) + "\n"
    )
    (EVIDENCE / "scene-manifest.json").write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n"
    )
    print(f"wrote {len(selected)} scenes to {sources}")


if __name__ == "__main__":
    main()
