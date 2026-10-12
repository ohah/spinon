import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { arch, platform, release } from 'node:os';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c11/grid-auto-placement-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c11/grid-auto-placement.html';
const captureRelativePath = 'tools/css-reference/capture-c11-grid-auto-placement-precomparison.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const defaultOutputRelativePath = 'tests/fixtures/css/references/c11-grid-auto-placement-precomparison-v3.json';
const outputArgument = process.argv.slice(2).find((argument) => argument.startsWith('--output='));
const outputRelativePath = outputArgument?.slice('--output='.length) ?? defaultOutputRelativePath;
assert.match(outputRelativePath,
  /^tests\/fixtures\/css\/references\/c11-grid-auto-placement-precomparison-v\d+\.json$/,
  '새 Chromium 기준은 버전이 있는 fixture 경로에만 저장합니다.');

const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const evidenceBase = 'spec/internal/evidence/c11-2-2-sparse-row-placement-precomparison-2026-10-12';
const runId = `c1122-chrome-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const [inventoryBytes, htmlBytes] = await Promise.all([readFile(inventoryPath), readFile(htmlPath)]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const nodeIds = inventory.cases.flatMap(({ nodeIds: ids }) => ids);
assert.equal(inventory.schema, 'spinon-css-c11-2-2-grid-auto-placement-inventory/v2');
assert.equal(inventory.fixtureId, 'C11-2-2-grid-auto-placement-precomparison-v2');
assert.deepEqual(inventory.viewport.deviceScaleFactors, [1, 2]);
assert.equal(inventory.cases.length, 15);
assert.equal(inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').length, 10);
assert.equal(inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only').length, 5);
assert.equal(new Set(nodeIds).size, nodeIds.length, '각 장면의 node ID는 전체 fixture에서 고유해야 합니다.');
assert.equal(inventory.wpt.revision, 'c999c58338ee1d223df5ad62e48be1202ced0d37');
assert.deepEqual(inventory.wpt.candidates.map(({ path, sha256 }) => [path, sha256]), [
  ['css/css-grid/placement/grid-layout-grid-span.html', '1be809e54f1b7398112923acd60ef423735934b528ff42c1a8f3b326adbebef7'],
  ['css/css-grid/placement/grid-negative-line-placement-001.html', 'e5d17120d87170282d2522ec483d751c6f7db9ac23ec09d2a540cce305d81d31'],
]);
await access(outputPath).then(
  () => { throw new Error(`기존 Chromium 기준을 덮어쓰지 않습니다: ${outputRelativePath}`); },
  (error) => { if (error.code !== 'ENOENT') throw error; },
);

const chromiumPath = await findChromiumExecutable();
const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    assert.equal(browserVersion.product, 'Chrome/154.0.8037.98');
    assert.equal(browserVersion.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setLocaleOverride', { locale: inventory.environment.locale });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: inventory.environment.timeZone });
    await page.send('Emulation.setUserAgentOverride', {
      userAgent: browserVersion.userAgent,
      acceptLanguage: inventory.environment.locale,
    });
    await page.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
    await page.send('Emulation.setEmulatedMedia', {
      features: [
        { name: 'prefers-color-scheme', value: inventory.environment.colorScheme },
        { name: 'forced-colors', value: inventory.environment.forcedColors },
        { name: 'pointer', value: inventory.environment.pointer },
        { name: 'hover', value: inventory.environment.hover },
      ],
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
    await loaded;

    const observations = [];
    const screenshots = [];
    for (const deviceScaleFactor of inventory.viewport.deviceScaleFactors) {
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: inventory.viewport.width,
        height: inventory.viewport.height,
        deviceScaleFactor,
        mobile: false,
        screenWidth: inventory.viewport.width,
        screenHeight: inventory.viewport.height,
      });
      const cases = [];
      for (const fixtureCase of inventory.cases) {
        const expression = '(() => {'
          + `const caseId = ${JSON.stringify(fixtureCase.id)};`
          + `const expectedIds = ${JSON.stringify(fixtureCase.nodeIds)};`
          + 'const scenes = Array.from(document.querySelectorAll("[data-scene]"));'
          + 'for (const scene of scenes) scene.classList.toggle("active", scene.dataset.scene === caseId);'
          + 'if (scenes.filter(scene => scene.classList.contains("active")).length !== 1) '
          + 'throw new Error("활성 scene 수가 1이 아닙니다: " + caseId);'
          + 'const scene = scenes.find(candidate => candidate.dataset.scene === caseId);'
          + 'if (!scene) throw new Error("scene 누락: " + caseId);'
          + 'const root = scene.querySelector(".scene-root");'
          + 'const rootRect = root.getBoundingClientRect();'
          + 'const elements = Array.from(scene.querySelectorAll("[data-node]"));'
          + 'const actualIds = elements.map(element => element.id);'
          + 'if (JSON.stringify(actualIds) !== JSON.stringify(expectedIds)) '
          + 'throw new Error("DOM preorder 불일치: " + caseId + "/" + JSON.stringify(actualIds));'
          + 'const nodes = elements.map(element => {'
          + 'const style = getComputedStyle(element);'
          + 'const rect = element.getBoundingClientRect();'
          + 'const parentId = element.parentElement?.closest("[data-node]")?.id ?? null;'
          + 'return {id:element.id,parentId,authoredStyle:element.getAttribute("style") ?? "",'
          + 'properties:Object.fromEntries('
          + `${JSON.stringify(inventory.comparison.computedProperties)}`
          + '.map(name => [name, style.getPropertyValue(name).trim()])), '
          + 'rect:{x:rect.left-rootRect.left,y:rect.top-rootRect.top,width:rect.width,height:rect.height},'
          + 'documentRect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height}};});'
          + `if (nodes.find(node => node.id === ${JSON.stringify(fixtureCase.rootId)})?.parentId !== null) `
          + 'throw new Error("case root parent가 null이 아닙니다: " + caseId);'
          + 'const grids = Array.from(scene.querySelectorAll("[data-grid]"));'
          + 'const gridContainers = grids.map(element => {'
          + 'const style = getComputedStyle(element);'
          + 'const trackCount = value => value === "none" ? 0 : value.trim().split(/\\s+/).filter(Boolean).length;'
          + 'const authoredTrackCount = property => {'
          + 'const value = element.style.getPropertyValue(property).trim();'
          + 'if (!/^\\d+(?:\\.\\d+)?px(?:\\s+\\d+(?:\\.\\d+)?px)*$/.test(value)) '
          + 'throw new Error("fixture authored track은 px list여야 합니다: " + element.id + "/" + property);'
          + 'return trackCount(value);};'
          + 'const computedColumns = trackCount(style.gridTemplateColumns);'
          + 'const computedRows = trackCount(style.gridTemplateRows);'
          + 'const explicitColumns = authoredTrackCount("grid-template-columns");'
          + 'const explicitRows = authoredTrackCount("grid-template-rows");'
          + 'if (computedColumns < explicitColumns || computedRows < explicitRows) '
          + 'throw new Error("computed track 수가 explicit보다 작습니다: " + element.id);'
          + 'return {id:element.id,trackCounts:{explicitColumns,computedColumns,implicitColumns:computedColumns-explicitColumns,'
          + 'explicitRows,computedRows,implicitRows:computedRows-explicitRows},properties:Object.fromEntries('
          + 'Array.from(style).filter(name => name.startsWith("grid-")).sort()'
          + '.map(name => [name, style.getPropertyValue(name).trim()]))};});'
          + 'const env = {direction:getComputedStyle(root).direction,writingMode:getComputedStyle(root).writingMode};'
          + 'return JSON.stringify({id:caseId,scope:' + JSON.stringify(fixtureCase.scope)
          + ',rootId:' + JSON.stringify(fixtureCase.rootId)
          + ',viewport:{width:innerWidth,height:innerHeight,deviceScaleFactor:devicePixelRatio},'
          + 'environment:env,rootRect:{x:rootRect.x,y:rootRect.y,width:rootRect.width,height:rootRect.height},'
          + 'gridContainers,nodes});'
          + '})()';
        const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
        const text = evaluation.result?.value;
        assert.equal(evaluation.exceptionDetails, undefined,
          `${fixtureCase.id}: ${evaluation.exceptionDetails?.text ?? 'Chrome 평가 실패'}`);
        assert.equal(typeof text, 'string', `${fixtureCase.id}: observation JSON 누락`);
        const observation = JSON.parse(text);
        assert.equal(observation.id, fixtureCase.id);
        assert.equal(observation.scope, fixtureCase.scope);
        assert.deepEqual(observation.nodes.map(({ id }) => id), fixtureCase.nodeIds);
        assert.equal(observation.nodes[0].id, fixtureCase.rootId);
        assert.deepEqual(observation.environment, {
          direction: inventory.environment.direction,
          writingMode: inventory.environment.writingMode,
        });
        assert.equal(observation.viewport.width, inventory.viewport.width);
        assert.equal(observation.viewport.height, inventory.viewport.height);
        assert.equal(observation.viewport.deviceScaleFactor, deviceScaleFactor);
        assert.equal(observation.gridContainers.length, fixtureCase.nodeIds.filter((id) => (
          observation.nodes.find((node) => node.id === id)?.properties.display === 'grid'
        )).length);
        for (const node of observation.nodes) {
          for (const field of inventory.comparison.rectFields) {
            assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}가 유한하지 않습니다.`);
          }
          assert.deepEqual(Object.keys(node.properties), inventory.comparison.computedProperties);
        }
        cases.push(observation);
        if (deviceScaleFactor === 1) {
          const screenshot = await page.send('Page.captureScreenshot', {
            format: 'png', fromSurface: true, captureBeyondViewport: false,
          });
          screenshots.push({ caseId: fixtureCase.id, data: screenshot.data });
        }
      }
      observations.push({ deviceScaleFactor, cases });
    }

    const [base, scaled] = observations;
    for (const fixtureCase of inventory.cases) {
      const baseCase = base.cases.find(({ id }) => id === fixtureCase.id);
      const scaledCase = scaled.cases.find(({ id }) => id === fixtureCase.id);
      assert.deepEqual(baseCase.nodes, scaledCase.nodes,
        `DPR 변경으로 CSS computed 값 또는 frame이 바뀌었습니다: ${fixtureCase.id}`);
      assert.deepEqual(baseCase.gridContainers, scaledCase.gridContainers,
        `DPR 변경으로 Grid track 관측값이 바뀌었습니다: ${fixtureCase.id}`);
    }
    return { observations, screenshots, browserVersion };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const runDirectoryRelative = join(evidenceBase, runId).replaceAll('\\', '/');
