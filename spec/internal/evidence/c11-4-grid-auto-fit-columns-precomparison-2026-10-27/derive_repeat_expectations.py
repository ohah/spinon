#!/usr/bin/env python3
"""Derive auto-repeat counts from the pinned WPT CSS inputs and Grid §7.2.3.2."""

from __future__ import annotations

import hashlib
import json
import re
from fractions import Fraction
from pathlib import Path


EVIDENCE = Path(__file__).parent
EXPECTED = EVIDENCE / "expected.json"
REPO_ROOT = EVIDENCE.parents[3]
WPT_SOURCE = REPO_ROOT / "build/wpt-c124-source/css/css-grid/grid-definition/grid-auto-fit-columns-001.html"
WPT_GRID_CSS = REPO_ROOT / "build/wpt-c124-source/css/support/grid.css"
BASIS_PX = Fraction(200)
REVISION = "c999c58338ee1d223df5ad62e48be1202ced0d37"
SOURCE_SHA256 = "11410caa92c400b653ac781ef7f1103c871cebb108c52b75f2e8308b2531ba63"
GRID_CSS_SHA256 = "b2f35862e7b8fa9dcae8f4f8d25add78d93f3e7993e08a4337030a4dd2d4425e"

AUTHORED_TRACK_LISTS = {
    "gridOnlyAutoRepeat": "repeat(auto-fit, 30px [autobar])",
    "gridPercentAutoRepeat": "repeat(auto-fit, 30% [autobar])",
    "gridMinMaxAutoRepeat": "repeat(auto-fill, minmax(13px, 30px))",
    "gridMaxMinAutoRepeat": "repeat(auto-fill, minmax(30px, 13px))",
    "gridMaxMinPercentAutoRepeat": "repeat(auto-fill, minmax(30px, 6.5%))",
    "gridAutoRepeatAndFixedBefore": "10px [foo] 20% [bar] repeat(auto-fit, [autofoo] 35px)",
    "gridAutoRepeatAndFixedAfter": "repeat(auto-fit, [first] 30px [last]) [foo] minmax(60px, 80px) [bar] minmax(45px, max-content)",
    "gridAutoRepeatAndFixed": "[start] repeat(2, 50px [a]) [middle] repeat(auto-fit, [autofoo] 15px [autobar]) minmax(5%, 10%) [end]",
    "gridMultipleNames": "[start] 20px [foo] 50% repeat(auto-fit, [bar] 20px [start foo]) [foo] 10% [end bar]",
    "gridMultipleTracks": "[start] 20px repeat(auto-fit, [a] 2em [b c] 10% [d]) [e] minmax(75px, 1fr) [last]",
    "gridMinMaxFixedFlex": "repeat(auto-fit, minmax(50px, 1fr))",
}

# The tuple values are resolved CSS Grid count sizes, not Chrome observations.
# Percentage values are resolved against the 200px content box; `minmax` values
# use the definite max when available, floored by the min, otherwise the min.
PROFILES = {
    "gridOnlyAutoRepeat": {
        "repeat": "auto-fit",
        "prefix": [],
        "pattern": [30],
        "suffix": [],
    },
    "gridPercentAutoRepeat": {
        "repeat": "auto-fit",
        "prefix": [],
        "pattern": [60],  # 30% of the definite 200px content box
        "suffix": [],
    },
    "gridMinMaxAutoRepeat": {
        "repeat": "auto-fill",
        "prefix": [],
        "pattern": [30],  # max(13px min, 30px max)
        "suffix": [],
    },
    "gridMaxMinAutoRepeat": {
        "repeat": "auto-fill",
        "prefix": [],
        "pattern": [30],  # max(30px min, 13px max)
        "suffix": [],
    },
    "gridMaxMinPercentAutoRepeat": {
        "repeat": "auto-fill",
        "prefix": [],
        "pattern": [30],  # max(30px min, 6.5%/13px max)
        "suffix": [],
    },
    "gridAutoRepeatAndFixedBefore": {
        "repeat": "auto-fit",
        "prefix": [10, 40],  # 10px + 20% of 200px
        "pattern": [35],
        "suffix": [],
    },
    "gridAutoRepeatAndFixedAfter": {
        "repeat": "auto-fit",
        "prefix": [],
        "pattern": [30],
        "suffix": [80, 45],  # minmax(60px,80px), minmax(45px,max-content)
    },
    "gridAutoRepeatAndFixed": {
        "repeat": "auto-fit",
        "prefix": [50, 50],
        "pattern": [15],
        "suffix": [20],  # 10% of 200px
    },
    "gridMultipleNames": {
        "repeat": "auto-fit",
        "prefix": [20, 100],  # 20px + 50% of 200px
        "pattern": [20],
        "suffix": [20],  # 10% of 200px
    },
    "gridMultipleTracks": {
        "repeat": "auto-fit",
        "prefix": [20],
        "pattern": [32, 20],  # 2em at 16px + 10% of 200px
        "suffix": [75],  # minmax(75px,1fr): flexible max uses its definite min
    },
    "gridMinMaxFixedFlex": {
        "repeat": "auto-fit",
        "prefix": [],
        "pattern": [50],  # minmax(50px,1fr): flexible max uses its definite min
        "suffix": [],
    },
}


