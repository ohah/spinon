import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { readdir, readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const legacyFixtureSources = [
  'crates/spinon-ffi/src/runtime_gpu/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_2_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/c11_2_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c06.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c07.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c08.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c09.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c10.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c10_3_5.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c11_2_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/c12.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/probe.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/layout_probe.rs',
];
const canonicalFixtureSources = [
  'crates/spinon-ffi/src/runtime_gpu/fixtures.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll_host.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture_host.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_named_lines.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_calc_minimum.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_calc_neighbors.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_positive_percent_minimum.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_zero_percent_minimum.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_percentage_maximum.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_percentage_neighbors.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_min_height.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_min_height_report.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_percentage_argument.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_percentage_argument_report.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_min_floor.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_min_floor_report.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_auto_repeat_rows.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_repeat_intrinsic_wpt.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/fixture.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/scene.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/tests.rs',
  'crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11_3_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_3.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c11.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_1.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_2.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_4.rs',
  'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c15_3.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/fixtures.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/fixtures/probe.rs',
  'crates/spinon-ffi/src/runtime_ua_cascade/fixtures/layout_probe.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_min_height.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_percentage_argument.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_percentage_argument/style_validation.rs',
  'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_auto_min_floor.rs',
];
test('fixture 이동 뒤 이전 경로에 Rust 사본이 남지 않는다', () => {
  for (const relativePath of legacyFixtureSources) {
    assert.equal(
      existsSync(resolve(repositoryRoot, relativePath)),
      false,
      `이전 fixture 경로에 소스가 남았습니다: ${relativePath}`,
    );
  }
});

test('calc 이웃 13 scene이 독립 oracle과 앱 경로에 고정된다', async () => {
  const fixturePath = 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_calc_neighbor_scenes.json';
  const fixture = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
  const fixtureIDs = fixture.map(({ id }) => id);
  const oracleIDs = fixture.map(({ oracleId }) => oracleId);
  assert.deepEqual(fixtureIDs, [
    'calc-neighbor-col-fill-both', 'calc-neighbor-row-fill-both',
    'calc-neighbor-col-fill-prefix', 'calc-neighbor-row-fill-suffix',
    'calc-neighbor-col-fit-first', 'calc-neighbor-col-fit-empty',
    'calc-neighbor-col-fill-overflow', 'calc-neighbor-col-fill-border-box',
    'calc-neighbor-col-fill-extra-term', 'calc-neighbor-col-fill-negative-term',
    'calc-neighbor-col-fill-negative-final',
    'calc-neighbor-col-max-content', 'calc-neighbor-col-fill-fixed-control',
  ]);
  assert.equal(new Set(fixtureIDs).size, fixtureIDs.length, 'runtime scene ID collision');
  assert.deepEqual(oracleIDs, [
    'col-fill-both', 'row-fill-both', 'col-fill-prefix', 'row-fill-suffix',
    'col-fit-first', 'col-fit-empty', 'col-fill-overflow', 'col-fill-border-box',
    'col-fill-extra-term', 'col-fill-negative-term', 'col-fill-negative-final',
    'col-max-content', 'col-fill-fixed-control',
  ]);
  assert.equal(fixture.filter(({ expectedRejection }) => expectedRejection).length, 2);

  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const captureSource = await readFile(capturePath, 'utf8');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 13);
  assert.deepEqual(catalog.selectedSceneIDs, fixtureIDs);

  const android = await readFile(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8');
  const ios = await readFile(resolve(repositoryRoot,
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8');
  const runtimeFixtures = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures.rs'), 'utf8');
  const runtimeFixtureRoute = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8');
  const fixtureSource = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_calc_neighbors.rs'), 'utf8');
  for (const id of fixtureIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  assert.ok(captureSource.includes('function autoRepeatCalcNeighborScenes()'));
  assert.match(runtimeFixtures,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_auto_repeat_calc_neighbors;/);
  assert.ok(runtimeFixtureRoute.includes('c11_4_auto_repeat_calc_neighbors::runtime_source'));
  assert.ok(fixtureSource.includes('auto_repeat_calc_neighbor_scenes.json'));
  assert.ok(android.includes('c114CalcNeighborExpectedRejection'));
  assert.ok(ios.includes('c114CalcNeighborExpectedRejection'));
  assert.ok(android.includes('c114CalcNeighborPixelSamplesMatch'));
  assert.ok(ios.includes('c114CalcNeighborPixelSamples'));
});

test('음수 percentage calc 이웃 9 scene이 규범 기대값과 앱 경로에 고정된다', async () => {
  const fixturePath = 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_negative_percentage_neighbor_scenes.json';
  const fixture = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
  const fixtureIDs = fixture.map(({ id }) => id);
  const oracleIDs = fixture.map(({ oracleId }) => oracleId);
  assert.deepEqual(fixtureIDs, [
    'negative-percentage-col-prefix-positive',
    'negative-percentage-col-suffix-positive',
    'negative-percentage-row-prefix-positive',
    'negative-percentage-row-suffix-positive',
    'negative-percentage-col-prefix-negative-final',
    'negative-percentage-row-suffix-negative-final',
    'negative-percentage-col-border-box',
    'negative-percentage-col-fit-prefix-item',
    'negative-percentage-col-both-negative-final',
  ]);
  assert.deepEqual(oracleIDs, [
    'col-prefix-positive', 'col-suffix-positive', 'row-prefix-positive',
    'row-suffix-positive', 'col-prefix-negative-final', 'row-suffix-negative-final',
    'col-border-box', 'col-fit-prefix-item', 'col-both-negative-final',
  ]);
  assert.equal(new Set(fixtureIDs).size, fixtureIDs.length, 'runtime scene ID collision');
  assert.ok(fixture.every(({ expectedRejection }) => expectedRejection === undefined));

  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const unselected = spawnSync(process.execPath, [capturePath, 'android-avd',
    '--run-kind=zero-fixed-maximum', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.notEqual(unselected.status, 0,
    'zero fixed-maximum run-kind must reject the unfiltered full catalog.');
  assert.match(unselected.stderr, /zero fixed-maximum run requires an explicit scene selection/);
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 9);
  assert.deepEqual(catalog.selectedSceneIDs, fixtureIDs);

  const android = await readFile(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8');
  const ios = await readFile(resolve(repositoryRoot,
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8');
  const runtimeFixtures = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures.rs'), 'utf8');
  const runtimeFixtureRoute = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8');
  const fixtureSource = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_negative_percentage_neighbors.rs'), 'utf8');
  for (const id of fixtureIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  assert.match(runtimeFixtures,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_auto_repeat_negative_percentage_neighbors;/);
  assert.ok(runtimeFixtureRoute.includes('c11_4_auto_repeat_negative_percentage_neighbors::runtime_source'));
  assert.ok(fixtureSource.includes('auto_repeat_negative_percentage_neighbor_scenes.json'));
  assert.ok(android.includes('negative-percentage-col-fit-prefix-item'));
  assert.ok(ios.includes('negative-percentage-col-fit-prefix-item'));
  assert.ok(android.includes('sampleId.startsWith("negative-percentage-")'),
    'Android pixel-color oracle does not recognize the new scene IDs');
});

test('41-scene 기준 matrix와 11-scene 교차축 slice가 capture·host 경로에 고정된다', async () => {
  const expectedIDs = [
    'template-area-repeat-two-axis-area-short',
    'template-area-repeat-two-axis-area-added-columns',
    'template-area-repeat-two-axis-area-added-rows',
    'template-area-repeat-two-axis-area-added-both',
    'template-area-repeat-two-axis-fill-fit',
    'template-area-repeat-two-axis-fit-fill',
    'template-area-repeat-two-axis-fit-fit-empty',
    'template-area-repeat-two-axis-fit-fit-span',
    'template-area-repeat-two-axis-area-longhand',
    'template-area-repeat-two-axis-fit-fit-area-empty',
    'template-area-repeat-two-axis-fit-fit-area-item',
    'template-area-repeat-two-axis-fit-fit-area-span',
    'template-area-repeat-two-axis-fit-fill-area-item',
    'template-area-repeat-two-axis-fill-fit-area-item',
    'template-area-repeat-two-axis-fit-fit-area-column',
    'template-area-repeat-two-axis-fit-fit-area-row',
  ];
  const [fixtureText, calcFixtureText, percentFixtureText, zeroPercentFixtureText,
    maximumFixtureText, neighborFixtureText, neighborExpectationText, neighborChromeText,
    android, ios, runtimeFfiHeader, captureSource, noAreaOracleText, matrixManifestText] = await Promise.all([
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/template_area_auto_repeat_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_calc_minimum_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_positive_percent_minimum_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_zero_percent_minimum_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_percentage_maximum_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_percentage_neighbor_scenes.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/spec-derived-expectations.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'spec/internal/evidence/c11-4-auto-repeat-percentage-neighbors-precomparison-2026-10-24/chrome156-observation.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/include/spinon_ffi.h'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'spec/internal/evidence/c11-4-two-axis-auto-repeat-precomparison-2026-10-20/chrome155-observation.json'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'spec/internal/evidence/c11-4-auto-fit-area-expansion-runtime-2026-10-23/focused-matrix-41-scenes.json'),
    'utf8'),
  ]);
  const fixtures = JSON.parse(fixtureText);
  const calcFixtures = JSON.parse(calcFixtureText);
  const percentFixtures = JSON.parse(percentFixtureText);
  const zeroPercentFixtures = JSON.parse(zeroPercentFixtureText);
  const neighborFixtures = JSON.parse(neighborFixtureText);
  const neighborExpectations = JSON.parse(neighborExpectationText);
  const neighborChromeObservations = JSON.parse(neighborChromeText);
  assert.ok(captureSource.includes('function c114TraceChunkState(log, runID)'),
    'C11.4 simulator capture must verify complete trace chunk sets.');
  assert.ok((captureSource.match(/c114TraceChunkState\(log, runID\)/g) ?? []).length >= 2,
    'C11.4 simulator capture must wait for complete trace chunks and verify the saved set.');
  const maximumFixtures = JSON.parse(maximumFixtureText);
  assert.equal(neighborFixtures.length, 13,
    'percentage auto-repeat neighbor fixture는 13 scene이어야 합니다.');
  assert.equal(neighborExpectations.cases.length, 13,
    'percentage auto-repeat neighbor normative matrix는 13 case여야 합니다.');
  assert.equal(neighborChromeObservations.scenes.length, 13,
    'percentage auto-repeat neighbor Chrome observation은 13 scene이어야 합니다.');
  assert.deepEqual(neighborFixtures.map(({ id }) => id),
    neighborExpectations.cases.map(({ id }) => id));
  assert.deepEqual(neighborFixtures.map(({ id }) => id),
    neighborChromeObservations.scenes.map(({ id }) => id));
  assert.equal(fixtures.length, 40,
    'template-area fixture는 단일 축 13개, 양 축 16개와 교차축 11개다.');
  assert.equal(calcFixtures.length, 12,
    'absolute 및 percentage calc auto-repeat fixture는 12개다.');
  assert.equal(percentFixtures.length, 16,
    '양수 percentage auto-repeat fixture는 16개다.');
  assert.equal(zeroPercentFixtures.length, 13,
    'zero-percent auto-repeat fixture는 13개다.');
  assert.equal(maximumFixtures.length, 28,
    'percentage auto-repeat maximum fixture는 28개다.');
  assert.match(captureSource, /assert\.equal\(scenes\.length, 618,/,
    'C11.4 simulator capture scene count must include percentage-neighbor cases.');
  const noAreaIDs = JSON.parse(noAreaOracleText).cases.map(({ id }) => id);
  assert.equal(noAreaIDs.length, 12, 'the no-area dual-axis oracle must contain 12 scenes.');
  const matrixManifest = JSON.parse(matrixManifestText);
  assert.deepEqual(matrixManifest.noAreaDualAxisSceneIDs, noAreaIDs);
  const crossAxisFixtures = fixtures.filter(({ id }) => (
    id.startsWith('template-area-repeat-cross-axis-')));
  const priorTemplateAreaIDs = fixtures.filter(({ id }) => (
    !id.startsWith('template-area-repeat-cross-axis-'))).map(({ id }) => id);
  assert.equal(crossAxisFixtures.length, 11);
  assert.deepEqual(matrixManifest.templateAreaSceneIDs, priorTemplateAreaIDs,
    'the historical 41-scene matrix must keep its original 29 template-area scenes.');
  const listScenes = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(listScenes.status, 0, listScenes.stderr);
  const captureCatalog = JSON.parse(listScenes.stdout);
  assert.equal(captureCatalog.sceneCount, 622);
  const matrixIDs = [...noAreaIDs, ...priorTemplateAreaIDs];
  assert.equal(matrixIDs.length, 41, 'the focused C11.4 matrix must contain 41 scenes.');
  assert.equal(new Set(matrixIDs).size, matrixIDs.length,
    'the focused C11.4 matrix must not repeat scene IDs.');
  assert.equal(matrixManifest.sceneCount, matrixIDs.length);
  assert.equal(matrixManifest.uniqueSceneCount, matrixIDs.length);
  const captureIDs = new Set(captureCatalog.sceneIDs);
  assert.deepEqual(matrixIDs.filter((id) => !captureIDs.has(id)), [],
    'every focused matrix scene must be present in the simulator capture route.');
  const selectedCatalogResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${matrixIDs.join(',')}`, '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedCatalogResult.status, 0, selectedCatalogResult.stderr);
  const selectedCatalog = JSON.parse(selectedCatalogResult.stdout);
  assert.equal(selectedCatalog.sceneCount, 622);
  assert.equal(selectedCatalog.selectedSceneCount, 41);
  assert.deepEqual([...selectedCatalog.selectedSceneIDs].sort(), [...matrixIDs].sort(),
    'the simulator selection must resolve to exactly the 41 focused matrix IDs.');
  assert.deepEqual(crossAxisFixtures.map(({ id }) => id).filter((id) => !captureIDs.has(id)), [],
    'every new cross-axis scene must be present in the simulator capture route.');
  const selectedCrossAxisResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${crossAxisFixtures.map(({ id }) => id).join(',')}`,
    '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedCrossAxisResult.status, 0, selectedCrossAxisResult.stderr);
  const selectedCrossAxis = JSON.parse(selectedCrossAxisResult.stdout);
  assert.equal(selectedCrossAxis.sceneCount, 622);
  assert.equal(selectedCrossAxis.selectedSceneCount, 11);
  assert.deepEqual([...selectedCrossAxis.selectedSceneIDs].sort(),
    crossAxisFixtures.map(({ id }) => id).sort(),
    'the simulator must select exactly the 11 new cross-axis scenes.');
  assert.match(selectedCrossAxis.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-cross-axis-fixed-auto-repeat-areas-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-cross-axis-fixed-auto-area-/,
    'cross-axis scene IDs must default to their own evidence folder.');
  assert.match(captureSource,
    /requestedSceneIDs\.every\(\(sceneID\) => \(\s*sceneID\.startsWith\('template-area-repeat-cross-axis-'\)/,
    'cross-axis IDs must infer their run family without an explicit evidence-group or run-kind.');
  assert.match(captureSource,
    /\? \[itemSample\[0\], rootHeight\]\s*:\s*\[rootWidth, itemSample\[1\]\]/,
    'overflow pre-item sample must hit the first pixel outside the root.');
  assert.match(captureSource,
    /\? \[itemSample\[0\], y \+ height\]\s*:\s*\[x \+ width, itemSample\[1\]\]/,
    'overflow after-item sample must hit the first pixel after the hero edge.');
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\) && SPINON_ENABLE_C04_RUNTIME_GPU && \\\s*defined\(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST\) && \\\s*SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST\s*\/\* C11\.4 고정 track-sizing scene host는 앱 fixture-test 빌드 전용입니다\. \*\/\s*SpinonRuntimeGpuHost \*spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture\([\s\S]{0,100}#endif/,
    'C11.4 fixture host declaration must be hidden unless the fixture-test define is enabled.');
  const calcIDs = calcFixtures.map(({ id }) => id);
  assert.deepEqual(calcIDs, [
    'columns-calc-fill', 'columns-fixed-fill-control', 'columns-calc-fit',
    'rows-calc-fill', 'rows-fixed-fill-control', 'rows-calc-fit',
    'columns-percent-calc-fill', 'columns-percent-fixed-fill-control',
    'columns-percent-calc-fit', 'rows-percent-calc-fill',
    'rows-percent-fixed-fill-control', 'rows-percent-calc-fit',
  ]);
  assert.deepEqual(calcIDs.filter((id) => !captureIDs.has(id)), [],
    'every calc fixture must be present in the simulator capture route.');
  for (const id of calcIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  const selectedCalcResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${calcIDs.join(',')}`, '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedCalcResult.status, 0, selectedCalcResult.stderr);
  const selectedCalcCatalog = JSON.parse(selectedCalcResult.stdout);
  assert.equal(selectedCalcCatalog.selectedSceneCount, 12);
  assert.deepEqual([...selectedCalcCatalog.selectedSceneIDs].sort(), [...calcIDs].sort());
  const percentIDs = percentFixtures.map(({ id }) => id);
  assert.equal(percentIDs.length, 16);
  assert.equal(new Set(percentIDs).size, percentIDs.length);
  assert.deepEqual(percentIDs.filter((id) => !captureIDs.has(id)), [],
    'every positive percentage fixture must be present in the simulator capture route.');
  for (const id of percentIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  const selectedPercentResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${percentIDs.join(',')}`, '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedPercentResult.status, 0, selectedPercentResult.stderr);
  const selectedPercentCatalog = JSON.parse(selectedPercentResult.stdout);
  assert.equal(selectedPercentCatalog.sceneCount, 622);
  assert.equal(selectedPercentCatalog.selectedSceneCount, 16);
  assert.deepEqual([...selectedPercentCatalog.selectedSceneIDs].sort(), [...percentIDs].sort());
  const zeroPercentIDs = zeroPercentFixtures.map(({ id }) => id);
  assert.equal(new Set(zeroPercentIDs).size, zeroPercentIDs.length);
  assert.deepEqual(zeroPercentIDs.filter((id) => !captureIDs.has(id)), [],
    'every zero-percent fixture must be present in the simulator capture route.');
  for (const id of zeroPercentIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  const androidZeroPercentExpectations = android.slice(
    android.indexOf('private static String c114ZeroPercentMinimumRootFrame'),
    android.indexOf('private static float[][] c114PercentageMaximumItemFrames'),
  );
  const iosZeroPercentExpectations = ios.slice(
    ios.indexOf('private func c114ZeroPercentMinimumRootFrame'),
    ios.indexOf('private struct C114PercentageMaximumItemFrame'),
  );
  assert.ok(androidZeroPercentExpectations.length > 0, 'Android zero-percent frame expectations are missing.');
  assert.ok(iosZeroPercentExpectations.length > 0, 'iOS zero-percent frame expectations are missing.');
  for (const id of zeroPercentIDs) {
    assert.ok(androidZeroPercentExpectations.includes(`"${id}"`),
      `Android zero-percent frame expectations are missing ${id}`);
    assert.ok(iosZeroPercentExpectations.includes(`"${id}"`),
      `iOS zero-percent frame expectations are missing ${id}`);
  }
  const selectedZeroPercentResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${zeroPercentIDs.join(',')}`, '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedZeroPercentResult.status, 0, selectedZeroPercentResult.stderr);
  const selectedZeroPercentCatalog = JSON.parse(selectedZeroPercentResult.stdout);
  assert.equal(selectedZeroPercentCatalog.sceneCount, 622);
  assert.equal(selectedZeroPercentCatalog.selectedSceneCount, 13);
  assert.deepEqual([...selectedZeroPercentCatalog.selectedSceneIDs].sort(), [...zeroPercentIDs].sort());
  const maximumIDs = maximumFixtures.map(({ id }) => id);
  assert.equal(maximumIDs.length, 28);
  assert.equal(new Set(maximumIDs).size, maximumIDs.length);
  assert.deepEqual(maximumIDs.filter((id) => !captureIDs.has(id)), [],
    'every percentage maximum fixture must be present in the simulator capture route.');
  assert.ok(captureSource.includes('xCssPx: Math.floor(x + width / 2)'),
    'fractional track centers must become valid integer surface sample coordinates.');
  assert.ok(android.includes('C114_PERCENTAGE_MAXIMUM_PIXEL_SCENE_IDS'),
    'Android must gate percentage maximum pixel samples to the fixed scene set.');
  assert.ok(android.includes('isC114PercentageMaximumPixelSceneId(pixelMatrixSceneId)'),
    'Android pixel-matrix scene selection must accept the same fixed percentage maximum IDs.');
  assert.ok(android.includes('c114PercentageMaximumPixelRgb(sampleId)'),
    'Android must compare percentage maximum samples against fixed expected colors.');
  assert.ok(ios.includes('c114PercentageMaximumPixelSceneIDs'),
    'iOS must gate percentage maximum pixel samples to the fixed scene set.');
  for (const id of maximumIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  const selectedMaximumResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${maximumIDs.join(',')}`, '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedMaximumResult.status, 0, selectedMaximumResult.stderr);
  const selectedMaximumCatalog = JSON.parse(selectedMaximumResult.stdout);
  assert.equal(selectedMaximumCatalog.sceneCount, 622);
  assert.equal(selectedMaximumCatalog.selectedSceneCount, 28);
  assert.deepEqual([...selectedMaximumCatalog.selectedSceneIDs].sort(), [...maximumIDs].sort());
  const neighborIDs = neighborFixtures.map(({ id }) => id);
  assert.deepEqual(neighborIDs, [
    'col-fill-both', 'row-fill-both', 'col-fill-prefix', 'row-fill-suffix',
    'col-fit-first', 'row-fit-first', 'col-fit-empty', 'row-fit-empty',
    'col-fill-overflow', 'row-fill-overflow', 'col-fill-zero-prefix',
    'col-fill-border-box', 'row-fill-border-box',
  ]);
  assert.deepEqual(neighborIDs.filter((id) => !captureIDs.has(id)), [],
    'every percentage-neighbor fixture must be present in the simulator capture route.');
  for (const id of neighborIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  assert.ok(captureSource.includes('function autoRepeatPercentageNeighborScenes()'),
    'percentage-neighbor fixtures must flow through the fixed simulator capture route.');
  assert.ok(captureSource.includes('oracleScene.chromeObservedTracksPx ?? oracleScene.usedTracksPx'),
    'Chrome auto-fit values and normative used-track values must be checked separately.');
  assert.ok(android.includes('c114PercentageNeighborPixelSamplesMatch'),
    'Android must validate the declared percentage-neighbor PixelCopy sample locations.');
  assert.ok(android.includes('isC114PercentageNeighborPixelSceneId(pixelMatrixSceneId)'),
    'Android PixelCopy preflight must admit the percentage-neighbor scene IDs.');
  assert.ok(android.includes('return c114PercentageNeighborRootSize(sceneId) != null;'),
    'Android percentage-neighbor PixelCopy admission must use the fixed scene table.');
  assert.match(android,
    /int\[\]\[\] percentageNeighborItems = c114PercentageNeighborItemFrames\(sceneId\);\s*if \(percentageNeighborItems != null\) return percentageNeighborItems\.length \+ 1;/,
    'Android runtime must derive expected box count from the percentage-neighbor item table.');
  assert.match(ios,
    /if let percentageNeighborItems = c114PercentageNeighborItemFrames\(sceneID\) \{\s*return percentageNeighborItems\.count \+ 1\s*\}/,
    'iOS runtime must derive expected box count from the percentage-neighbor item table.');
  assert.ok(ios.includes('c114PercentageNeighborPixelSamples'),
    'iOS must validate the declared percentage-neighbor Metal sample locations.');
  const selectedNeighborResult = spawnSync(process.execPath, [
    resolve(repositoryRoot, 'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'),
    'android-avd', `--scene-id=${neighborIDs.join(',')}`, '--list-scenes',
  ], { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selectedNeighborResult.status, 0, selectedNeighborResult.stderr);
  const selectedNeighborCatalog = JSON.parse(selectedNeighborResult.stdout);
  assert.equal(selectedNeighborCatalog.sceneCount, 622);
  assert.equal(selectedNeighborCatalog.selectedSceneCount, 13);
  assert.deepEqual([...selectedNeighborCatalog.selectedSceneIDs].sort(), [...neighborIDs].sort());
  assert.deepEqual(
    fixtures.map(({ id }) => id).filter((id) => id.startsWith('template-area-repeat-two-axis-')),
    expectedIDs,
  );
  const overflow = fixtures.find(({ id }) => id === 'template-area-repeat-two-axis-area-added-both');
  assert.deepEqual(overflow.expect.root, [40, 40]);
  assert.deepEqual(overflow.expect.items, [{ name: 'hero', frame: [44, 40, 8, 8] }]);
  assert.deepEqual(overflow.expect.tracks, {
    columns: [20, 16, 16],
    rows: [20, 12, 12],
  });
  const areaExpandedSpan = fixtures.find(({ id }) =>
    id === 'template-area-repeat-two-axis-fit-fit-area-span');
  assert.deepEqual(areaExpandedSpan.expect.scroll, [64, 60]);
  assert.deepEqual(areaExpandedSpan.expect.items, [{ name: 'hero', frame: [0, 0, 64, 60] }]);
  const fitRowsFill = fixtures.find(({ id }) =>
    id === 'template-area-repeat-two-axis-fit-fill-area-item');
  assert.deepEqual(fitRowsFill.expect.items, [{ name: 'hero', frame: [0, 48, 16, 12] }]);
  const fillColumnsFit = fixtures.find(({ id }) =>
    id === 'template-area-repeat-two-axis-fill-fit-area-item');
  assert.deepEqual(fillColumnsFit.expect.items, [{ name: 'hero', frame: [48, 0, 16, 12] }]);
  const expectedAreaExpansion = [
    ['fit-fit-area-empty', [0, 0, 16], [0, 0, 12], [], [52, 52]],
    ['fit-fit-area-item', [0, 0, 16], [0, 0, 12], [[0, 0, 16, 12]], [52, 52]],
    ['fit-fit-area-span', [20, 20, 16], [20, 20, 12], [[0, 0, 64, 60]], [64, 60]],
    ['fit-fill-area-item', [0, 0, 16], [20, 20, 12], [[0, 48, 16, 12]], [52, 60]],
    ['fill-fit-area-item', [20, 20, 16], [0, 0, 12], [[48, 0, 16, 12]], [64, 52]],
    ['fit-fit-area-column', [0, 0, 16], [20, 0], [[0, 0, 16, 20]], [52, 52]],
    ['fit-fit-area-row', [20, 0], [0, 0, 12], [[0, 0, 20, 12]], [52, 52]],
  ];
  for (const [suffix, columns, rows, itemFrames, scroll] of expectedAreaExpansion) {
    const id = `template-area-repeat-two-axis-${suffix}`;
    const scene = fixtures.find((fixture) => fixture.id === id);
    assert.ok(scene, `missing ${id}`);
    assert.deepEqual(scene.expect.root, [52, 52], `${id} root`);
    assert.deepEqual(scene.expect.scroll, scroll, `${id} scroll`);
    assert.deepEqual(scene.expect.tracks, { columns, rows }, `${id} used tracks`);
    assert.deepEqual(scene.expect.items.map(({ frame }) => frame), itemFrames, `${id} items`);
  }
  for (const suffix of ['-fit-fit-area-span', '-fit-fill-area-item', '-fill-fit-area-item']) {
    assert.ok(captureSource.includes(`id.endsWith('${suffix}')`),
      `simulator capture is missing overflow samples for ${suffix}`);
  }
  for (const id of expectedIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(ios.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  const extractSceneIDs = (source, expression, label) => {
    const match = source.match(expression);
    assert.ok(match, `${label} allowlist is missing`);
    return new Set([...match[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]));
  };
  const androidBaseSceneIDs = extractSceneIDs(android,
    /C114_GRID_TRACK_SIZING_SCENE_IDS\s*=\s*\{([\s\S]*?)\};/,
    'Android Grid');
  const iosBaseSceneIDs = extractSceneIDs(ios,
    /c114GridTrackSizingSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS Grid');
  const iosCrossAxisSceneIDs = extractSceneIDs(ios,
    /c114CrossAxisFixedAutoRepeatAreaSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]/,
    'iOS cross-axis fixture');
  const androidSurfaceCapture = await readFile(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/SurfaceViewPixelCapture.java'),
  'utf8');
  assert.match(ios,
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1\s*private let c114CrossAxisFixedAutoRepeatAreaSceneIDs/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\) && SPINON_ENABLE_C04_RUNTIME_GPU && \\\s*defined\(SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST\) && \\\s*SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST\s*\/\* C11\.4 고정 track-sizing scene host는 앱 fixture-test 빌드 전용입니다\. \*\/\s*SpinonRuntimeGpuHost \*spinon_runtime_gpu_host_new_c11_4_grid_track_sizing_fixture\(/,
    'C11.4 fixture host declaration must use the native build flag that gates its symbol.');
  assert.match(ios,
    /let checkMiddleCollapsedAutoFitSharedGutter = sceneID =[\s\S]*?precedingSize\.doubleValue - 4\.0[\s\S]*?followingSize\.doubleValue\)/,
    'iOS trace validation must preserve the single shared 4px gap around the middle-collapsed track.');
  assert.match(android,
    /c114CrossAxisTemplateAreaSharedCollapsedTrackGutterMatches[\s\S]*?Math\.abs\(precedingSize - 4\.0\)[\s\S]*?Math\.abs\(followingSize\)/,
    'Android trace validation must preserve the single shared 4px gap around the middle-collapsed track.');
  assert.match(android,
    /if \(sceneId != null && sceneId\.startsWith\(C114_CROSS_AXIS_TEMPLATE_AREA_SCENE_PREFIX\)\)\s*\{\s*return BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&& c114CrossAxisTemplateAreaCanonicalSceneId\(sceneId\) != null;/);
  assert.match(android,
    /if \(sampleId\.startsWith\("template-area-repeat-cross-axis-"\)\)\s*\{\s*if \(sampleId\.endsWith\("-root-interior"\)\) return new int\[\] \{229, 231, 235\};\s*if \(sampleId\.endsWith\("-before-overflow-item"\)\s*\|\| sampleId\.endsWith\("-after-overflow-item"\)\)\s*\{\s*return new int\[\] \{0, 0, 0\};/,
    'Android cross-axis PixelCopy colors must include unobscured root and clear overflow samples.');
  assert.match(android, /targetNodeId\.matches\("\[a-z0-9-\]\{1,128\}"\)/);
  assert.match(android, /id\.matches\("\[a-z0-9-\]\{1,128\}"\)/);
  assert.match(androidSurfaceCapture, /caseId\.matches\("\[a-z0-9-\]\{1,128\}"\)/);
  assert.match(ios, /sample\.id\.range\(of: "\^\[a-z0-9-\]\{1,128\}\$"/);
  assert.match(ios, /sample\.nodeId\.range\(of: "\^\[a-zA-Z0-9_-\]\{1,128\}\$"/);
  for (const { id } of crossAxisFixtures) {
    const canonicalID = id.slice('template-area-repeat-cross-axis-'.length);
    assert.ok(!androidBaseSceneIDs.has(id), `Android base allowlist must not expose ${id}`);
    assert.ok(!iosBaseSceneIDs.has(id), `iOS base allowlist must not expose ${id}`);
    assert.ok(iosCrossAxisSceneIDs.has(id), `iOS gated allowlist is missing ${id}`);
    assert.ok(android.includes(`case "${canonicalID}":`),
      `Android cross-axis route is missing ${canonicalID}`);
    assert.ok(ios.includes(`case "${canonicalID}":`),
      `iOS cross-axis route is missing ${canonicalID}`);
  }
  const hostMappings = [
    ['Android', android, [
      ['root frame', 'private static String c114TemplateAreaRepeatRootFrame',
        'private static String c114TemplateAreaRepeatItemFrame'],
      ['item names', 'private static String[] c114TemplateAreaRepeatItemNames',
        'private static int[][] c114TemplateAreaRepeatItemRects'],
      ['item rects', 'private static int[][] c114TemplateAreaRepeatItemRects',
        'private static int[] c114TemplateAreaRepeatRootSize'],
    ]],
    ['iOS', ios, [
      ['root frame', 'private func c114TemplateAreaRepeatRootFrame',
        'private func c114TemplateAreaRepeatItemFrame'],
      ['item names', 'private func c114TemplateAreaRepeatItemNames',
        'private func c114TemplateAreaRepeatItemRects'],
      ['item rects', 'private func c114TemplateAreaRepeatItemRects',
        'private func c114TemplateAreaRepeatNodeFrames'],
    ]],
  ];
  for (const [platform, source, mappings] of hostMappings) {
    for (const [mappingName, startMarker, endMarker] of mappings) {
      const start = source.indexOf(startMarker);
      const end = source.indexOf(endMarker, start + startMarker.length);
      assert.ok(start >= 0 && end > start,
        `${platform} ${mappingName} helper boundary is missing`);
      const mapping = source.slice(start, end);
      for (const { id } of fixtures) {
        const expectedFragment = id.startsWith('template-area-repeat-cross-axis-')
          ? id.slice('template-area-repeat-cross-axis-'.length) : id;
        const hasCanonicalID = mapping.includes(`"${expectedFragment}"`);
        const hasCrossAxisRouter = id.startsWith('template-area-repeat-cross-axis-')
          && (mapping.includes('c114CrossAxisTemplateAreaCanonicalSceneId')
            || mapping.includes('c114CrossAxisFixedAutoRepeatAreaCanonicalSceneID')
            || mapping.includes('c114CrossAxisFixedAutoRepeatAreaExpectation'));
        assert.ok(hasCanonicalID || hasCrossAxisRouter,
          `${platform} ${mappingName} helper is missing ${expectedFragment}`);
      }
    }
  }
});

test('fit-content() 열 축 limit 네 장면이 opt-in runtime과 두 simulator에 연결된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const sceneIDs = [
    'column-fit-content-40', 'column-fit-content-30', 'column-fit-content-200',
    'column-fit-content-auto-minimum-control',
  ];
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${sceneIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 4);
  assert.deepEqual(catalog.selectedSceneIDs, sceneIDs);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-limit-columns-simulators-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-limit-columns-/);

  const files = [
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs',
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_limit_columns.rs',
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_limit_columns_report.rs',
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_limit_columns.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_limit_columns/validation.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_limit_columns/style_validation.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_limit_columns/trace_validation.rs',
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  ];
  const [capture, ...contents] = await Promise.all([
    readFile(capturePath, 'utf8'),
    ...files.map((path) => readFile(resolve(repositoryRoot, path), 'utf8')),
  ]);
  const [route, source, report, traceReport, runtime, runtimeModules,
    runtimeFixture, runtimeValidation, runtimeStyleValidation, runtimeTraceValidation,
    android, ios] = contents;
  assert.ok(sceneIDs.every((sceneID) => capture.includes(sceneID)));
  assert.ok(capture.includes('fitContentLimitColumns'));
  assert.ok(capture.includes('child-overflows-item'));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_limit_columns\.rs"\]\s*mod fit_content_limit_columns;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_limit_columns_report\.rs"\]/);
  for (const sourceText of [report, runtime, runtimeModules, runtimeFixture,
    runtimeValidation, traceReport]) {
    assert.ok(sourceText.includes('fit_content_limit_columns'),
      'runtime path is missing the fit-content column limit fixture name');
  }
  assert.ok(source.includes('grid-auto-flow:column'));
  assert.ok(source.includes('grid-auto-columns:fit-content('));
  assert.ok(!source.includes('overflow:visible'),
    'the initial visible overflow must not be emitted as an unsupported declaration');
  assert.ok(runtimeTraceValidation.includes('MaximizeTracks'));
  assert.ok(runtimeTraceValidation.includes('growth_limit'));
  assert.ok(runtimeStyleValidation.includes('computed root style changed'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentLimitColumnsTraceMatches'],
    ['iOS', ios, 'c114FitContentLimitColumnsMatches'],
  ]) {
    assert.ok(sceneIDs.every((sceneID) => app.includes(sceneID)),
      `${platform} scene allowlist is incomplete`);
    assert.ok(app.includes(matcher), `${platform} strict column trace validation is missing`);
    assert.ok(app.includes('child-overflows-item'), `${platform} overflow pixel sample is missing`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});

test('CSS reference 도구가 삭제된 Rust fixture 경로를 참조하지 않는다', async () => {
  const entries = await readdir(resolve(repositoryRoot, 'tools/css-reference'));
  const sources = await Promise.all(entries
    .filter((entry) => entry.endsWith('.mjs') && entry !== 'fixture-module-boundary.test.mjs')
    .map((entry) => readFile(resolve(repositoryRoot, 'tools/css-reference', entry), 'utf8')));
  const staleReferences = legacyFixtureSources.filter((legacyPath) =>
    sources.some((source) => source.includes(legacyPath)));

  assert.deepEqual(staleReferences, []);
});

test('이동한 fixture 구현은 정해진 fixtures 모듈 경로에 있다', () => {
  for (const relativePath of canonicalFixtureSources) {
    assert.equal(
      existsSync(resolve(repositoryRoot, relativePath)),
      true,
      `정식 fixture 모듈 경로가 없습니다: ${relativePath}`,
    );
  }
});

test('C11.4 cyclic row auto-repeat 장면과 양 플랫폼 검사가 feature fixture에 고정된다', async () => {
  const runtimeGpu = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu.rs'), 'utf8',
  );
  const fixtureSource = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/cyclic_percentage_auto_repeat_rows.rs'),
    'utf8',
  );
  const c114Dispatcher = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8',
  );
  const android = await readFile(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8');
  const ios = await readFile(resolve(repositoryRoot,
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8');
  const capture = await readFile(resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs'), 'utf8');
  const sceneIDs = [
    'cyclic-max-content-auto-fill-prefix-suffix',
    'cyclic-max-content-auto-fit-prefix-suffix',
    'definite-44px-auto-fill-prefix-suffix-control',
    'definite-44px-auto-fit-prefix-suffix-control',
  ];
  assert.match(runtimeGpu,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(c114Dispatcher,
    /#\[path = "c11_4\/cyclic_percentage_auto_repeat_rows\.rs"\]\s*mod cyclic_percentage_auto_repeat_rows;/);
  assert.match(c114Dispatcher,
    /cyclic_percentage_auto_repeat_rows::runtime_source\(scene_id\)/);
  for (const sceneID of sceneIDs) {
    assert.ok(fixtureSource.includes(`"${sceneID}"`), `FFI fixture is missing ${sceneID}`);
    assert.ok(android.includes(`"${sceneID}"`), `Android scene route is missing ${sceneID}`);
    assert.ok(ios.includes(`"${sceneID}"`), `iOS scene route is missing ${sceneID}`);
    assert.ok(capture.includes(`'${sceneID}'`), `simulator capture route is missing ${sceneID}`);
  }
  for (const token of ['"max-content"', '"44px"', '"auto-fill"', '"auto-fit"',
    'row-gap:2px', 'height:{root_height};grid-template-columns:24px;grid-template-rows:',
    'width:24px;height:24px', 'width:16px;height:16px']) {
    assert.ok(fixtureSource.includes(token), `FFI generalized row profile is missing ${token}`);
  }
  assert.ok(!fixtureSource.includes('height:{root_height};min-height:'),
    'root min-height must remain the typed Auto initial value.');
  for (const sampleID of [
    'generalized-prefix', 'generalized-secondary-overlap',
    'generalized-primary-overlap', 'generalized-primary-overflow',
    'generalized-root-background',
  ]) {
    assert.ok(android.includes(`"${sampleID}"`), `Android pixel sample is missing ${sampleID}`);
    assert.ok(ios.includes(`"${sampleID}"`), `iOS pixel sample is missing ${sampleID}`);
    assert.ok(capture.includes(`'${sampleID}'`), `simulator pixel sample is missing ${sampleID}`);
  }
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${sceneIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 4);
  assert.deepEqual(catalog.selectedSceneIDs, sceneIDs);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-cyclic-row-auto-repeat-generalization-runtime-\d{4}-\d{2}-\d{2}\/android-avd-/);
  assert.ok(android.includes('c114GeneralizedCyclicRowTraceMatches'));
  assert.ok(ios.includes('c114GeneralizedCyclicRowAutoRepeatTraceMatches'));
});

test('C11.4 zero fixed-maximum auto-repeat 24 scene은 opt-in runtime에만 연결된다', async () => {
  const scenePath = resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_zero_fixed_maximum_scenes.json');
  const fixtures = JSON.parse(await readFile(scenePath, 'utf8'));
  const sceneIDs = fixtures.map(({ id }) => id);
  assert.equal(sceneIDs.length, 24);
  assert.equal(new Set(sceneIDs).size, 24);

  const android = await readFile(resolve(repositoryRoot,
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8');
  const ios = await readFile(resolve(repositoryRoot,
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8');
  const runtimeFixtures = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures.rs'), 'utf8');
  const runtimeDispatcher = await readFile(resolve(repositoryRoot,
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8');
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const capture = await readFile(capturePath, 'utf8');

  assert.match(runtimeFixtures,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_auto_repeat_zero_fixed_maximum;/);
  assert.match(runtimeDispatcher,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*if let Some\(source\) = super::c11_4_auto_repeat_zero_fixed_maximum::runtime_source\(scene_id\)/);

  const androidAllowlist = android.match(
    /private static final String\[\] C114_ZERO_FIXED_MAXIMUM_PIXEL_SCENE_IDS\s*=\s*BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*\?\s*new String\[\]\s*\{([\s\S]*?)\}\s*:\s*new String\[0\];/,
  );
  assert.ok(androidAllowlist, 'Android scene payload must be compile-time gated by the fixture flag.');
  assert.deepEqual([...androidAllowlist[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]), sceneIDs);
  const androidRoute = android.slice(
    android.indexOf('private static boolean isC114ZeroFixedMaximumPixelSceneId'),
    android.indexOf('private static String c114ZeroFixedMaximumRootFrame'),
  );
  assert.match(androidRoute,
    /return BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&& java\.util\.Arrays\.asList\(C114_ZERO_FIXED_MAXIMUM_PIXEL_SCENE_IDS\)\.contains\(sceneId\);/);

  const iosAllowlist = ios.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1\s+private let c114ZeroFixedMaximumPixelSceneIDs:\s*Set<String>\s*=\s*\[([\s\S]*?)\]\s+#else\s+private let c114ZeroFixedMaximumPixelSceneIDs:\s*Set<String>\s*=\s*\[\]\s+#endif/,
  );
  assert.ok(iosAllowlist, 'iOS scene payload must be compile-time gated by the fixture flag.');
  assert.deepEqual([...iosAllowlist[1].matchAll(/"([^"]+)"/g)].map((entry) => entry[1]), sceneIDs);

  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${sceneIDs.join(',')}`, '--run-kind=zero-fixed-maximum', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 24);
  assert.deepEqual(catalog.selectedSceneIDs, sceneIDs);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-auto-repeat-zero-fixed-maximum-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-zero-fixed-maximum-/);
  assert.ok(capture.includes('scenes.push(...await autoRepeatZeroFixedMaximumScenes())'));
});

test('작업별 FFI fixture 모듈은 runtime 소스 루트에 남지 않는다', async () => {
  const misplaced = [];
  const queue = [['crates/spinon-ffi/src', '']];
  while (queue.length > 0) {
    const [absoluteDirectory, relativeDirectory] = queue.pop();
    const entries = await readdir(resolve(repositoryRoot, absoluteDirectory), { withFileTypes: true });
    for (const entry of entries) {
      const relativePath = relativeDirectory ? `${relativeDirectory}/${entry.name}` : entry.name;
      if (entry.isDirectory()) {
        queue.push([`${absoluteDirectory}/${entry.name}`, relativePath]);
      } else if (
        entry.isFile()
        && /^c\d+(?:_\d+)*\.rs$/.test(entry.name)
        && !relativePath.split('/').includes('fixtures')
      ) {
        misplaced.push(`${absoluteDirectory}/${entry.name}`);
      }
    }
  }
  assert.deepEqual(misplaced, [], `fixture 모듈을 fixtures/ 아래로 옮기세요: ${misplaced.join(', ')}`);
});

test('앱 fixture adapter와 include 데이터는 opt-in Cargo feature에 격리된다', async () => {
  const runtimeSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu.rs'),
    'utf8',
  );
  const ffiSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi.rs'),
    'utf8',
  );
  const runtimeFixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures.rs'),
    'utf8',
  );
  const c114RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'),
    'utf8',
  );
  const c114NamedLineRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_named_lines.rs'),
    'utf8',
  );
  const c114TemplateAreaRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_template_area_auto_repeat.rs'),
    'utf8',
  );
  const c114CalcMinimumRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_calc_minimum.rs'),
    'utf8',
  );
  const c114PositivePercentRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_positive_percent_minimum.rs'),
    'utf8',
  );
  const c114ZeroPercentRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_zero_percent_minimum.rs'),
    'utf8',
  );
  const c114PercentageMaximumRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_percentage_maximum.rs'),
    'utf8',
  );
  const c114PercentageNeighborRuntimeFixture = await readFile(
    resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4_auto_repeat_percentage_neighbors.rs'),
    'utf8',
  );
  const c114RuntimeFixtureHost = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'),
    'utf8',
  );
  const ffiFixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs'),
    'utf8',
  );
  const c13FixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_1.rs'),
    'utf8',
  );
  const c13FfiFixtureSource = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_1.rs'),
    'utf8',
  );
  const c13BorderRuntimeFixture = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c13/runtime-overflow-clip-hit-border-c13-1.js'),
    'utf8',
  );
  const c13ScrollHostFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll_host.rs'),
    'utf8',
  );
  const c13ScrollFfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_2.rs'),
    'utf8',
  );
  const c13ScrollRuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_2_scroll.rs'),
    'utf8',
  );
  const c13GestureRuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture.rs'),
    'utf8',
  );
  const c13GestureHostFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c13_3_scroll_gesture_host.rs'),
    'utf8',
  );
  const c13GestureFfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c13_3.rs'),
    'utf8',
  );
  const runtimeFfiHeader = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/include/spinon_ffi.h'),
    'utf8',
  );
  const c13GestureRuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c13/runtime-scroll-gesture-c13-3.js'),
    'utf8',
  );
  const c18RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_1.rs'),
    'utf8',
  );
  const c182RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_2.rs'),
    'utf8',
  );
  const c182FfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_2.rs'),
    'utf8',
  );
  const c184RuntimeFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4.rs'),
    'utf8',
  );
  const c184FixtureImplementation = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c18_4/fixture.rs'),
    'utf8',
  );
  const c184FfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_4.rs'),
    'utf8',
  );
  const c18FfiFixture = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c18_1.rs'),
    'utf8',
  );
  const c18RuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c18/runtime-image-overflow-c18-1.js'),
    'utf8',
  );
  const c182RuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c18/runtime-image-alpha-c18-2.js'),
    'utf8',
  );
  const c184RuntimeScript = await readFile(
    resolve(repositoryRoot, 'tests/fixtures/css/c18/runtime-image-async-replacement-c18-4.js'),
    'utf8',
  );
  const cargoManifest = await readFile(
    resolve(repositoryRoot, 'crates/spinon-ffi/Cargo.toml'),
    'utf8',
  );
  const androidJniSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/cpp/spinon_jni.cc'),
    'utf8',
  );
  const androidGradleSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/build.gradle.kts'),
    'utf8',
  );
  const androidActivitySource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    'utf8',
  );
  const androidGpuDemoSource = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    'utf8',
  );
  const androidImagePaintCaptureAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_1_image_paint_fixture_test/java/dev/spinon/bootstrap/ImagePaintFixtureCaptureAdapter.java'),
    'utf8',
  );
  const androidImagePaintNativeAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_1_image_paint_fixture_test/java/dev/spinon/bootstrap/ImagePaintFixtureNativeAdapter.java'),
    'utf8',
  );
  const androidAlphaImageCaptureAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_2_alpha_image_fixture_test/java/dev/spinon/bootstrap/AlphaImageFixtureCaptureAdapter.java'),
    'utf8',
  );
  const androidAsyncImageCaptureAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_4_async_image_fixture_test/java/dev/spinon/bootstrap/AsyncImageFixtureCaptureAdapter.java'),
    'utf8',
  );
  const androidAsyncImageNativeAdapter = await readFile(
    resolve(repositoryRoot, 'platforms/android/app/src/c18_4_async_image_fixture_test/java/dev/spinon/bootstrap/AsyncImageFixtureNativeAdapter.java'),
    'utf8',
  );
  const androidBuildScript = await readFile(
    resolve(repositoryRoot, 'tools/build-android.sh'),
    'utf8',
  );
  const iosRunnerHeader = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/SpinonRunner.h'),
    'utf8',
  );
  const iosRunnerImplementation = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/SpinonRunner.mm'),
    'utf8',
  );
  const iosGpuDemoSource = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
    'utf8',
  );
  const iosAppDelegateSource = await readFile(
    resolve(repositoryRoot, 'platforms/ios/Sources/AppDelegate.swift'),
    'utf8',
  );
  const iosBuildSimulatorScript = await readFile(
    resolve(repositoryRoot, 'tools/build-ios-sim.sh'),
    'utf8',
  );
  const iosProject = await readFile(
    resolve(repositoryRoot, 'platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj'),
    'utf8',
  );
  assert.match(runtimeSource, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(ffiSource, /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(cargoManifest, /c04-runtime-gpu\s*=\s*\[[^\]]*"dep:spinon-render-wgpu"[^\]]*\]/);
  const runtimeGpuFixtureFeature = cargoManifest.match(
    /^c04-runtime-gpu-fixture-test\s*=\s*\[([\s\S]*?)^\]/m,
  );
  assert.ok(runtimeGpuFixtureFeature, 'Runtime GPU fixtures stay behind the fixture-test feature');
  assert.match(runtimeGpuFixtureFeature[1], /"c04-runtime-gpu"/);
  assert.match(runtimeGpuFixtureFeature[1], /"spinon-runtime\/c11-4-grid-track-sizing-fixture-test"/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-1-overflow-clip-fixture-test"\)\]\s*pub\(crate\) mod c13_1;/);
  assert.match(runtimeFixtureSource,
    /pub\(crate\) mod c11_4_auto_repeat_named_lines;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_template_area_auto_repeat;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_auto_repeat_calc_minimum;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_auto_repeat_percentage_maximum;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*pub\(crate\) mod c11_4_auto_repeat_percentage_neighbors;/);
  assert.match(c114RuntimeFixtureHost,
    /#\[path = "c11_4_auto_repeat_positive_percent_minimum\.rs"\]\s*mod auto_repeat_positive_percent_minimum;/);
  assert.match(c114RuntimeFixtureHost,
    /super::c11_4_template_area_auto_repeat::runtime_source\(scene_id\)/);
  assert.match(c114RuntimeFixtureHost,
    /super::c11_4_auto_repeat_calc_minimum::runtime_source\(scene_id\)/);
  assert.match(c114RuntimeFixtureHost,
    /auto_repeat_positive_percent_minimum::runtime_source\(scene_id\)/);
  assert.match(c114RuntimeFixtureHost,
    /#\[path = "c11_4_auto_repeat_zero_percent_minimum\.rs"\]\s*mod auto_repeat_zero_percent_minimum;/);
  assert.match(c114RuntimeFixtureHost,
    /auto_repeat_zero_percent_minimum::runtime_source\(scene_id\)/);
  assert.match(c114RuntimeFixtureHost,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*if let Some\(source\) = super::c11_4_auto_repeat_percentage_maximum::runtime_source\(scene_id\)/);
  assert.match(c114RuntimeFixtureHost,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*if let Some\(source\) = super::c11_4_auto_repeat_percentage_neighbors::runtime_source\(scene_id\)/);
  assert.match(c114TemplateAreaRuntimeFixture,
    /include_str!\("c11_4\/template_area_auto_repeat_scenes\.json"\)/);
  assert.match(c114CalcMinimumRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_calc_minimum_scenes\.json"\)/);
  assert.match(c114PositivePercentRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_positive_percent_minimum_scenes\.json"\)/);
  assert.match(c114ZeroPercentRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_zero_percent_minimum_scenes\.json"\)/);
  assert.match(c114PercentageMaximumRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_percentage_maximum_scenes\.json"\)/);
  assert.match(c114PercentageNeighborRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_percentage_neighbor_scenes\.json"\)/);
  assert.match(c114RuntimeFixture,
    /super::c11_4_auto_repeat_named_lines::runtime_source/);
  assert.match(c114NamedLineRuntimeFixture,
    /include_str!\("c11_4\/auto_repeat_named_line_scenes\.json"\)/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c13-1-overflow-clip-fixture-test"\)\]\s*mod c13_1;/);
  assert.match(cargoManifest,
    /c13-1-overflow-clip-fixture-test\s*=\s*\["c04-runtime-gpu-fixture-test",\s*"dep:sha2"\]/);
  assert.match(c13FixtureSource,
    /RUNTIME_CSS_C13_1_BORDER_FIXTURE_SOURCE[\s\S]*runtime-overflow-clip-hit-border-c13-1\.js/);
  assert.match(c13FfiFixtureSource,
    /spinon_runtime_gpu_host_new_c13_1_overflow_clip_border_fixture/);
  assert.match(c13FfiFixtureSource,
    /spinon_runtime_gpu_host_eval_c13_1_overflow_clip_border_fixture/);
  assert.match(c13BorderRuntimeFixture,
    /data-c131-border-node/);
  assert.match(androidGpuDemoSource,
    /nativeCreateC131OverflowClipBorderHost\(\)/);
  assert.match(androidGpuDemoSource,
    /nativeEvalC131OverflowClipBorderFixture\(long host\)/);
  assert.match(androidJniSource,
    /Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeCreateC131OverflowClipBorderHost/);
  assert.match(androidJniSource,
    /Java_dev_spinon_bootstrap_C0410RuntimeGpuDemo_nativeEvalC131OverflowClipBorderFixture/);
  assert.match(androidActivitySource,
    /spinon_c131_overflow_clip_border/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-2-scroll-fixture-test"\)\]\s*pub\(crate\) mod c13_2_scroll;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-2-scroll-fixture-test"\)\]\s*mod c13_2_scroll_host;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c13-2-scroll-fixture-test"\)\]\s*mod c13_2;/);
  assert.match(cargoManifest,
    /c13-2-scroll-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test",\s*"dep:spinon-core",[\s\S]*?"spinon-runtime\/c19-2-box-paint-opacity-fixture-test"[\s\S]*?\]/);
  assert.match(c13ScrollRuntimeFixture,
    /runtime-scroll-container-c13-2\.js/);
  assert.match(c13ScrollHostFixture,
    /border_escape_outside_parent=true[\s\S]*border_escape_hit=matched/);
  assert.match(c13ScrollFfiFixture,
    /spinon_runtime_gpu_host_new_c13_2_scroll_fixture/);
  assert.match(c13ScrollFfiFixture,
    /spinon_runtime_gpu_host_eval_c13_2_scroll_fixture/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\?\s*"SPINON · C13\.2 scroll state"/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\?\s*"V8 scroll owner·offset·paint\/hit → WGPU · 5 PixelCopy 표본 · 400×240 CSS px"/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\?\s*"C13\.2 scroll owner·offset·paint\/hit WGPU 장면"/);
  assert.match(androidGpuDemoSource,
    /c132ScrollFixtureRequested\s*\|\|\s*c133GestureFixtureRequested\s*\?\s*400/);
  assert.match(androidGpuDemoSource,
    /if \(c132ScrollFixtureRequested \|\| c133GestureFixtureRequested\) return 400\.0f;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-3-gesture-fixture-test"\)\]\s*pub\(crate\) mod c13_3_scroll_gesture;/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c13-3-gesture-fixture-test"\)\]\s*pub\(super\) mod c13_3_scroll_gesture_host;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c13-3-gesture-fixture-test"\)\]\s*pub\(super\) mod c13_3;/);
  assert.match(cargoManifest,
    /c13-3-gesture-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-core"[\s\S]*?"spinon-render\/c13-3-gesture-fixture-test"[\s\S]*?"spinon-runtime\/c13-3-gesture-fixture-test"[\s\S]*?\]/);
  assert.match(c13GestureRuntimeFixture,
    /RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE[\s\S]*runtime-scroll-gesture-c13-3\.js/);
  assert.match(c13GestureHostFixture,
    /C13_3GestureFixtureScenario[\s\S]*RUNTIME_CSS_C13_3_SCROLL_GESTURE_SOURCE/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_new_c13_3_gesture_fixture/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_eval_c13_3_gesture_fixture/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_enqueue_c13_3_gesture/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.match(c13GestureFfiFixture,
    /spinon_runtime_gpu_host_process_c13_3_gesture_queue/);
  assert.match(c13GestureRuntimeScript,
    /data-c133-gesture-node/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c13-3-gesture-fixture-test\)/);
  assert.match(androidBuildScript,
    /c133_gesture_fixture_cpp_flag=.*SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST=.*SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c13_3_gesture_fixture/);
  assert.match(androidGpuDemoSource,
    /nativeCreateC133GestureHost\(int scenarioId\)/);
  assert.match(androidGpuDemoSource,
    /nativeProcessC133GestureQueue\(long host\)/);
  assert.match(androidGpuDemoSource,
    /nativeEnqueueC133Wheel\([\s\S]*?sourceUnit, float coordinateScale, int source/);
  assert.match(androidGpuDemoSource,
    /KeyEvent\.normalizeMetaState\(metaState\)/);
  assert.match(androidGpuDemoSource,
    /normalizedMetaState & KeyEvent\.META_CTRL_ON/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.doesNotMatch(androidGpuDemoSource, /SPINON_C133_SURFACE_BOUNDS/);
  assert.match(androidGpuDemoSource,
    /new StringBuilder\("SPINON_C133_SURFACE"\)[\s\S]*?append\("_BOUNDS"\)/);
  assert.doesNotMatch(androidGpuDemoSource, /SPINON_C133_STYLE_UPDATE /);
  assert.doesNotMatch(androidGpuDemoSource, /c13_3_style_invalidation=applied/);
  assert.match(androidGpuDemoSource,
    /\.append\("SPINON_C133_"\)[\s\S]*?\.append\("STYLE_UPDATE "\)/);
  assert.match(androidActivitySource,
    /spinon_c133_gesture_fixture/);
  assert.match(iosProject,
    /SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST=\$\(SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST\)/);
  assert.match(iosProject,
    /SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_\$\(SPINON_ENABLE_C13_3_GESTURE_FIXTURE_TEST\)/);
  assert.match(iosRunnerHeader,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?createRuntimeGpuHostWithC13_3GestureFixture/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c13_3_gesture_fixture/);
  assert.match(iosRunnerHeader,
    /enqueueRuntimeGpuC13_3Wheel:[\s\S]*?modifierFlags:/);
  assert.match(iosRunnerImplementation,
    /enqueueRuntimeGpuC13_3Wheel:[\s\S]*?spinon_runtime_gpu_host_enqueue_c13_3_wheel/);
  assert.match(iosGpuDemoSource,
    /#if SPINON_ENABLE_C133_GESTURE_FIXTURE_TEST_1[\s\S]*?withC13_3GestureFixture:/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c18-1-image-paint-fixture-test"\)\]\s*pub\(super\) mod c18_1;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c18-1-image-paint-fixture-test"\)\]\s*mod c18_1;/);
  assert.match(cargoManifest,
    /c18-1-image-paint-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-image"[\s\S]*?"spinon-runtime\/c14-1-live-v8-fixture-test"[\s\S]*?\]/);
  assert.match(c18RuntimeFixture,
    /RUNTIME_CSS_C18_1_IMAGE_PAINT_FIXTURE_SOURCE[\s\S]*runtime-image-overflow-c18-1\.js/);
  assert.match(c18RuntimeFixture,
    /include_bytes![\s\S]*c14-1-png-basic\.png/);
  assert.match(runtimeSource,
    /fn set_environment\([\s\S]*?#\[cfg\(feature = "c18-1-image-paint-fixture-test"\)\][\s\S]*?let attach_c18_1_images = self\.c18_1_image_paint_fixture[\s\S]*?c18_1_image_document_ready[\s\S]*?Ordering::Acquire[\s\S]*?self\.wait_and_publish\([\s\S]*?WaitAndPublishOptions\s*\{[\s\S]*?attach_c18_1_images,/);
  assert.match(c18FfiFixture,
    /spinon_runtime_gpu_host_new_c18_1_image_paint_fixture[\s\S]*spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test/);
  assert.match(c18RuntimeScript,
    /spinon-fixture:c18-1\/static-png-visible[\s\S]*spinon-fixture:c18-1\/static-png-hidden/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_request_c18_1_surface_pixels_for_test/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c18-1-image-paint-fixture-test\)/);
  assert.match(androidBuildScript,
    /c181_image_paint_cpp_flag=.*SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_1_image_paint_fixture/);
  assert.match(androidImagePaintCaptureAdapter,
    /SurfaceViewPixelCapture\.requestForCssViewport\(surfaceView,[\s\S]*?"c181-image-overflow"/);
  assert.match(androidImagePaintCaptureAdapter,
    /getLocationOnScreen\(contentOrigin\)[\s\S]*?surfaceView\.getLocationOnScreen\(surfaceOrigin\)[\s\S]*?SPINON_C181_SCREEN_GEOMETRY[\s\S]*?surface_view_size_px=/);
  assert.doesNotMatch(androidGpuDemoSource,
    /nativeRequestC181SurfacePixelsForTest/);
  assert.doesNotMatch(androidGpuDemoSource,
    /private\s+static\s+native\s+[^;]*(?:C181|c18_1)/i);
  assert.match(androidGpuDemoSource,
    /cssViewportWidthLocked\(\)\s*\{[\s\S]*?imagePaintProbeRequested\)\s*\{\s*return imageAsyncReplacementProbeRequested \|\| imageObjectFitPositionProbeRequested\s*\? 224\.0f : 80\.0f/);
  assert.match(androidGpuDemoSource,
    /cssViewportHeightLocked\(\)\s*\{[\s\S]*?imagePaintProbeRequested\)\s*\{\s*return imageAsyncReplacementProbeRequested \? 88\.0f\s*:\s*imageObjectFitPositionProbeRequested \? 72\.0f : 40\.0f/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_IMAGE_PAINT_FIXTURE_TEST[\s\S]*spinon_c181_image_paint/);
  assert.match(androidGradleSource,
    /buildConfigField\([\s\S]*?"SPINON_IMAGE_PAINT_FIXTURE_TEST"[\s\S]*?spinonC181ImagePaintFixtureTestValue/);
  assert.match(androidGradleSource,
    /if \(spinonImagePaintFixtureTestValue\)\s*\{\s*java\.srcDir\("src\/c18_1_image_paint_fixture_test\/java"\)/);
  assert.match(androidImagePaintCaptureAdapter,
    /SPINON_C181_PIXELCOPY_BEGIN[\s\S]*SPINON_C181_PIXELCOPY_SUMMARY/);
  assert.match(androidImagePaintNativeAdapter,
    /native long createHost\(\);[\s\S]*native byte\[\] evalFixture\(long host\);/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST=\$\(SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST\)/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\)[\s\S]*?SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST[\s\S]*?spinon_runtime_gpu_host_new_c18_1_image_paint_fixture/);
  assert.match(iosGpuDemoSource,
    /imagePaintProbeRequested[\s\S]*?requestC181MetalPixelReadback/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c18-2-alpha-image-compositing-fixture-test"\)\]\s*pub\(super\) mod c18_2;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c18-2-alpha-image-compositing-fixture-test"\)\]\s*mod c18_2;/);
  assert.match(cargoManifest,
    /c18-2-alpha-image-compositing-fixture-test\s*=\s*\["c18-1-image-paint-fixture-test"\]/);
  assert.match(c182RuntimeFixture,
    /include_str![\s\S]*runtime-image-alpha-c18-2\.js[\s\S]*include_bytes![\s\S]*c18-2-alpha-source-over\.png/);
  assert.match(c182RuntimeFixture,
    /C18\.2 img src가 고정 입력과 다릅니다[\s\S]*C18\.2 img 수가 다릅니다/);
  assert.match(c182FfiFixture,
    /spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture[\s\S]*spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test/);
  assert.match(c182RuntimeScript,
    /spinon-fixture:c18-2\/alpha-source-over/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_request_c18_2_surface_pixels_for_test/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c18-2-alpha-image-compositing-fixture-test\)/);
  assert.match(androidBuildScript,
    /c182_alpha_image_cpp_flag=.*SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture/);
  assert.match(iosBuildSimulatorScript,
    /c18_image_paint_shared=\$\{c18_image_paint_shared\} c18_1_image_paint_fixture=\$\{SPINON_ENABLE_C18_1_IMAGE_PAINT_FIXTURE_TEST:-0\} c18_2_alpha_image_fixture=\$\{SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST:-0\}/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST=\$\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_2_ALPHA_IMAGE_FIXTURE_TEST_\$\(SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST\)/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\)[\s\S]*?SPINON_ENABLE_C18_2_ALPHA_IMAGE_COMPOSITING_FIXTURE_TEST[\s\S]*?spinon_runtime_gpu_host_new_c18_2_alpha_image_fixture/);
  assert.match(androidAlphaImageCaptureAdapter,
    /SPINON_C182_PIXELCOPY_SAMPLE[\s\S]*SPINON_C182_PIXELCOPY_SUMMARY[\s\S]*SPINON_C182_SCREEN_GEOMETRY/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_C18_2_ALPHA_IMAGE_FIXTURE_TEST[\s\S]*spinon_c182_alpha_image[\s\S]*imageAlphaFixtureRequested/);
  assert.match(iosAppDelegateSource,
    /#if SPINON_ENABLE_C18_IMAGE_PAINT_FIXTURE_TEST_1[\s\S]*runRuntimeGpu = runRuntimeGpuBase[\s\S]*--spinon-c181-image-paint[\s\S]*--spinon-c182-alpha-image[\s\S]*--spinon-c183-object-fit-position/);
  assert.match(runtimeFixtureSource,
    /#\[cfg\(feature = "c18-4-image-fixture-test"\)\]\s*pub\(super\) mod c18_4;/);
  assert.match(ffiFixtureSource,
    /#\[cfg\(feature = "c18-4-image-fixture-test"\)\]\s*mod c18_4;/);
  assert.match(cargoManifest,
    /c18-4-image-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-image"[\s\S]*?"dep:spinon-layout"[\s\S]*?\]/);
  assert.match(c184RuntimeFixture,
    /RUNTIME_CSS_C18_4_ASYNC_IMAGE_FIXTURE_SOURCE[\s\S]*runtime-image-async-replacement-c18-4\.js/);
  assert.match(c184FixtureImplementation,
    /include_bytes![\s\S]*c18-4-async-[abcd]-(?:slow|fast)\.png/);
  assert.match(c184FfiFixture,
    /spinon_runtime_gpu_host_new_c18_4_async_image_fixture[\s\S]*spinon_runtime_gpu_host_request_c18_4_surface_pixels_for_test/);
  assert.match(c184RuntimeScript,
    /spinon-fixture:c18-4\/b-fast[\s\S]*spinon-fixture:c18-4\/c-slow[\s\S]*spinon-fixture:c18-4\/broken[\s\S]*spinon-fixture:c18-4\/d-fast/);
  assert.match(runtimeFfiHeader,
    /#if defined\(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_4_async_image_fixture/);
  assert.match(androidBuildScript,
    /ffi_features\+=\(c18-4-image-fixture-test\)/);
  assert.match(androidBuildScript,
    /c184_async_image_cpp_flag=.*SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST/);
  assert.match(androidJniSource,
    /#if defined\(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c18_4_async_image_fixture/);
  assert.match(androidGradleSource,
    /if \(spinonC184AsyncImageFixtureTestValue == "1"\)\s*\{\s*java\.srcDir\("src\/c18_4_async_image_fixture_test\/java"\)/);
  assert.match(androidActivitySource,
    /BuildConfig\.SPINON_C18_4_ASYNC_IMAGE_FIXTURE_TEST[\s\S]*spinon_c184_async_image/);
  assert.match(androidAsyncImageNativeAdapter,
    /native long createHost\(\);[\s\S]*native byte\[\] evalFixture\(long host\);/);
  assert.match(androidAsyncImageCaptureAdapter,
    /SPINON_C184_PIXELCOPY_SAMPLE[\s\S]*SPINON_C184_PIXELCOPY_SUMMARY/);
  assert.match(iosBuildSimulatorScript,
    /c18_4_async_image_fixture=\$\{SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST:-0\}/);
  assert.match(iosProject,
    /SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST=\$\(SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST\)/);
  assert.match(iosRunnerImplementation,
    /#if defined\(SPINON_ENABLE_C04_RUNTIME_GPU\)[\s\S]*?SPINON_ENABLE_C18_4_ASYNC_IMAGE_FIXTURE_TEST[\s\S]*?spinon_runtime_gpu_host_new_c18_4_async_image_fixture/);
  assert.match(iosAppDelegateSource,
    /runC184ImageRoute = arguments\.contains\("--spinon-c184-async-image"\)[\s\S]*runC184ImageRoute/);
});

test('C15.3 glyph paint 앱 adapter와 고정 글꼴은 opt-in feature 경계에 묶인다', async () => {
  const read = (path) => readFile(resolve(repositoryRoot, path), 'utf8');
  const [runtimeModules, ffiModules, runtimeSource, ffiSource, cargoManifest,
    c15RuntimeFixture, c15HostDocumentFixture, c15HostDocumentLayout, c15HostDocumentRaster,
    c15CssFixture, c15FfiFixture, ffiHeader,
    androidBuild, androidGradle, androidActivity, androidDemo, androidJni, androidAdapter,
    iosBuild, iosProject, iosHeader,
    iosRunner, iosAppDelegate, iosDemo] = await Promise.all([
    read('crates/spinon-ffi/src/runtime_gpu/fixtures.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/ffi/fixtures.rs'),
    read('crates/spinon-ffi/src/runtime_gpu.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/ffi.rs'),
    read('crates/spinon-ffi/Cargo.toml'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3_hostdocument_text_paint.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3_hostdocument_text_paint/layout.rs'),
    read('crates/spinon-ffi/src/runtime_gpu/fixtures/c15_3_hostdocument_text_paint/raster.rs'),
    read('tests/fixtures/css/c15/c15-3-wgpu-glyph-paint.js'),
    read('crates/spinon-ffi/src/runtime_gpu/ffi/fixtures/c15_3.rs'),
    read('crates/spinon-ffi/include/spinon_ffi.h'),
    read('tools/build-android.sh'),
    read('platforms/android/app/build.gradle.kts'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/MainActivity.java'),
    read('platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    read('platforms/android/app/src/main/cpp/spinon_jni.cc'),
    read('platforms/android/app/src/c15_3_wgpu_glyph_paint_fixture_test/java/dev/spinon/bootstrap/GlyphPaintFixtureNativeAdapter.java'),
    read('tools/build-ios-native.sh'),
    read('platforms/ios/SpinonBootstrap.xcodeproj/project.pbxproj'),
    read('platforms/ios/Sources/SpinonRunner.h'),
    read('platforms/ios/Sources/SpinonRunner.mm'),
    read('platforms/ios/Sources/AppDelegate.swift'),
    read('platforms/ios/Sources/C0410RuntimeGpuDemo.swift'),
  ]);

  assert.match(runtimeModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\]\s*pub\(super\) mod c15_3;/);
  assert.match(runtimeModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\]\s*pub\(super\) mod c15_3_hostdocument_text_paint;/);
  assert.match(c15HostDocumentFixture, /mod layout;\s*mod raster;/);
  assert.match(ffiModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\]\s*mod c15_3;/);
  assert.match(cargoManifest,
    /c15-3-wgpu-glyph-paint-fixture-test\s*=\s*\[[\s\S]*?"c04-runtime-gpu-fixture-test"[\s\S]*?"dep:spinon-core"[\s\S]*?"dep:spinon-layout"[\s\S]*?"dep:spinon-text-layout"[\s\S]*?spinon-runtime\/c19-2-box-paint-opacity-fixture-test[\s\S]*?\]/);
  assert.match(c15RuntimeFixture,
    /include_bytes![\s\S]*NotoSansKR-VF\.ttf[\s\S]*c15-3-wgpu-glyph-paint\.js[\s\S]*TextLayoutEngine::new_without_system_fonts[\s\S]*RuntimePaintCommand::DrawText/);
  assert.match(c15HostDocumentLayout,
    /from_host_document_with_text_paragraphs[\s\S]*compute_with_text_layout_engine[\s\S]*text_layouts/);
  assert.match(c15HostDocumentFixture,
    /rasterize_glyphs[\s\S]*validate_raster_snapshot[\s\S]*RuntimeRenderText::new/);
  assert.match(c15HostDocumentLayout,
    /LayoutSourceRevision::HostDocument[\s\S]*source_utf16/);
  assert.match(c15HostDocumentRaster, /overflow_ink_samples[\s\S]*encode_sample_points/);
  assert.match(c15CssFixture,
    /spinon-c15-3-wgpu-glyph-paint\/v2/);
  assert.match(c15CssFixture, /width:360px;height:240px/);
  assert.match(c15CssFixture, /left:160px/);
  assert.match(c15FfiFixture,
    /spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture[\s\S]*spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);
  assert.match(runtimeSource,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\][\s\S]*fixtures::c15_3_hostdocument_text_paint::attach_hostdocument_text_paint[\s\S]*fixtures::c15_3::attach_glyph_paint/);
  assert.match(ffiModules,
    /#\[cfg\(feature = "c15-3-wgpu-glyph-paint-fixture-test"\)\][\s\S]*c15_3/);
  assert.match(ffiHeader,
    /#if defined\(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*?spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);

  assert.match(androidBuild,
    /ffi_features\+=\(c15-3-wgpu-glyph-paint-fixture-test\)/);
  assert.match(androidBuild,
    /c153_wgpu_glyph_paint_cpp_flag=.*SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST/);
  assert.match(androidGradle,
    /src\/c15_3_wgpu_glyph_paint_fixture_test\/java/);
  assert.match(androidActivity,
    /spinon_c153_glyph_paint[\s\S]*SPINON_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST/);
  assert.match(androidDemo,
    /GlyphPaintFixtureNativeAdapter[\s\S]*createHostDocumentTextPaintHost[\s\S]*hostdocument_text_samples_css_px=/);
  assert.match(androidJni,
    /#if defined\(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST\)[\s\S]*?spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture[\s\S]*?spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*?spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);
  assert.match(androidAdapter,
    /createHost\(\);[\s\S]*createHostDocumentTextPaintHost\(\);[\s\S]*evalFixture\(long host\);/);

  assert.match(iosBuild,
    /append_ffi_feature c15-3-wgpu-glyph-paint-fixture-test/);
  assert.match(iosProject,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST=\$\(SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST\)/);
  assert.match(iosHeader,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST[\s\S]*createRuntimeGpuHostWithC15_3GlyphPaintFixture[\s\S]*createRuntimeGpuHostWithC15_3HostDocumentTextPaintFixture/);
  assert.match(iosRunner,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST[\s\S]*spinon_runtime_gpu_host_new_c15_3_glyph_paint_fixture[\s\S]*spinon_runtime_gpu_host_new_c15_3_hostdocument_text_paint_fixture[\s\S]*spinon_runtime_gpu_host_eval_c15_3_glyph_paint_fixture/);
  assert.match(iosAppDelegate,
    /--spinon-c153-glyph-paint[\s\S]*--spinon-c153-hostdocument-text-paint/);
  assert.match(iosDemo,
    /\$0 != "--spinon-c153-glyph-paint"[\s\S]*\$0 != "--spinon-c153-hostdocument-text-paint"/);
  assert.match(iosDemo,
    /SPINON_ENABLE_C15_3_WGPU_GLYPH_PAINT_FIXTURE_TEST_1[\s\S]*createRuntimeGpuHostWithC15_3HostDocumentTextPaintFixture[\s\S]*hostdocument_text_samples_css_px=/);
});

test('C11.4 fixed-repeat + auto-repeat 9 scene은 opt-in runtime fixture 경계에 있다', async () => {
  const fixturePath = 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_fixed_repeat_scenes.json';
  const fixtures = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
  const fixedRepeatFixtures = fixtures.filter(({ id }) => id.startsWith('fixed-repeat-'));
  const fixedAreaFixtures = fixtures.filter(({ id }) => id.startsWith('fixed-auto-area-'));
  const fixtureIDs = fixedRepeatFixtures.map(({ id }) => id);
  assert.equal(fixtures.length, 18);
  assert.equal(fixedRepeatFixtures.length, 9);
  assert.equal(fixedAreaFixtures.length, 9);
  assert.equal(new Set(fixtureIDs).size, 9, 'runtime scene ID collision');
  assert.ok(fixedRepeatFixtures.every(({ id, oracleId }) => (
    id.startsWith('fixed-repeat-') && typeof oracleId === 'string'
  )));

  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--run-kind=fixed-repeat', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 9);
  assert.deepEqual(catalog.selectedSceneIDs, fixtureIDs);

  const [android, ios, runtimeFixtureRoute, runtimeFixtureSource] = await Promise.all([
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs'), 'utf8'),
  ]);
  assert.match(android,
    /BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&&\s*sceneId != null\s*&& sceneId\.startsWith\("fixed-repeat-"\)/);
  assert.match(android,
    /isC114FixedRepeatAutoRepeatPixelSamples\(sceneId, samples\)/,
    'Android C11.4 PixelCopy must validate fixed-repeat sample IDs and coordinates.');
  assert.match(android,
    /sampleId\.startsWith\("fixed-repeat-"\)[\s\S]*?new int\[\] \{34, 197, 94\}/,
    'Android PixelCopy must compare the fixed-repeat green grid background.');
  assert.match(android,
    /BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&&\s*pixelMatrixSceneId != null\s*&&\s*pixelMatrixSceneId\.startsWith\("fixed-repeat-"\)/,
    'Android fixed-repeat pixel samples must remain gated by the opt-in fixture build.');
  const iosFixtureBlock = ios.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1([\s\S]*?)#endif/,
  )?.[1];
  assert.ok(iosFixtureBlock, 'iOS scene IDs must be compiled only for opt-in fixture builds.');
  assert.match(ios,
    /sceneID\.hasPrefix\("fixed-repeat-"\)[\s\S]*?\[34, 197, 94, 255\]/,
    'iOS Metal pixel samples must validate fixed-repeat IDs and expected root color.');
  for (const id of fixtureIDs) {
    assert.ok(iosFixtureBlock.includes(`"${id}"`), `iOS opt-in allowlist is missing ${id}`);
  }
  assert.ok(runtimeFixtureRoute.includes('fixed_repeat_auto_repeat::runtime_source'));
  assert.ok(runtimeFixtureSource.includes('auto_repeat_fixed_repeat_scenes.json'));
});

test('C11.4 fixed + auto-repeat + template areas 9 scene은 두 simulator fixture에 고정된다', async () => {
  const fixturePath = 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/auto_repeat_fixed_repeat_scenes.json';
  const fixtures = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'))
    .filter(({ id }) => id.startsWith('fixed-auto-area-'));
  const fixtureIDs = fixtures.map(({ id }) => id);
  assert.deepEqual(fixtureIDs, [
    'fixed-auto-area-columns-fill-collision',
    'fixed-auto-area-columns-fit-empty',
    'fixed-auto-area-columns-fit-occupied',
    'fixed-auto-area-columns-area-larger',
    'fixed-auto-area-columns-area-shorter',
    'fixed-auto-area-rows-fill-collision',
    'fixed-auto-area-rows-fit-empty',
    'fixed-auto-area-rows-fit-occupied',
    'fixed-auto-area-rows-area-larger',
  ]);
  assert.equal(fixtures.length, 9);

  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--run-kind=fixed-auto-area', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 9);
  assert.deepEqual(catalog.selectedSceneIDs, fixtureIDs);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fixed-auto-repeat-template-areas-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fixed-auto-area-/,
    'fixed-auto-area capture must default to its own evidence folder.');
  const inferred = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(inferred.status, 0, inferred.stderr);
  const inferredCatalog = JSON.parse(inferred.stdout);
  assert.match(inferredCatalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fixed-auto-repeat-template-areas-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fixed-auto-area-/,
    'fixed-auto-area scene IDs must infer the matching run kind without a flag.');

  const [android, ios, runtimeFixtureSource] = await Promise.all([
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fixed_repeat_auto_repeat.rs'), 'utf8'),
  ]);
  const iosFixtureBlock = ios.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1([\s\S]*?)#endif/,
  )?.[1];
  assert.ok(iosFixtureBlock, 'iOS fixed-auto-area IDs must remain opt-in.');
  for (const id of fixtureIDs) {
    assert.ok(android.includes(`"${id}"`), `Android allowlist is missing ${id}`);
    assert.ok(iosFixtureBlock.includes(`"${id}"`), `iOS allowlist is missing ${id}`);
  }
  assert.match(android, /c114FixedAutoAreaPixelSamplesMatch/,
    'Android must validate the root, overflow item, and outside-clear sample request.');
  assert.match(ios, /c114FixedAutoAreaPixelSamples/,
    'iOS must validate the matching Metal surface sample request.');
  assert.match(runtimeFixtureSource,
    /const SCENES_JSON: &str = include_str!\("auto_repeat_fixed_repeat_scenes\.json"\);/);
});

test('C11.4 multiple fixed-repeat groups 12 scene은 opt-in runtime fixture 경계에 있다', async () => {
  const fixturePath = 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/multiple_fixed_repeat_groups_scenes.json';
  const fixtures = JSON.parse(await readFile(resolve(repositoryRoot, fixturePath), 'utf8'));
  const fixtureIDs = fixtures.map(({ id }) => id);
  assert.equal(fixtures.length, 12);
  assert.equal(new Set(fixtureIDs).size, 12, 'runtime scene ID collision');
  assert.ok(fixtures.every(({ id, oracleId }) => (
    id.startsWith('fixed-repeat-') && typeof oracleId === 'string'
  )));

  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--run-kind=multiple-fixed-repeat-groups', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 12);
  assert.deepEqual(catalog.selectedSceneIDs, fixtureIDs);

  const [android, ios, runtimeGpu, runtimeFixtureRoute, runtimeFixtureSource] = await Promise.all([
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu.rs'), 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/multiple_fixed_repeat_groups.rs'), 'utf8'),
  ]);
  assert.match(android,
    /BuildConfig\.SPINON_C04_RUNTIME_GPU_FIXTURE_TEST\s*&&\s*sceneId != null\s*&& sceneId\.startsWith\("fixed-repeat-"\)/);
  const iosFixtureBlock = ios.match(
    /#if SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST_1([\s\S]*?)#endif/,
  )?.[1];
  assert.ok(iosFixtureBlock, 'iOS scene IDs must be compiled only for opt-in fixture builds.');
  for (const id of fixtureIDs) {
    assert.ok(iosFixtureBlock.includes(`"${id}"`), `iOS opt-in scene allowlist is missing ${id}`);
  }
  assert.match(runtimeGpu,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*mod fixtures;/);
  assert.match(runtimeFixtureRoute,
    /#\[path = "c11_4\/multiple_fixed_repeat_groups\.rs"\]\s*mod multiple_fixed_repeat_groups;/);
  assert.ok(runtimeFixtureRoute.includes('multiple_fixed_repeat_groups::runtime_source'));
  assert.ok(runtimeFixtureSource.includes('multiple_fixed_repeat_groups_scenes.json'));
});

test('C11.4 intrinsic auto-repeat WPT 8 scene은 oracle 및 양 플랫폼 검사에 고정된다', async () => {
  const fixtureIDs = [
    'auto-repeat-intrinsic-fill-fixed-max',
    'auto-repeat-intrinsic-fill-fixed-min',
    'auto-repeat-intrinsic-fill-auto-max',
    'auto-repeat-intrinsic-fill-auto-min',
    'auto-repeat-intrinsic-fit-fixed-max',
    'auto-repeat-intrinsic-fit-fixed-min',
    'auto-repeat-intrinsic-fit-auto-max',
    'auto-repeat-intrinsic-fit-auto-min',
  ];
  const oracle = JSON.parse(await readFile(resolve(repositoryRoot,
    'spec/internal/evidence/c11-4-grid-auto-repeat-intrinsic-precomparison-2026-10-27/expected.json'),
  'utf8'));
  assert.deepEqual(oracle.cases.map(({ sceneId }) => sceneId), fixtureIDs);
  assert.deepEqual(oracle.cases.map(({ usedColumnsPx }) => usedColumnsPx), [
    [20, 20, 20, 40], [20, 20, 20, 40], [10, 10, 10, 10, 60], [10, 60],
    [20, 20, 20, 40], [20, 20, 20, 40], [10, 10, 10, 10, 60], [10, 60],
  ]);
  assert.deepEqual(oracle.cases.filter(({ surfaceChecks }) => surfaceChecks)
    .map(({ sceneId }) => sceneId), [fixtureIDs[3], fixtureIDs[7]]);

  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${fixtureIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, fixtureIDs.length);
  assert.deepEqual(catalog.selectedSceneIDs, fixtureIDs);

  const [fixtureRoute, fixtureSource, layoutMatcher, android, ios] = await Promise.all([
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/grid_auto_repeat_intrinsic_wpt.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-layout/src/grid/c114_grid_auto_repeat_intrinsic_wpt.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'),
    'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  assert.match(fixtureRoute,
    /#\[path = "c11_4\/grid_auto_repeat_intrinsic_wpt\.rs"\]\s*mod grid_auto_repeat_intrinsic_wpt;/);
  assert.ok(fixtureRoute.includes('grid_auto_repeat_intrinsic_wpt::runtime_source'));
  assert.equal((fixtureSource.match(/id: "auto-repeat-intrinsic-/g) ?? []).length, 8);
  assert.ok(layoutMatcher.includes('supports_wpt_probe_matrix'));
  for (const source of [android, ios]) {
    assert.ok(source.includes('c114GridAutoRepeatIntrinsicWpt'),
      'both simulator runtimes must validate the WPT track-sizing trace');
    for (const id of fixtureIDs) {
      assert.ok(source.includes(`"${id}"`),
        `both simulator runtimes must include the scene ID ${id}`);
    }
  }
});

test('fit-content min-height WPT slice는 opt-in runtime과 양쪽 simulator 판정에 고정된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    '--scene-id=fit-content-min-height-wpt', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 1);
  assert.deepEqual(catalog.selectedSceneIDs, ['fit-content-min-height-wpt']);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-min-height-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-min-height-/);

  const [capture, route, source, report, runtimeFixture, android, ios] = await Promise.all([
    readFile(capturePath, 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_min_height.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_min_height_report.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_min_height.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  assert.ok(capture.includes('fit-content-min-height-wpt'));
  assert.ok(capture.includes('fit-content-min-height-outside-root'));
  assert.ok(capture.includes('row sizing stages are missing, duplicated, or out of order'));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_min_height\.rs"\]\s*mod fit_content_min_height;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_min_height_report\.rs"\]/);
  assert.ok(source.includes("height:auto;min-height:300px"));
  assert.ok(source.includes("grid-auto-rows:fit-content(40px)"));
  assert.ok(report.includes('fit-content-min-height-wpt'));
  assert.ok(runtimeFixture.includes('validate_rows_pass'));
  assert.ok(runtimeFixture.includes('fn validate_output'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentMinHeightTraceMatches'],
    ['iOS', ios, 'c114FitContentMinHeightMatches'],
  ]) {
    assert.ok(app.includes('fit-content-min-height-wpt'), `${platform} scene allowlist is missing`);
    assert.ok(app.includes(matcher),
      `${platform} strict trace validation is missing`);
    assert.ok(app.includes('fit-content-min-height-outside-root'),
      `${platform} outside-root pixel sample is missing`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});

test('zoomed fit-content WPT slice는 fixture parser와 양쪽 simulator 화면 판정에 고정된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    '--scene-id=fit-content-zoomed-grid-wpt', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 1);
  assert.deepEqual(catalog.selectedSceneIDs, ['fit-content-zoomed-grid-wpt']);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-zoom-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-zoomed-grid-/);

  const [capture, route, source, report, runtimeFixture, android, ios] = await Promise.all([
    readFile(capturePath, 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_zoomed_grid.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_zoomed_grid_report.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_zoomed_grid.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  assert.ok(capture.includes('fit-content-zoomed-grid-wpt'));
  for (const sampleID of [
    'fit-content-zoom-child-in-row', 'fit-content-zoom-child-past-item',
    'fit-content-zoom-child-past-root', 'fit-content-zoom-outside-root',
  ]) assert.ok(capture.includes(sampleID), `${sampleID} surface sample is missing`);
  assert.ok(capture.includes("rootFrame: 'width=120,height=600'"));
  assert.ok(capture.includes("itemFrame: 'width=120,height=80'"));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_zoomed_grid\.rs"\]\s*mod fit_content_zoomed_grid;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_zoomed_grid_report\.rs"\]/);
  assert.ok(source.includes('zoom:2'));
  assert.ok(source.includes('grid-auto-rows:fit-content(40px)'));
  assert.ok(report.includes('upstreamWptCssomGridTemplateRows'));
  assert.ok(runtimeFixture.includes('validate_rows_pass'));
  assert.ok(runtimeFixture.includes('validate_output'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentZoomedGridTraceMatches'],
    ['iOS', ios, 'c114FitContentZoomedGridMatches'],
  ]) {
    assert.ok(app.includes('fit-content-zoomed-grid-wpt'), `${platform} scene allowlist is missing`);
    assert.ok(app.includes(matcher), `${platform} zoomed-grid trace validation is missing`);
    assert.ok(app.includes('fit-content-zoom-child-past-root'),
      `${platform} child overflow past the zoomed root is not sampled`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});

test('definite-height percentage fit-content WPT slices는 opt-in runtime과 양쪽 simulator 판정에 고정된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    '--scene-id=fit-content-percentage-row-wpt', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 1);
  assert.deepEqual(catalog.selectedSceneIDs, ['fit-content-percentage-row-wpt']);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-percentage-argument-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-percentage-argument-/);

  const [capture, route, source, report, runtimeFixture, runtimeTraceValidator,
    runtimeStyleValidator, runtimeFixtureTests, android, ios] = await Promise.all([
    readFile(capturePath, 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_percentage_argument.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_percentage_argument_report.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_percentage_argument.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_percentage_argument/trace_validation.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_percentage_argument/style_validation.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_percentage_argument/tests.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  assert.ok(capture.includes('fit-content-percentage-row-wpt'));
  assert.ok(capture.includes('fit-content-calc-percentage-row-wpt'));
  assert.ok(capture.includes('fit-content-percentage-child-outside-root'));
  assert.ok(capture.includes('fit-content-calc-percentage-child-outside-root'));
  assert.ok(capture.includes('TrackList([FitContent(Percentage(0.5))])'));
  assert.ok(capture.includes('FitContent(Calc('));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_percentage_argument\.rs"\]\s*mod fit_content_percentage_argument;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_percentage_argument_report\.rs"\]/);
  assert.ok(source.includes('width:60px;height:300px'));
  assert.ok(source.includes('grid-auto-rows:fit-content(50%)'));
  assert.ok(source.includes('grid-auto-rows:fit-content(calc(10% + 5px))'));
  assert.ok(source.includes('min-height:0;'));
  assert.doesNotMatch(source, /item\.setAttribute\('style', '[^']*overflow:visible/,
    'the default visible overflow must remain an un-authored CSS default');
  assert.ok(report.includes('observation.scene_id'));
  assert.ok(runtimeStyleValidator.includes('ComputedCssMath::Sum'));
  assert.ok(runtimeStyleValidator.includes('GridTrackLengthPercentage::Percentage(0.5)'));
  assert.ok(runtimeFixture.includes('validate_rows_pass'));
  assert.match(runtimeFixture,
    /#\[path = "c11_4_fit_content_percentage_argument\/style_validation\.rs"\]\s*mod style_validation;/);
  assert.ok(runtimeStyleValidator.includes('ComputedCssMath::Sum'));
  assert.ok(runtimeStyleValidator.includes('LayoutCssMathProperty::GridTemplateRows'));
  assert.ok(runtimeStyleValidator.includes('validate_layout_math'));
  assert.ok(runtimeTraceValidator.includes('LayoutGridTrackSizingStageKind::IntrinsicItemBatch'));
  assert.ok(runtimeTraceValidator.includes('row minimum/min-content/max-content contributions differ'));
  assert.match(runtimeFixture,
    /#\[cfg\(test\)\]\s*#\[path = "c11_4_fit_content_percentage_argument\/tests\.rs"\]\s*mod tests;/);
  assert.ok(runtimeFixtureTests.includes('accepts_only_the_pinned_definite_height_percentage_grid_input'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentPercentageArgumentTraceMatches'],
    ['iOS', ios, 'c114FitContentPercentageArgumentMatches'],
  ]) {
    assert.ok(app.includes('fit-content-percentage-row-wpt'),
      `${platform} scene allowlist is missing`);
    assert.ok(app.includes('fit-content-calc-percentage-row-wpt'),
      `${platform} calc scene allowlist is missing`);
    assert.ok(app.includes(matcher), `${platform} strict trace validation is missing`);
    assert.ok(app.includes('IntrinsicItemBatch { span: 1, crosses_flexible_track: false }'),
      `${platform} strict matcher does not lock the one-row intrinsic stage`);
    assert.ok(app.includes('child-outside-root'),
      `${platform} outside-root overflow sample is missing`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});

test('definite-height calc fit-content WPT slice is isolated in the runtime catalog', () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    '--scene-id=fit-content-calc-percentage-row-wpt', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 1);
  assert.deepEqual(catalog.selectedSceneIDs, ['fit-content-calc-percentage-row-wpt']);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-percentage-argument-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-percentage-argument-/);
});

test('fit-content automatic-minimum floor와 zero control이 opt-in runtime에 고정된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const sceneIDs = [
    'fit-content-auto-min-floor-wpt',
    'fit-content-auto-min-floor-zero-control',
  ];
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${sceneIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 2);
  assert.deepEqual(catalog.selectedSceneIDs, sceneIDs);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-auto-min-floor-runtime-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-auto-min-floor-/);

  const [capture, route, source, report, runtimeFixture, android, ios] = await Promise.all([
    readFile(capturePath, 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_min_floor.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_auto_min_floor_report.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_auto_min_floor.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  for (const sceneID of sceneIDs) assert.ok(capture.includes(sceneID));
  assert.ok(capture.includes('fitContentAutoMinFloor'));
  assert.ok(capture.includes('itemMinHeights'));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_auto_min_floor\.rs"\]\s*mod fit_content_auto_min_floor;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_auto_min_floor_report\.rs"\]/);
  assert.ok(source.includes('grid-auto-rows:fit-content(40px)'));
  assert.ok(source.includes('min-height:0'));
  assert.ok(report.includes('expectedRowBaseSizesCssPx'));
  assert.ok(runtimeFixture.includes('track_entries_match'));
  assert.ok(runtimeFixture.includes('fn validate_output'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentAutoMinFloorMatches'],
    ['iOS', ios, 'c114FitContentAutoMinFloorMatches'],
  ]) {
    for (const sceneID of sceneIDs) {
      assert.ok(app.includes(sceneID), `${platform} scene allowlist is missing ${sceneID}`);
    }
    assert.ok(app.includes(matcher), `${platform} strict trace validation is missing`);
    assert.ok(app.includes('fit-content-auto-min-floor-control-overflow'),
      `${platform} overflow sample is missing`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});

test('fit-content span-2 WPT slice가 opt-in runtime과 simulator capture에 연결된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    '--scene-id=fit-content-spanning-wpt', '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 1);
  assert.deepEqual(catalog.selectedSceneIDs, ['fit-content-spanning-wpt']);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-spanning-simulators-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-spanning-/);

  const [capture, route, source, report, runtimeFixture, runtimeValidation, android, ios] = await Promise.all([
    readFile(capturePath, 'utf8'),
    readFile(resolve(repositoryRoot, 'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_spanning.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_spanning_report.rs'), 'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_spanning.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_spanning/validation.rs'),
    'utf8'),
    readFile(resolve(repositoryRoot,
      'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java'), 'utf8'),
    readFile(resolve(repositoryRoot, 'platforms/ios/Sources/C0410RuntimeGpuDemo.swift'), 'utf8'),
  ]);
  assert.ok(capture.includes('fit-content-spanning-wpt'));
  assert.ok(capture.includes('fitContentSpanning'));
  assert.ok(capture.includes('spanning-child-over-item'));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_spanning\.rs"\]\s*mod fit_content_spanning;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_spanning_report\.rs"\]/);
  assert.ok(source.includes('grid-auto-rows:fit-content(40px)'));
  assert.ok(source.includes('grid-row:span 2'));
  assert.ok(source.includes('min-height:0'));
  assert.ok(!source.includes('overflow:visible'),
    'the default visible overflow must not become an authored CSS declaration');
  assert.ok(report.includes('itemRowSpanTwo'));
  assert.ok(report.includes('itemOverflowY'));
  assert.ok(report.includes('"infinitelyGrowable"'));
  assert.ok(runtimeValidation.includes('validate_rows_pass'));
  assert.ok(runtimeValidation.includes('track.growth_limit == expected_growth_limit'));
  assert.ok(runtimeFixture.includes('mod validation'));
  assert.ok(runtimeFixture.includes('fn validate_output'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentSpanningTraceMatches'],
    ['iOS', ios, 'c114FitContentSpanningMatches'],
  ]) {
    assert.ok(app.includes('fit-content-spanning-wpt'), `${platform} scene allowlist is missing`);
    assert.ok(app.includes(matcher), `${platform} strict trace validation is missing`);
    assert.ok(app.includes('spanning-child-over-item'),
      `${platform} overflow pixel sample is missing`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});

test('fit-content WPT growth-limit 경계 세 건이 opt-in runtime과 두 simulator에 연결된다', async () => {
  const capturePath = resolve(repositoryRoot,
    'tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs');
  const sceneIDs = [
    'fit-content-limit-40-wpt', 'fit-content-limit-30-wpt', 'fit-content-limit-200-wpt',
  ];
  const selected = spawnSync(process.execPath, [capturePath, 'android-avd',
    `--scene-id=${sceneIDs.join(',')}`, '--list-scenes'],
  { cwd: repositoryRoot, encoding: 'utf8' });
  assert.equal(selected.status, 0, selected.stderr);
  const catalog = JSON.parse(selected.stdout);
  assert.equal(catalog.sceneCount, 622);
  assert.equal(catalog.selectedSceneCount, 3);
  assert.deepEqual(catalog.selectedSceneIDs, sceneIDs);
  assert.match(catalog.evidenceDirectory,
    /^spec\/internal\/evidence\/c11-4-fit-content-limit-boundaries-simulators-\d{4}-\d{2}-\d{2}\/android-avd-c114-fit-content-limit-boundaries-/);

  const files = [
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs',
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_limit_boundaries.rs',
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/fit_content_limit_boundaries_report.rs',
    'crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4/trace_report.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_limit_boundaries.rs',
    'crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_fit_content_limit_boundaries/validation.rs',
    'platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java',
    'platforms/ios/Sources/C0410RuntimeGpuDemo.swift',
  ];
  const [capture, ...contents] = await Promise.all([
    readFile(capturePath, 'utf8'),
    ...files.map((path) => readFile(resolve(repositoryRoot, path), 'utf8')),
  ]);
  const [route, source, report, traceReport, runtime, runtimeModules,
    runtimeFixture, runtimeValidation, android, ios] = contents;
  assert.ok(sceneIDs.every((sceneID) => capture.includes(sceneID)));
  assert.ok(capture.includes('fitContentLimitBoundaries'));
  assert.ok(capture.includes('child-past-item'));
  assert.match(route,
    /#\[path = "c11_4\/fit_content_limit_boundaries\.rs"\]\s*mod fit_content_limit_boundaries;/);
  assert.match(route,
    /#\[cfg\(feature = "c04-runtime-gpu-fixture-test"\)\]\s*#\[path = "c11_4\/fit_content_limit_boundaries_report\.rs"\]/);
  for (const sourceText of [source, report, runtime, runtimeModules, runtimeFixture,
    runtimeValidation, traceReport]) {
    assert.ok(sourceText.includes('fit_content_limit_boundaries'),
      'runtime path is missing the growth-limit boundary fixture name');
  }
  assert.ok(source.includes('min-height:0'));
  assert.ok(source.includes('height:100px'));
  assert.ok(!source.includes('overflow:visible'),
    'the default visible overflow must not become an authored CSS declaration');
  assert.ok(runtimeValidation.includes('MaximizeTracks'));
  assert.ok(runtimeValidation.includes('growth_limit'));
  for (const [platform, app, matcher] of [
    ['Android', android, 'c114FitContentLimitBoundariesTraceMatches'],
    ['iOS', ios, 'c114FitContentLimitBoundariesMatches'],
  ]) {
    assert.ok(sceneIDs.every((sceneID) => app.includes(sceneID)),
      `${platform} scene allowlist is incomplete`);
    assert.ok(app.includes(matcher), `${platform} strict trace validation is missing`);
    assert.ok(app.includes('child-past-item'), `${platform} overflow pixel sample is missing`);
    assert.ok(app.includes('C114_GRID_PRESENTATION_MATCH'),
      `${platform} environment-to-draw identity check is missing`);
  }
});
