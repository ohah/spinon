import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import {
  assertFixtureSceneMappings,
  classifyCandidateSubtests,
} from './c11-3-3-grid-template-areas-wpt-classification.mjs';

const wptNames = ['.grid 1', '.grid 2', '.grid 3', '.grid 4'];
const fixtureSceneIds = [
  'unknown-both-named-row',
  'unknown-column-named-span',
  'unknown-row-named-span',
  'unknown-column-span-one',
];

function unknownNamedAreaCandidate() {
  return {
    classification: 'Four pinned subtests have derived fixture coverage.',
    subtestClassification: 'Derived opt-in scenes are not Spinon WPT execution.',
    subtestCoverage: wptNames.map((name, index) => ({
      name,
      classification: 'spinon-fixture-candidate',
      fixtureSceneId: fixtureSceneIds[index],
      fixtureEvidence: 'plan/c11-3-3-unknown-named-area-implicit-lines.md',
    })),
  };
}

const report = wptNames.map((name) => ({ name, status: 'PASS' }));
const fixtureHtml = readFileSync(new URL(
  '../../tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133.html',
  import.meta.url,
), 'utf8');

test('maps each pinned unknown-area WPT subtest to its derived scene by report name', () => {
  const classified = classifyCandidateSubtests(unknownNamedAreaCandidate(), report);

  assert.deepEqual(classified.map(({ fixtureSceneId }) => fixtureSceneId), fixtureSceneIds);
  assert.ok(classified.every(({ classification }) => classification === 'spinon-fixture-candidate'));
  assert.ok(classified.every(({ fixtureEvidence }) =>
    fixtureEvidence === 'plan/c11-3-3-unknown-named-area-implicit-lines.md'));
});

test('requires each mapped fixture scene to exist in its declared source HTML', () => {
  const candidate = unknownNamedAreaCandidate();
  candidate.fixtureSceneSource = 'tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133.html';
  assert.doesNotThrow(() => assertFixtureSceneMappings(candidate, fixtureHtml));

  candidate.subtestCoverage[0].fixtureSceneId = 'removed-scene';
  assert.throws(
    () => assertFixtureSceneMappings(candidate, fixtureHtml),
    /Mapped fixture scene removed-scene is absent/,
  );
});

test('rejects ambiguous fixture HTML with duplicate scene IDs', () => {
  const candidate = unknownNamedAreaCandidate();
  candidate.fixtureSceneSource = 'fixture.html';
  assert.throws(
    () => assertFixtureSceneMappings(candidate,
      '<div data-scene="unknown-both-named-row"></div><section data-scene="unknown-both-named-row"></section>'),
    /declares duplicate data-scene IDs/,
  );
});

test('mapping is independent of WPT report order', () => {
  const classified = classifyCandidateSubtests(unknownNamedAreaCandidate(), [...report].reverse());

  assert.deepEqual(classified.map(({ fixtureSceneId }) => fixtureSceneId), [...fixtureSceneIds].reverse());
});

test('rejects a missing or newly added report subtest when explicit coverage is pinned', () => {
  assert.throws(
    () => classifyCandidateSubtests(unknownNamedAreaCandidate(), report.slice(0, 3)),
    /must match the pinned report exactly/,
  );
  assert.throws(
    () => classifyCandidateSubtests(unknownNamedAreaCandidate(), [
      ...report.slice(0, 3), { name: '.grid 5', status: 'PASS' },
    ]),
    /must match the pinned report exactly/,
  );
});

test('rejects malformed report entries before reading subtest names', () => {
  assert.throws(
    () => classifyCandidateSubtests(unknownNamedAreaCandidate(), [null]),
    /WPT report subtests must be objects/,
  );
});

test('rejects duplicate explicit subtest mappings', () => {
  const candidate = unknownNamedAreaCandidate();
  candidate.subtestCoverage.push({ ...candidate.subtestCoverage[0] });

  assert.throws(
    () => classifyCandidateSubtests(candidate, report),
    /Duplicate explicit coverage/,
  );
});

test('keeps legacy whole-test classifications for candidates without explicit mapping', () => {
  const unsupported = classifyCandidateSubtests({
    classification: 'Currently unsupported: implicit tracks.',
    subtestClassification: 'No fixture covers this test.',
  }, report.slice(0, 1));
  const candidate = classifyCandidateSubtests({
    classification: 'Spinon fixture candidates.',
    subtestClassification: 'Derived fixture only.',
  }, report.slice(0, 1));

  assert.equal(unsupported[0].classification, 'currently-unsupported');
  assert.equal(candidate[0].classification, 'spinon-fixture-candidate');
});
