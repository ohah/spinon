#!/bin/sh
set -eu

mkdir -p build/spinon/r05-presentation-ledger
swiftc \
  platforms/ios/Sources/R05PresentationLedger.swift \
  spec/internal/evidence/r05-presentation-signal-probe-2026-10-08/ios-layer-smoke/R05PresentationLedgerSelfTest.swift \
  -o build/spinon/r05-presentation-ledger/R05PresentationLedgerSelfTest

run=1
while [ "$run" -le 20 ]; do
  output="build/spinon/r05-presentation-ledger/round-${run}.log"
  if ! build/spinon/r05-presentation-ledger/R05PresentationLedgerSelfTest > "$output" 2>&1; then
    cat "$output"
    exit 1
  fi
  if ! rg -q '^R05 callback ledger 자체 시험 통과: 6개 그룹$' "$output"; then
    cat "$output"
    exit 1
  fi
  printf '통과 %02d/20\n' "$run"
  run=$((run + 1))
done
