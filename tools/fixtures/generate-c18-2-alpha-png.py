#!/usr/bin/env python3
"""C18.2 고정 alpha PNG fixture를 결정적으로 생성한다."""

from pathlib import Path
import struct
import zlib


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/fixtures/resources/c18-2-alpha-source-over.png"
WIDTH = 24
HEIGHT = 16
BANDS = (
    (250, 20, 210, 0),
    (220, 40, 80, 128),
    (32, 200, 64, 255),
)


def chunk(kind: bytes, data: bytes) -> bytes:
    payload = kind + data
    return struct.pack(">I", len(data)) + payload + struct.pack(">I", zlib.crc32(payload) & 0xFFFFFFFF)


rows = bytearray()
for _ in range(HEIGHT):
    rows.append(0)
    for x in range(WIDTH):
        rows.extend(BANDS[x // 8])

png = bytearray(b"\x89PNG\r\n\x1a\n")
png.extend(chunk(b"IHDR", struct.pack(">IIBBBBB", WIDTH, HEIGHT, 8, 6, 0, 0, 0)))
png.extend(chunk(b"IDAT", zlib.compress(bytes(rows), level=9)))
png.extend(chunk(b"IEND", b""))
OUTPUT.write_bytes(png)
print(f"{OUTPUT.relative_to(ROOT)} {len(png)} bytes")
