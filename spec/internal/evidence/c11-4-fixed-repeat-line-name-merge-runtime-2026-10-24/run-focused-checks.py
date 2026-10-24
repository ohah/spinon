#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = Path(__file__).resolve().parent
CHECKS = [
    (
        "style-fixed-repeat",
        ["cargo", "test", "--locked", "-p", "spinon-style", "fixed_repeat", "--", "--nocapture"],
    ),
    (
        "layout-fixed-repeat-line-names",
        [
            "cargo",
            "test",
            "--locked",
            "-p",
            "spinon-style-to-layout",
            "c11_4_track_sizing_areas_tests::fixed_repeat_line_names",
            "--",
            "--nocapture",
        ],
    ),
    (
        "ffi-runtime-scenes",
        [
            "cargo",
            "test",
            "--locked",
            "-p",
            "spinon-ffi",
            "--features",
            "c04-runtime-gpu-fixture-test",
            "c11_4_auto_repeat_named_lines::tests::exposes_all_fixed_named_auto_repeat_runtime_scenes",
            "--",
            "--nocapture",
        ],
    ),
    (
        "fixture-module-boundary",
        ["node", "--test", "tools/css-reference/fixture-module-boundary.test.mjs"],
    ),
    ("format", ["cargo", "fmt", "--all", "--", "--check"]),
    (
        "workspace-clippy",
        ["cargo", "clippy", "--locked", "--workspace", "--all-targets", "--", "-D", "warnings"],
    ),
    ("staged-diff-check", ["git", "diff", "--cached", "--check"]),
    ("worktree-diff-check", ["git", "diff", "--check"]),
]

results = []
for name, command in CHECKS:
    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, check=False)
    output = result.stdout + result.stderr
    (EVIDENCE / f"{name}.log").write_text(output)
    results.append(
        {
            "name": name,
            "command": command,
            "exitCode": result.returncode,
            "passed": result.returncode == 0,
            "summary": [
                line.strip()
                for line in output.splitlines()
                if line.strip().startswith("test result:")
                or line.strip().startswith("ℹ tests ")
                or "Finished `test` profile" in line
            ],
        }
    )
    if result.returncode != 0:
        break

report = {"schemaVersion": 1, "results": results, "allPassed": len(results) == len(CHECKS) and all(x["passed"] for x in results)}
(EVIDENCE / "focused-checks.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps(report, indent=2))
if not report["allPassed"]:
    raise SystemExit(1)
