#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
result_dir="${SPINON_R06_FAIRNESS_SIMULATORS_OUTPUT_DIR:-$repo_root/build/spinon/priority-fairness-simulators/$run_id}"

fail() {
  printf '실패: %s\n' "$1" >&2
  exit 1
}

[[ "$(uname -s)" == "Darwin" ]] || fail "이 검증은 Android 에뮬레이터와 iOS 시뮬레이터가 있는 macOS에서 실행해야 합니다"
command -v bash >/dev/null 2>&1 || fail "bash를 찾을 수 없습니다"
command -v rg >/dev/null 2>&1 || fail "ripgrep의 rg를 찾을 수 없습니다"
[[ ! -e "$result_dir" && ! -L "$result_dir" ]] || fail "결과 폴더가 이미 있습니다: $result_dir"
mkdir -p "$result_dir"

for attempt in 1 2 3 4 5; do
  run_dir="$result_dir/run-$attempt"
  console_log="$result_dir/run-$attempt.console.log"
  printf 'Android·iOS 실제 V8 우선순위 검증 %s/5 실행\n' "$attempt"
  if ! SPINON_R06_PRIORITY_OUTPUT_DIR="$run_dir" \
    bash "$repo_root/tools/verify-r06-priority-simulators.sh" \
    > "$console_log" 2>&1; then
    cat "$console_log" >&2
    fail "시뮬레이터 검증 $attempt/5가 실패했습니다. 결과: $run_dir"
  fi
  cat "$console_log"
done

printf 'Android 에뮬레이터와 iOS 시뮬레이터의 실제 V8 검증을 각각 5회 통과했습니다.\n결과 폴더: %s\n' "$result_dir"
