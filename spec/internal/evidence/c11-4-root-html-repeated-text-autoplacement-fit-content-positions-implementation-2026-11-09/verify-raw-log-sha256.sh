#!/usr/bin/env bash
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
target_directory="${1:-$script_directory}"
manifest="$target_directory/raw-log-sha256.txt"

while read -r expected_hash archive_name; do
  [[ -n "$expected_hash" && -n "$archive_name" ]] || continue
  actual_hash="$(gzip -dc "$target_directory/$archive_name" | shasum -a 256 | awk '{print $1}')"
  if [[ "$actual_hash" != "$expected_hash" ]]; then
    printf 'raw log hash mismatch: %s\n' "$archive_name" >&2
    exit 1
  fi
  printf 'verified raw log: %s\n' "$archive_name"
done < "$manifest"
