import assert from 'node:assert/strict';
import { inflateSync } from 'node:zlib';

const SIGNATURE = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
const CRC_TABLE = new Uint32Array(256);
for (let value = 0; value < CRC_TABLE.length; value += 1) {
  let crc = value;
  for (let bit = 0; bit < 8; bit += 1) {
    crc = (crc & 1) ? (0xedb88320 ^ (crc >>> 1)) : (crc >>> 1);
  }
  CRC_TABLE[value] = crc >>> 0;
}

function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) crc = CRC_TABLE[(crc ^ byte) & 0xff] ^ (crc >>> 8);
  return (crc ^ 0xffffffff) >>> 0;
}

function paeth(left, above, upperLeft) {
  const estimate = left + above - upperLeft;
  const leftDistance = Math.abs(estimate - left);
  const aboveDistance = Math.abs(estimate - above);
  const upperLeftDistance = Math.abs(estimate - upperLeft);
  if (leftDistance <= aboveDistance && leftDistance <= upperLeftDistance) return left;
  if (aboveDistance <= upperLeftDistance) return above;
  return upperLeft;
}

export function decodeRgbaPng(bytes) {
  assert.ok(Buffer.isBuffer(bytes), 'PNG bytes는 Buffer여야 합니다.');
  assert.ok(bytes.length >= 33, 'PNG 파일 길이가 올바르지 않습니다.');
  assert.deepEqual(bytes.subarray(0, SIGNATURE.length), SIGNATURE, 'PNG signature가 다릅니다.');

  let width;
  let height;
  let bitDepth;
  let colorType;
  let interlace;
  let sawHeader = false;
  let sawEnd = false;
  const compressedData = [];
  let offset = SIGNATURE.length;
  while (offset < bytes.length) {
    assert.ok(offset + 12 <= bytes.length, 'PNG chunk header가 잘렸습니다.');
    const length = bytes.readUInt32BE(offset);
    const type = bytes.toString('ascii', offset + 4, offset + 8);
    const dataStart = offset + 8;
    const dataEnd = dataStart + length;
    assert.ok(dataEnd + 4 <= bytes.length, `${type} PNG chunk이 잘렸습니다.`);
    const typeAndData = bytes.subarray(offset + 4, dataEnd);
    assert.equal(bytes.readUInt32BE(dataEnd), crc32(typeAndData), `${type} PNG chunk CRC가 다릅니다.`);
    const data = bytes.subarray(dataStart, dataEnd);

    if (type === 'IHDR') {
      assert.equal(sawHeader, false, 'PNG IHDR이 중복되었습니다.');
      assert.equal(length, 13, 'PNG IHDR 길이가 올바르지 않습니다.');
      width = data.readUInt32BE(0);
      height = data.readUInt32BE(4);
      bitDepth = data[8];
      colorType = data[9];
      assert.equal(data[10], 0, 'PNG compression method를 지원하지 않습니다.');
      assert.equal(data[11], 0, 'PNG filter method를 지원하지 않습니다.');
      interlace = data[12];
      sawHeader = true;
    } else if (type === 'IDAT') {
      assert.equal(sawHeader, true, 'PNG IHDR보다 IDAT가 먼저 나왔습니다.');
      compressedData.push(data);
    } else if (type === 'IEND') {
      assert.equal(length, 0, 'PNG IEND에 payload가 있습니다.');
      sawEnd = true;
      offset = dataEnd + 4;
      break;
    }
    offset = dataEnd + 4;
  }

  assert.equal(sawHeader, true, 'PNG IHDR이 없습니다.');
  assert.equal(sawEnd, true, 'PNG IEND가 없습니다.');
  assert.ok(Number.isSafeInteger(width) && width > 0 && Number.isSafeInteger(height) && height > 0,
    'PNG 크기가 올바르지 않습니다.');
  assert.ok(width * height <= 20_000_000, 'PNG가 허용 픽셀 수를 넘었습니다.');
  assert.equal(bitDepth, 8, '8-bit PNG만 지원합니다.');
  assert.ok(colorType === 2 || colorType === 6, 'RGB 또는 RGBA PNG만 지원합니다.');
  assert.equal(interlace, 0, 'interlaced PNG는 지원하지 않습니다.');

  const sourceChannels = colorType === 6 ? 4 : 3;
  const rowBytes = width * sourceChannels;
  const expectedRawBytes = height * (rowBytes + 1);
  const raw = inflateSync(Buffer.concat(compressedData), { maxOutputLength: expectedRawBytes });
  assert.equal(raw.length, expectedRawBytes, 'PNG inflate 크기가 예상과 다릅니다.');
  const sourcePixels = Buffer.alloc(height * rowBytes);

  let sourceOffset = 0;
  for (let y = 0; y < height; y += 1) {
    const filter = raw[sourceOffset++];
    assert.ok(filter <= 4, `알 수 없는 PNG row filter입니다: ${filter}`);
    const rowOffset = y * rowBytes;
    const previousRow = rowOffset - rowBytes;
    for (let index = 0; index < rowBytes; index += 1) {
      const encoded = raw[sourceOffset++];
      const left = index >= sourceChannels ? sourcePixels[rowOffset + index - sourceChannels] : 0;
      const above = y > 0 ? sourcePixels[previousRow + index] : 0;
      const upperLeft = y > 0 && index >= sourceChannels
        ? sourcePixels[previousRow + index - sourceChannels] : 0;
      const predictor = filter === 0 ? 0
        : filter === 1 ? left
          : filter === 2 ? above
            : filter === 3 ? Math.floor((left + above) / 2)
              : paeth(left, above, upperLeft);
      sourcePixels[rowOffset + index] = (encoded + predictor) & 0xff;
    }
  }

  const rgba = Buffer.alloc(width * height * 4);
  for (let index = 0; index < width * height; index += 1) {
    const source = index * sourceChannels;
    const destination = index * 4;
    rgba[destination] = sourcePixels[source];
    rgba[destination + 1] = sourcePixels[source + 1];
    rgba[destination + 2] = sourcePixels[source + 2];
    rgba[destination + 3] = colorType === 6 ? sourcePixels[source + 3] : 255;
  }

  return {
    width,
    height,
    getPixel(x, y) {
      assert.ok(Number.isInteger(x) && x >= 0 && x < width, 'PNG x 표본이 범위 밖입니다.');
      assert.ok(Number.isInteger(y) && y >= 0 && y < height, 'PNG y 표본이 범위 밖입니다.');
      const offset = (y * width + x) * 4;
      return [...rgba.subarray(offset, offset + 4)];
    },
  };
}
