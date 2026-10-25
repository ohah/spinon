#!/usr/bin/env python3
"""Package raw simulator logs as gzip while preserving and verifying their bytes."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
from pathlib import Path


OUT = Path(__file__).resolve().parent
LOG_DIRS = (OUT / "android", OUT / "ios-simulator")
MANIFEST = OUT / "compressed-log-manifest.json"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def package() -> None:
    records = []
    for directory in LOG_DIRS:
        for source in sorted(directory.glob("*.log")):
            raw = source.read_bytes()
            compressed = gzip.compress(raw, compresslevel=9, mtime=0)
            target = source.with_suffix(source.suffix + ".gz")
            target.write_bytes(compressed)
            records.append({
                "path": str(target.relative_to(OUT)),
                "rawBytes": len(raw),
                "rawSha256": sha256(raw),
                "gzipBytes": len(compressed),
                "gzipSha256": sha256(compressed),
            })
    if not records:
        raise RuntimeError("no raw simulator .log files found")
    MANIFEST.write_text(json.dumps({
        "format": "spinon-c11-4-compressed-logs-v1",
        "fileCount": len(records),
        "files": records,
    }, indent=2) + "\n", encoding="utf-8")
    print(f"packaged {len(records)} logs; raw={sum(x['rawBytes'] for x in records)} bytes; "
          f"gzip={sum(x['gzipBytes'] for x in records)} bytes")


def verify() -> None:
    data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    if data.get("format") != "spinon-c11-4-compressed-logs-v1":
        raise RuntimeError("unknown compressed log manifest format")
    records = data["files"]
    if len(records) != data.get("fileCount"):
        raise RuntimeError("manifest file count does not match")
    for record in records:
        path = OUT / record["path"]
        compressed = path.read_bytes()
        if len(compressed) != record["gzipBytes"] or sha256(compressed) != record["gzipSha256"]:
            raise RuntimeError(f"compressed bytes changed: {record['path']}")
        raw = gzip.decompress(compressed)
        if len(raw) != record["rawBytes"] or sha256(raw) != record["rawSha256"]:
            raise RuntimeError(f"decompressed bytes changed: {record['path']}")
    print(f"verified {len(records)} compressed logs and their original byte hashes")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--verify", action="store_true", help="verify compressed and original hashes")
    args = parser.parse_args()
    verify() if args.verify else package()


if __name__ == "__main__":
    main()