const runDirectory = join(repositoryRoot, runDirectoryRelative);
await mkdir(join(repositoryRoot, evidenceBase), { recursive: true });
await mkdir(runDirectory, { recursive: false });
const screenshots = [];
for (const screenshot of captureResult.screenshots) {
  const bytes = Buffer.from(screenshot.data, 'base64');
  const relativePath = `${runDirectoryRelative}/${screenshot.caseId}-dpr1.png`;
  await writeFile(join(repositoryRoot, relativePath), bytes, { flag: 'wx' });
  screenshots.push({ caseId: screenshot.caseId, path: relativePath, sha256: hashBytes(bytes) });
}

const captureSha256 = await sha256File(fileURLToPath(import.meta.url));
const reference = {
  schema: 'spinon-css-c11-2-2-grid-auto-placement-precomparison/v3',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + hashBytes(inventoryBytes).slice(0, 12)
    + '-' + hashBytes(htmlBytes).slice(0, 12)
    + '-' + captureSha256.slice(0, 12),
  runId,
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: hashBytes(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: hashBytes(htmlBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: captureSha256,
    dependencies: [{ path: helperRelativePath, sha256: await sha256File(join(repositoryRoot, helperRelativePath)) }],
  },
  oracle: {
    name: 'Google Chrome',
    product: captureResult.browserVersion.product,
    revision: captureResult.browserVersion.revision,
    userAgent: captureResult.browserVersion.userAgent,
    executablePath: chromiumPath,
    executableSha256: await sha256File(chromiumPath),
    host: { platform: platform(), architecture: arch(), release: release(), node: process.version },
    flags: browserFlags,
  },
  environment: {
    viewportCssPx: { width: inventory.viewport.width, height: inventory.viewport.height },
    deviceScaleFactors: inventory.viewport.deviceScaleFactors,
    locale: inventory.environment.locale,
    timeZone: inventory.environment.timeZone,
    colorScheme: inventory.environment.colorScheme,
    forcedColors: inventory.environment.forcedColors,
    pointer: inventory.environment.pointer,
    hover: inventory.environment.hover,
    direction: inventory.environment.direction,
    writingMode: inventory.environment.writingMode,
  },
  wpt: inventory.wpt,
  comparison: inventory.comparison,
  screenshots,
  observations: captureResult.observations,
};
await writeFile(outputPath, JSON.stringify(reference, null, 2) + '\n', { flag: 'wx' });
await writeFile(join(runDirectory, 'capture-summary.json'), JSON.stringify({
  runId,
  referencePath: outputRelativePath,
  referenceId: reference.referenceId,
  screenshotCount: screenshots.length,
  caseCount: inventory.cases.length,
  nodeCount: nodeIds.length,
  implementationCandidateCases: inventory.cases.filter(({ scope }) => scope === 'implementation-candidate').map(({ id }) => id),
  oracleOnlyCases: inventory.cases.filter(({ scope }) => scope === 'chromium-oracle-only').map(({ id }) => id),
  dpr: inventory.viewport.deviceScaleFactors,
  wptSourcesHashedNotRun: inventory.wpt.candidates.map(({ path, sha256, status }) => ({ path, sha256, status })),
}, null, 2) + '\n', { flag: 'wx' });
console.log(`저장: ${outputRelativePath}`);
console.log(`Chrome: ${captureResult.browserVersion.product} ${captureResult.browserVersion.revision}`);
console.log(`${inventory.cases.length} scene, ${nodeIds.length} node, DPR 1·2, screenshot ${screenshots.length}개`);
