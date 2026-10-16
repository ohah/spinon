#!/usr/bin/env python3
"""Generate the fixed C18.4 async image replacement PNG inputs."""

from pathlib import Path
import struct
import zlib


ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/fixtures/resources"


def png(width: int, height: int, rgba: tuple[int, int, int, int]) -> bytes:
    def chunk(kind: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + kind
            + data
            + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
        )

    rows = b"".join(b"\x00" + bytes(rgba) * width for _ in range(height))
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(rows))
        + chunk(b"IEND", b"")
    )


IMAGES = {
    "c18-4-async-a-slow.png": (2, 2, (240, 24, 32, 255)),
    "c18-4-async-b-fast.png": (1, 4, (24, 64, 240, 255)),
    "c18-4-async-c-slow.png": (4, 2, (32, 220, 80, 255)),
    "c18-4-async-d-fast.png": (2, 1, (200, 40, 220, 255)),
}


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for name, (width, height, rgba) in IMAGES.items():
        (OUTPUT / name).write_bytes(png(width, height, rgba))


if __name__ == "__main__":
    main()
