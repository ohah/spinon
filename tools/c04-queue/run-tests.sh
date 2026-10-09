#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
output_dir="$repo_root/build/spinon/tests/c04-queue"
java_output="$output_dir/java"
swift_output="$output_dir/swift-tests"
mkdir -p "$java_output"

if command -v mise >/dev/null 2>&1; then
  mise exec -- javac -d "$java_output" \
    "$repo_root/platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410LatestTaskLane.java" \
    "$repo_root/tools/c04-queue/java/C0410LatestTaskLaneTest.java"
  mise exec -- java -cp "$java_output" dev.spinon.bootstrap.C0410LatestTaskLaneTest
  mise exec -- swiftc \
    "$repo_root/platforms/ios/Sources/C0410LatestTaskLane.swift" \
    "$repo_root/tools/c04-queue/swift/C0410LatestTaskLaneTest.swift" \
    -o "$swift_output"
  mise exec -- "$swift_output"
else
  javac -d "$java_output" \
    "$repo_root/platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410LatestTaskLane.java" \
    "$repo_root/tools/c04-queue/java/C0410LatestTaskLaneTest.java"
  java -cp "$java_output" dev.spinon.bootstrap.C0410LatestTaskLaneTest
  swiftc \
    "$repo_root/platforms/ios/Sources/C0410LatestTaskLane.swift" \
    "$repo_root/tools/c04-queue/swift/C0410LatestTaskLaneTest.swift" \
    -o "$swift_output"
  "$swift_output"
fi
