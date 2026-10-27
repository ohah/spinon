#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="spec/internal/evidence/c11-4-generic-mixed-row-axis-runtime-2026-10-28"
capture_script="$repo_root/spec/internal/evidence/c11-4-generic-mixed-row-axis-runtime-2026-10-28/capture-generic-row-axis.mjs"
scenes="direct-min-content-forward,direct-min-content-reversed,direct-max-content-forward,direct-max-content-reversed,direct-row-min-content-forward,direct-row-min-content-reversed,direct-row-max-content-forward,direct-row-max-content-reversed,direct-min-content-generic-forward,direct-min-content-generic-reversed,direct-max-content-generic-forward,direct-max-content-generic-reversed,direct-row-min-content-generic-forward,direct-row-min-content-generic-reversed,direct-row-max-content-generic-forward,direct-row-max-content-generic-reversed"

export SPINON_C114_EVIDENCE_ROOT="$evidence_root"
export SPINON_C114_SCREENSHOT_SCENES="direct-row-min-content-generic-forward,direct-row-max-content-generic-reversed"

mise exec -- node "$capture_script" android-avd emulator-5554 "$scenes"
mise exec -- node "$capture_script" ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB "$scenes"
