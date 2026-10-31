import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import fs from 'node:fs';

const fixturePath = 'tests/fixtures/css/c11/grid-track-sizing-c114-anonymous-text-word-spacing.html';
const fontPath = 'tests/fixtures/resources/c15/Ahem.ttf';
const evidence = 'spec/internal/evidence/c11-4-anonymous-grid-text-word-spacing-precomparison-2026-10-31';
const hashes = {
  fixture: 'feed66c8e8ef34b20aca20d91899b3f3369e9346e6b70e95646c79908f3f2d65',
  font: 'b719ecb31c5b21fc573c03f6421c74ac63c271a5a3ff841e34f9705fb94b8448',
};
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
assert.equal(sha256(fs.readFileSync(fixturePath)), hashes.fixture, 'fixture input changed');
assert.equal(sha256(fs.readFileSync(fontPath)), hashes.font, 'Ahem input changed');
const observation = JSON.parse(fs.readFileSync(`${evidence}/chrome156-observation.json`, 'utf8'));
assert.equal(observation.fontLoaded, true);
assert.match(observation.userAgent, /HeadlessChrome\/156\.0\.0\.0/);
assert.deepEqual(observation.viewport, { width: 1280, height: 633 });
assert.equal(observation.dpr, 1);

// Independent calculation: Ahem has 1000 units/em for X and U+0020, and the fixture uses 10px.
// CSS Text normal-space processing collapses U+0020 runs and discards line-edge spaces.
const values = [
  { id: 'normal', authored: 'normal', computed: '0px', extra: 0 },
  { id: 'zero', authored: '0px', computed: '0px', extra: 0 },
  { id: 'minus-one', authored: '-1px', computed: '-1px', extra: -1 },
  { id: 'plus-one', authored: '1px', computed: '1px', extra: 1 },
];
const contents = [
  { id: 'two-words', text: 'XX XX', words: ['XX', 'XX'] },
  { id: 'collapsed-three-words', text: 'XX   XX XX  ', words: ['XX', 'XX', 'XX'] },
  { id: 'single-word', text: 'XX', words: ['XX'] },
];
const expected = [];
for (const value of values) {
  for (const content of contents) {
    for (const track of ['min', 'max']) {
      const id = `word-spacing-${value.id}-${content.id}-${track}-content`;
      const letterCount = content.words.reduce((sum, word) => sum + word.length, 0);
      const separatorCount = content.words.length - 1;
      const width = track === 'min'
        ? Math.max(...content.words.map((word) => word.length)) * 10
        : letterCount * 10 + separatorCount * (10 + value.extra);
      const height = track === 'min' ? content.words.length * 10 : 10;
      expected.push({ id, authoredWordSpacing: value.authored, computedWordSpacing: value.computed, text: content.text, track, columnsPx: width, rowsPx: height });
    }
  }
}
assert.equal(observation.observations.length, expected.length);
for (let i = 0; i < expected.length; i += 1) {
  const actual = observation.observations[i];
  const item = expected[i];
  assert.equal(actual.id, item.id, `${item.id}: scene order`);
  for (const key of ['authoredWordSpacing', 'computedWordSpacing', 'text', 'track']) assert.equal(actual[key], item[key], `${item.id}: ${key}`);
  assert.equal(actual.columns, `${item.columnsPx}px`, `${item.id}: Columns`);
  assert.equal(actual.rows, `${item.rowsPx}px`, `${item.id}: Rows`);
  assert.deepEqual(actual.rect, { ...actual.rect, width: item.columnsPx, height: item.rowsPx }, `${item.id}: grid frame`);
}
const output = {
  method: 'CSS Text word separator rules + Ahem hmtx units/em; no Chrome geometry copied into expectations',
  ahem: { unitsPerEm: 1000, xAdvanceUnits: 1000, spaceAdvanceUnits: 1000, fontSizeCssPx: 10 },
  whiteSpace: 'normal (collapse runs; discard line-edge spaces)',
  formula: {
    minContentWidth: '10px * longest unbreakable word length',
    maxContentWidth: '10px * total X count + (10px + computed word-spacing) * collapsed separator count',
    minContentHeight: '10px * word count',
    maxContentHeight: '10px',
  },
  cases: expected,
  chromeExactMatches: expected.length,
};
fs.writeFileSync(`${evidence}/spec-derived-expectations.json`, `${JSON.stringify(output, null, 2)}\n`);
console.log(JSON.stringify({ fixtureSha256: hashes.fixture, ahemSha256: hashes.font, scenes: expected.length, exactMatches: expected.length, written: `${evidence}/spec-derived-expectations.json` }));
