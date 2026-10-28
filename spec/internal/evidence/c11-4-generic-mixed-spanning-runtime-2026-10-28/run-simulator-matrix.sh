#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
evidence_root="spec/internal/evidence/c11-4-generic-mixed-spanning-runtime-2026-10-28"
capture_script="$repo_root/$evidence_root/capture-generic-spanning.mjs"
scenes="mixed-text-image-span2-forward,mixed-text-image-span2-reversed,mixed-text-image-span2-before-span3,mixed-text-image-span3-before-span2"

export SPINON_C114_EVIDENCE_ROOT="$evidence_root"
export SPINON_C114_SCREENSHOT_SCENES="mixed-text-image-span2-forward,mixed-text-image-span2-before-span3"

mise exec -- node "$capture_script" android-avd emulator-5554 "$scenes"
mise exec -- node "$capture_script" ios-simulator 9DCCE836-7659-4914-B6C1-13F54BFFA9DB "$scenes"