def required_size(profile: dict, repeats: int, gap: int) -> Fraction:
    track_count = (
        len(profile["prefix"])
        + len(profile["suffix"])
        + len(profile["pattern"]) * repeats
    )
    return (
        sum(map(Fraction, profile["prefix"]), Fraction(0))
        + sum(map(Fraction, profile["suffix"]), Fraction(0))
        + repeats * sum(map(Fraction, profile["pattern"]), Fraction(0))
        + max(0, track_count - 1) * gap
    )


def rational(value: Fraction) -> str:
    return str(value.numerator) if value.denominator == 1 else f"{value.numerator}/{value.denominator}"


def css_rule_declarations(source: str, selector: str) -> dict[str, str]:
    match = re.search(
        rf"(?m)^\s*{re.escape(selector)}\s*\{{([^{{}}]*)\}}",
        source,
    )
    if match is None:
        raise SystemExit(f"pinned WPT source is missing CSS rule {selector!r}")
    declarations = {}
    for declaration in match.group(1).split(";"):
        if not declaration.strip():
            continue
        name, separator, value = declaration.partition(":")
        if not separator:
            raise SystemExit(f"invalid declaration in {selector!r}: {declaration!r}")
        declarations[name.strip()] = " ".join(value.split())
    return declarations


def validate_source_inputs() -> tuple[str, str]:
    source_bytes = WPT_SOURCE.read_bytes()
    source = source_bytes.decode("utf-8")
    actual_hash = hashlib.sha256(source_bytes).hexdigest()
    if actual_hash != SOURCE_SHA256:
        raise SystemExit(
            f"pinned WPT source hash mismatch: expected {SOURCE_SHA256}, found {actual_hash}"
        )
    base = css_rule_declarations(source, ".grid")
    if base.get("width") != "200px" or base.get("grid-auto-columns") != "157px" or base.get("grid-auto-rows") != "25px":
        raise SystemExit(f"pinned WPT grid sizing inputs changed: {base}")
    gap = css_rule_declarations(source, ".gap")
    if gap.get("grid-column-gap") != "20px":
        raise SystemExit(f"pinned WPT column gap input changed: {gap}")
    if base.get("border") != "2px solid magenta" or base.get("justify-content") != "start":
        raise SystemExit(f"pinned WPT grid border/alignment inputs changed: {base}")
    for profile_name, track_list in AUTHORED_TRACK_LISTS.items():
        declarations = css_rule_declarations(source, f".{profile_name}")
        if declarations.get("grid-template-columns") != track_list:
            raise SystemExit(
                f"pinned WPT track list changed for {profile_name}: "
                f"expected {track_list!r}, found {declarations.get('grid-template-columns')!r}"
            )
    padded = css_rule_declarations(source, ".gridPercentAutoRepeat")
    if padded.get("padding") != "0 13px":
        raise SystemExit(f"pinned WPT percent-repeat padding changed: {padded}")
    grid_css_hash = hashlib.sha256(WPT_GRID_CSS.read_bytes()).hexdigest()
    if grid_css_hash != GRID_CSS_SHA256:
        raise SystemExit(
            f"pinned WPT grid helper hash mismatch: expected {GRID_CSS_SHA256}, "
            f"found {grid_css_hash}"
        )
    if css_rule_declarations(WPT_GRID_CSS.read_text(encoding="utf-8"), ".grid").get("display") != "grid":
        raise SystemExit("pinned WPT helper no longer defines .grid as display:grid")
    return actual_hash, grid_css_hash


