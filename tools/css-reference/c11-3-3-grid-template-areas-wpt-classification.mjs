import assert from 'node:assert/strict';

const classifications = new Set([
  'currently-unsupported',
  'spinon-fixture-candidate',
]);

export function classifyCandidateSubtests(candidate, reportSubtests) {
  assert.ok(Array.isArray(reportSubtests), 'WPT report subtests must be an array.');
  assert.ok(reportSubtests.every((subtest) => subtest && typeof subtest === 'object' && !Array.isArray(subtest)),
    'WPT report subtests must be objects.');

  const reportNames = reportSubtests.map(({ name }) => name);
  assert.ok(reportNames.every((name) => typeof name === 'string' && name.length > 0),
    'WPT report subtests must have non-empty names.');
  assert.equal(new Set(reportNames).size, reportNames.length,
    'WPT report subtest names must be unique.');

  const explicitCoverage = candidate.subtestCoverage;
  const coverageByName = new Map();
  if (explicitCoverage !== undefined) {
    assert.ok(Array.isArray(explicitCoverage), 'subtestCoverage must be an array.');
    for (const entry of explicitCoverage) {
      assert.ok(entry && typeof entry.name === 'string' && entry.name.length > 0,
        'Each explicit coverage entry must have a non-empty subtest name.');
      assert.ok(classifications.has(entry.classification),
        `Unknown subtest classification for ${entry.name}.`);
      assert.ok(!coverageByName.has(entry.name),
        `Duplicate explicit coverage for ${entry.name}.`);
      if (entry.classification === 'spinon-fixture-candidate') {
        assert.ok(typeof entry.fixtureSceneId === 'string' && entry.fixtureSceneId.length > 0,
          `Fixture candidate ${entry.name} must identify its derived scene.`);
        assert.ok(typeof entry.fixtureEvidence === 'string' && entry.fixtureEvidence.length > 0,
          `Fixture candidate ${entry.name} must identify its evidence.`);
      }
      coverageByName.set(entry.name, entry);
    }

    assert.deepEqual(
      [...coverageByName.keys()].sort(),
      [...reportNames].sort(),
      'Explicit WPT subtest coverage must match the pinned report exactly.',
    );
  }

  const legacyClassification = candidate.classification.startsWith('Currently unsupported:')
    ? 'currently-unsupported'
    : 'spinon-fixture-candidate';

  return reportSubtests.map((subtest, index) => {
    const explicit = coverageByName.get(subtest.name);
    const result = {
      index: index + 1,
      name: subtest.name,
      status: subtest.status,
      classification: explicit?.classification ?? legacyClassification,
      reason: explicit?.reason ?? candidate.subtestClassification,
    };
    if (explicit?.fixtureSceneId) result.fixtureSceneId = explicit.fixtureSceneId;
    if (explicit?.fixtureEvidence) result.fixtureEvidence = explicit.fixtureEvidence;
    return result;
  });
}

export function assertFixtureSceneMappings(candidate, fixtureHtml) {
  const mappedEntries = (candidate.subtestCoverage ?? [])
    .filter(({ fixtureSceneId }) => fixtureSceneId !== undefined);
  if (mappedEntries.length === 0) return;

  assert.ok(typeof candidate.fixtureSceneSource === 'string' && candidate.fixtureSceneSource.length > 0,
    'Fixture scene mappings must identify the source HTML.');
  assert.ok(typeof fixtureHtml === 'string', 'Fixture source must be read as text.');

  const sceneIds = [...fixtureHtml.matchAll(/\bdata-scene\s*=\s*["']([^"']+)["']/g)]
    .map((match) => match[1]);
  const declaredSceneIds = new Set(sceneIds);
  assert.equal(declaredSceneIds.size, sceneIds.length,
    `Fixture source ${candidate.fixtureSceneSource} declares duplicate data-scene IDs.`);
  for (const entry of mappedEntries) {
    assert.ok(declaredSceneIds.has(entry.fixtureSceneId),
      `Mapped fixture scene ${entry.fixtureSceneId} is absent from ${candidate.fixtureSceneSource}.`);
  }
}
