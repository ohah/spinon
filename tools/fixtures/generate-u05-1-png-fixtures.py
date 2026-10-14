#!/usr/bin/env python3
"""Generate deterministic U05.1 24x16 RGBA PNG fixtures."""

import base64
import struct
import zlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/fixtures/resources"


def chunk(kind: bytes, data: bytes) -> bytes:
    checksum = zlib.crc32(kind + data) & 0xFFFFFFFF
    return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", checksum)


def rgba(x: int, y: int) -> bytes:
    return bytes((255 if x < 12 else 0, 255 if y < 8 else 0, 64, 255))


def encode(interlaced: bool) -> bytes:
    width, height = 24, 16
    rows = []
    if interlaced:
        passes = ((0, 0, 8, 8), (4, 0, 8, 8), (0, 4, 4, 8), (2, 0, 4, 4),
                  (0, 2, 2, 4), (1, 0, 2, 2), (0, 1, 1, 2))
        for x_start, y_start, x_step, y_step in passes:
            for y in range(y_start, height, y_step):
                rows.append(b"\0" + b"".join(rgba(x, y) for x in range(x_start, width, x_step)))
    else:
        for y in range(height):
            rows.append(b"\0" + b"".join(rgba(x, y) for x in range(width)))
    header = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, int(interlaced))
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(b"".join(rows), 9)) + chunk(b"IEND", b"")


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    basic = encode(False)
    (OUTPUT / "c14-1-png-basic.png").write_bytes(basic)
    (OUTPUT / "c14-1-png-adam7.png").write_bytes(encode(True))
    data = base64.b64encode(basic).decode("ascii")
    page = (
        "<!doctype html>\n<meta charset=\"utf-8\">\n"
        "<style>body{margin:0} img{display:block;width:auto;height:auto}</style>\n"
        f"<img id=\"image\" alt=\"fixture\" src=\"data:image/png;base64,{data}\">\n"
    )
    (OUTPUT / "c14-1-png-basic.html").write_text(page, encoding="utf-8")


if __name__ == "__main__":
    main()