def main() -> None:
    source_hash, grid_css_hash = validate_source_inputs()
    wpt = json.loads(EXPECTED.read_text())
    if wpt["source"]["wptRevision"] != REVISION or wpt["source"]["sha256"] != source_hash:
        raise SystemExit("expected.json does not match the pinned WPT source")
    cases = []
    for case in wpt["assertions"]:
        classes = case["rootClass"].split()
        profile_name = next((name for name in PROFILES if name in classes), None)
        if profile_name is None:
            raise SystemExit(f"no count profile for case {case['wptIndex']}: {classes}")
        profile = PROFILES[profile_name]
        gap = 20 if "gap" in classes else 0
        required_for_one = required_size(profile, 1, gap)
        if required_for_one > BASIS_PX:
            count = 1
        else:
            count = 1
            while required_size(profile, count + 1, gap) <= BASIS_PX:
                count += 1
        next_size = required_size(profile, count + 1, gap)
        cases.append(
            {
                "wptIndex": case["wptIndex"],
                "sceneId": case["sceneId"],
                "rootClass": case["rootClass"],
                "authoredGridTemplateColumns": AUTHORED_TRACK_LISTS[profile_name],
                "axis": "columns",
                "repeatMode": profile["repeat"],
                "definiteContentBasisPx": rational(BASIS_PX),
                "prefixTrackCount": len(profile["prefix"]),
                "prefixCountSizesPx": list(map(str, profile["prefix"])),
                "repeatedPatternCountSizesPx": list(map(str, profile["pattern"])),
                "suffixTrackCount": len(profile["suffix"]),
                "suffixCountSizesPx": list(map(str, profile["suffix"])),
                "columnGapPx": gap,
                "requiredSizeAtSelectedCountPx": rational(required_size(profile, count, gap)),
                "requiredSizeAtNextCountPx": rational(next_size),
                "expectedRepeatCount": count,
                "forcedOneWhenNoPositiveCountFits": required_for_one > BASIS_PX,
            }
        )
    if len(cases) != 35:
        raise SystemExit(f"expected 35 cases, found {len(cases)}")
    result = {
        "sourceWptRevision": REVISION,
        "sourceGeometrySha256": source_hash,
        "sourceCssSha256": grid_css_hash,
        "normativeReference": "CSS Grid Layout Module Level 1 2025 CRD §7.2.3.2",
        "method": "Use each definite repeated track max sizing function, or its definite min when the max is indefinite; floor a definite max by the min; include fixed prefix/suffix tracks and every inter-track gap. Choose the largest positive count that fits the definite 200px grid-container content box, falling back to one if no positive count fits.",
        "percentRepeatNote": "Case 2 has a definite 200px content box and 13px inline padding on each side. Padding shifts the grid content box origin and enlarges the border box; it does not subtract from the declared 200px content-box width. Thus 30% resolves to 60px, three repeats fit, and the source-authored full-span item is 180px wide.",
        "emResolutionNote": "Case 32 uses 2em; its non-collapsed repeated track is 32px in the pinned Chrome CSSOM observation, fixing the 16px inherited WPT font-size basis for this fixture. The corresponding case 33 track collapses under auto-fit and is not used to infer the em basis.",
        "cases": cases,
    }
    print(json.dumps(result, indent=2, ensure_ascii=False) + "\n", end="")


if __name__ == "__main__":
    main()
