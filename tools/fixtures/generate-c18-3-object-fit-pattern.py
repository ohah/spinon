#!/usr/bin/env python3
"""C18.3 object-fit/object-position 픽셀 기준 PNG를 결정적으로 생성한다."""

from pathlib import Path
import struct
import zlib


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/fixtures/resources/c18-3-object-fit-pattern.png"
WIDTH = 24
HEIGHT = 12
CELL = 4


def chunk(kind: bytes, data: bytes) -> bytes:
    payload = kind + data
    checksum = zlib.crc32(payload) & 0xFFFFFFFF
    return struct.pack(">I", len(data)) + payload + struct.pack(">I", checksum)


rows = bytearray()
for y in range(HEIGHT):
    rows.append(0)
    for x in range(WIDTH):
        column = x // CELL
        row = y // CELL
        rows.extend((24 + column * 37, 32 + row * 91, 232 - column * 31 - row * 13, 255))

png = bytearray(b"\x89PNG\r\n\x1a\n")
png.extend(chunk(b"IHDR", struct.pack(">IIBBBBB", WIDTH, HEIGHT, 8, 6, 0, 0, 0)))
png.extend(chunk(b"IDAT", zlib.compress(bytes(rows), level=9)))
png.extend(chunk(b"IEND", b""))
OUTPUT.write_bytes(png)
print(f"{OUTPUT.relative_to(ROOT)} {len(png)} bytes")
