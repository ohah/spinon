import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { arch, platform, release } from 'node:os';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c11/grid-template-areas-c1133-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c11/grid-template-areas-c1133.html';
const captureRelativePath = 'tools/css-reference/capture-c11-3-3-grid-template-areas.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const defaultOutputRelativePath = 'tests/fixtures/css/references/c11-3-3-grid-template-areas-precomparison-v1.json';
const outputArgument = process.argv.slice(2).find((argument) => argument.startsWith('--output='));
const outputRelativePath = outputArgument?.slice('--output='.length) ?? defaultOutputRelativePath;
assert.match(outputRelativePath,
  /^tests\/fixtures\/css\/references\/c11-3-3-grid-template-areas-precomparison-v\d+\.json$/,
  'reference는 버전이 있는 CSS fixture 경로에만 저장합니다.');

const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const evidenceDirectory = join(repositoryRoot, 'spec/internal/evidence/c11-3-3-grid-template-areas-precomparison-2026-10-14');
const runId = `c1133-chrome-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const [inventoryBytes, htmlBytes] = await Promise.all([readFile(inventoryPath), readFile(htmlPath)]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const expectedCaseIds = [
  'basic-rectangles', 'null-cell-dot-runs', 'all-null-cells', 'single-cell-area',
  'bare-area-edge-aliases', 'grid-area-one-value', 'grid-area-two-values',
  'grid-area-three-values', 'grid-area-four-values', 'row-column-longhands-area',
  'grid-template-shorthand', 'grid-shorthand', 'bare-area-vs-explicit-one-name',
  'explicit-line-before-area', 'explicit-line-after-area', 'area-names-ending-start-or-end',
  'implicit-area-from-explicit-lines', 'escaped-and-unicode-identifiers',
  'invalid-l-shape-fallback', 'invalid-disconnected-area-fallback',
  'invalid-row-width-fallback', 'invalid-trash-token-fallback', 'empty-string-fallback',
  'area-track-count-mismatch', 'unknown-area-implicit-tracks', 'nested-hidden-source-order',
];
assert.equal(inventory.schema, 'spinon-css-c11-3-3-grid-template-areas-inventory/v1');
assert.equal(inventory.fixtureId, 'C11.3.3-grid-template-areas-precomparison-v1');
assert.deepEqual(inventory.cases.map(({ id }) => id), expectedCaseIds);
assert.equal(inventory.viewport.width, 320);
assert.equal(inventory.viewport.height, 240);
assert.deepEqual(inventory.viewport.deviceScaleFactors, [1, 2]);
assert.equal(new Set(inventory.cases.flatMap(({ nodeIds }) => nodeIds)).size,
  inventory.cases.flatMap(({ nodeIds }) => nodeIds).length);
await access(outputPath).then(
  () => { throw new Error(`기존 Chromium 기준 파일을 덮어쓰지 않습니다: ${outputRelativePath}`); },
  (error) => { if (error.code !== 'ENOENT') throw error; },
);

function parseResolvedGridTemplate(serialized) {
  const value = serialized.trim();
  if (value === 'none') return { trackSizes: [], lineNameGroups: [[]] };
  const trackSizes = [];
  const lineNameGroups = [[]];
  let index = 0;
  while (index < value.length) {
    while (/\s/.test(value[index] ?? '')) index += 1;
    if (index >= value.length) break;
    if (value[index] === '[') {
      const end = value.indexOf(']', index + 1);
      if (end < 0) throw new Error(`CSSOM grid line name이 닫히지 않았습니다: ${serialized}`);
      lineNameGroups.at(-1).push(...value.slice(index + 1, end).trim().split(/\s+/).filter(Boolean));
      index = end + 1;
      continue;
    }
    const start = index;
    let parentheses = 0;
    while (index < value.length) {
      const character = value[index];
      if (character === '(') parentheses += 1;
      else if (character === ')') parentheses -= 1;
      if (parentheses === 0 && (character === '[' || /\s/.test(character))) break;
      index += 1;
    }
    const trackSize = value.slice(start, index).trim();
    if (!trackSize || parentheses !== 0) throw new Error(`CSSOM grid track token을 해석할 수 없습니다: ${serialized}`);
    trackSizes.push(trackSize);
    lineNameGroups.push([]);
  }
  return { trackSizes, lineNameGroups };
}

function parseCssEscape(value) {
  return value.replace(/\\([0-9a-fA-F]{1,6})\s?|\\(.)/g, (_match, hex, escaped) =>
    hex ? String.fromCodePoint(Number.parseInt(hex, 16)) : escaped);
}

function parseGridTemplateAreas(serialized) {
  const value = serialized.trim();
  if (value === 'none') return null;
  const rows = [...value.matchAll(/"((?:\\.|[^"\\])*)"/gs)]
    .map((match) => parseCssEscape(match[1]).trim().split(/\s+/).filter(Boolean)
      .map((token) => /^\.+$/.test(token) ? '.' : token));
  if (rows.length === 0) throw new Error(`CSSOM grid-template-areas를 읽을 수 없습니다: ${serialized}`);
  return rows;
}

function deriveNamedAreaRectangles(rows) {
  if (rows === null) return {};
  const coordinates = new Map();
  rows.forEach((row, y) => row.forEach((name, x) => {
    if (name === '.') return;
    const points = coordinates.get(name) ?? [];
    points.push([x, y]);
    coordinates.set(name, points);
  }));
  return Object.fromEntries([...coordinates].map(([name, points]) => {
    const xs = points.map(([x]) => x);
    const ys = points.map(([, y]) => y);
    const xStart = Math.min(...xs);
    const xEnd = Math.max(...xs) + 1;
    const yStart = Math.min(...ys);
    const yEnd = Math.max(...ys) + 1;
    assert.equal(points.length, (xEnd - xStart) * (yEnd - yStart), `Chrome area rectangle has a hole: ${name}`);
    return [name, { rowStart: yStart, rowEnd: yEnd, columnStart: xStart, columnEnd: xEnd }];
  }));
}

const chromiumPath = await findChromiumExecutable();
const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    assert.equal(browserVersion.product, 'Chrome/154.0.8037.92');
    assert.equal(browserVersion.revision, '@334b65d254ccc35df4fca82706d1753227b01039');
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
      const caseObservations = [];
      for (const fixtureCase of inventory.cases) {
        const expression = '(() => {'
          + `const caseId=${JSON.stringify(fixtureCase.id)};`
          + `const expectedIds=${JSON.stringify(fixtureCase.nodeIds)};`
          + 'for(const scene of document.querySelectorAll("[data-scene]"))'
          + 'scene.classList.toggle("active",scene.dataset.scene===caseId);'
          + 'const scene=document.querySelector(`[data-scene="${caseId}"]`);'
          + 'if(!scene)throw new Error("scene 누락: "+caseId);'
          + 'const root=scene.querySelector("[data-node]");'
          + 'const rootRect=root.getBoundingClientRect();'
          + 'const elements=Array.from(scene.querySelectorAll("[data-node]"));'
          + 'const actualIds=elements.map(element=>element.id);'
          + 'if(JSON.stringify(actualIds)!==JSON.stringify(expectedIds))'
          + 'throw new Error("DOM preorder 불일치: "+caseId+"/"+JSON.stringify(actualIds));'
          + 'const nodes=elements.map(element=>{'
          + 'const style=getComputedStyle(element);const rect=element.getBoundingClientRect();'
          + 'const parentId=element.parentElement?.closest("[data-node]")?.id??null;'
          + `const properties=Object.fromEntries(${JSON.stringify(inventory.comparison.computedProperties)}`
          + '.map(name=>[name,style.getPropertyValue(name).trim()]));'
          + 'return{id:element.id,parentId,properties,rect:{'
          + 'x:rect.left-rootRect.left,y:rect.top-rootRect.top,width:rect.width,height:rect.height},'
          + 'documentRect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height}};});'
          + 'const rootStyle=getComputedStyle(root);'
          + 'return JSON.stringify({id:caseId,rootId:root.id,'
          + 'viewport:{width:innerWidth,height:innerHeight,deviceScaleFactor:devicePixelRatio},'
          + 'environment:{direction:rootStyle.direction,writingMode:rootStyle.writingMode},'
          + 'rootRect:{x:rootRect.x,y:rootRect.y,width:rootRect.width,height:rootRect.height},nodes});'
          + '})()';
        const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
        assert.equal(evaluation.exceptionDetails, undefined,
          `${fixtureCase.id}: ${evaluation.exceptionDetails?.text ?? 'Chrome 캡처 실패'}`);
        assert.equal(typeof evaluation.result?.value, 'string', `${fixtureCase.id}: observation JSON이 없습니다.`);
        const observation = JSON.parse(evaluation.result.value);
        assert.deepEqual(observation.nodes.map(({ id }) => id), fixtureCase.nodeIds);
        assert.equal(observation.nodes[0].id, fixtureCase.rootId);
        assert.equal(observation.viewport.width, inventory.viewport.width);
        assert.equal(observation.viewport.height, inventory.viewport.height);
        assert.equal(observation.viewport.deviceScaleFactor, deviceScaleFactor);
        assert.deepEqual(observation.environment, {
          direction: inventory.environment.direction,
          writingMode: inventory.environment.writingMode,
        });

        const rootNode = observation.nodes[0];
        const columns = parseResolvedGridTemplate(rootNode.properties['grid-template-columns']);
        const rows = parseResolvedGridTemplate(rootNode.properties['grid-template-rows']);
        const areaRows = parseGridTemplateAreas(rootNode.properties['grid-template-areas']);
        observation.resolvedGrid = {
          columns,
          rows,
          areas: {
            rows: areaRows,
            rectangles: deriveNamedAreaRectangles(areaRows),
            derivedFromCssomResolvedValues: true,
          },
        };
        assert.equal(columns.trackSizes.length, fixtureCase.expectedColumns,
          `${fixtureCase.id}: Chrome resolved columns (${rootNode.properties['grid-template-columns']})`);
        assert.equal(columns.lineNameGroups.length, columns.trackSizes.length + 1,
          `${fixtureCase.id}: Chrome resolved column line groups`);
        assert.equal(rows.trackSizes.length, fixtureCase.expectedRows,
          `${fixtureCase.id}: Chrome resolved rows (${rootNode.properties['grid-template-rows']})`);
        assert.equal(rows.lineNameGroups.length, rows.trackSizes.length + 1,
          `${fixtureCase.id}: Chrome resolved row line groups`);
        assert.deepEqual(areaRows, fixtureCase.expectedComputedAreaRows,
          `${fixtureCase.id}: CSS cascade or area tokenization differs from expected matrix`);
        for (const node of observation.nodes) {
          for (const field of inventory.comparison.rectFields) {
            assert.ok(Number.isFinite(node.rect[field]), `${fixtureCase.id}/${node.id}/${field}가 유한하지 않습니다.`);
          }
        }
        caseObservations.push(observation);
        if (deviceScaleFactor === 1) {
          const screenshot = await page.send('Page.captureScreenshot', {
            format: 'png', fromSurface: true, captureBeyondViewport: false,
          });
          screenshots.push({ caseId: fixtureCase.id, data: screenshot.data });
        }
      }
      observations.push({ deviceScaleFactor, cases: caseObservations });
    }

    const [base, scaled] = observations;
    for (const fixtureCase of inventory.cases) {
      const baseCase = base.cases.find(({ id }) => id === fixtureCase.id);
      const scaledCase = scaled.cases.find(({ id }) => id === fixtureCase.id);
      assert.deepEqual(
        { rootRect: baseCase.rootRect, nodes: baseCase.nodes, resolvedGrid: baseCase.resolvedGrid },
        { rootRect: scaledCase.rootRect, nodes: scaledCase.nodes, resolvedGrid: scaledCase.resolvedGrid },
        `DPR 변경으로 CSSOM 또는 CSS px geometry가 달라졌습니다: ${fixtureCase.id}`,
      );
    }
    return { observations, screenshots, browserVersion };
  },
});

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
await mkdir(evidenceDirectory, { recursive: true });
const runDirectory = join(evidenceDirectory, runId);
await mkdir(runDirectory, { recursive: false });
const screenshotRecords = [];
for (const screenshot of captureResult.screenshots) {
  const bytes = Buffer.from(screenshot.data, 'base64');
  const relativePath = join('spec/internal/evidence/c11-3-3-grid-template-areas-precomparison-2026-10-14', runId,
    `${screenshot.caseId}-dpr1.png`).replaceAll('\\', '/');
  await writeFile(join(repositoryRoot, relativePath), bytes, { flag: 'wx' });
  screenshotRecords.push({ caseId: screenshot.caseId, path: relativePath, sha256: sha256(bytes) });
}

const captureToolSha256 = await sha256File(fileURLToPath(import.meta.url));
const reference = {
  schema: 'spinon-css-c11-3-3-grid-template-areas-precomparison/v1',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + sha256(inventoryBytes).slice(0, 12)
    + '-' + sha256(htmlBytes).slice(0, 12)
    + '-' + captureToolSha256.slice(0, 12),
  runId,
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: sha256(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: sha256(htmlBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: captureToolSha256,
    dependencies: [{ path: helperRelativePath, sha256: await sha256File(join(repositoryRoot, helperRelativePath)) }],
  },
  oracle: {
    name: 'Google Chrome for Testing',
    product: captureResult.browserVersion.product,
    revision: captureResult.browserVersion.revision,
    userAgent: captureResult.browserVersion.userAgent,
    executablePath: chromiumPath,
    executableSha256: await sha256File(chromiumPath),
    host: { platform: platform(), architecture: arch(), release: release(), node: process.version },
    flags: browserFlags,
  },
  environment: {
    viewportCssPx: inventory.viewport,
    deviceScaleFactors: inventory.viewport.deviceScaleFactors,
    ...inventory.environment,
  },
  wpt: inventory.wpt,
  comparison: inventory.comparison,
  cases: inventory.cases,
  observations: captureResult.observations,
  screenshots: screenshotRecords,
};
await mkdir(join(repositoryRoot, 'tests/fixtures/css/references'), { recursive: true });
await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`, { flag: 'wx' });
console.log(`Chrome 기준을 저장했습니다: ${outputRelativePath}`);
console.log(`Chrome run ID: ${runId}`);
console.log(`Cases: ${inventory.cases.length}; DPR captures: ${captureResult.observations.length}`);
