import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { arch, platform, release } from 'node:os';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133.html';
const captureRelativePath = 'tools/css-reference/capture-c11-3-3-unknown-named-area-precomparison.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c11-3-3-unknown-named-area-implicit-lines-precomparison-v1.json';
const evidenceDirectoryRelativePath = 'spec/internal/evidence/c11-3-3-unknown-named-area-precomparison-2026-10-20';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const evidenceDirectory = join(repositoryRoot, evidenceDirectoryRelativePath);
const runId = `c1133-unknown-name-chrome-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`;
const [inventoryBytes, htmlBytes] = await Promise.all([readFile(inventoryPath), readFile(htmlPath)]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
assert.equal(inventory.schema, 'spinon-css-c11-3-3-unknown-named-area-implicit-lines-inventory/v1');
assert.equal(inventory.fixtureId, 'C11.3.3-unknown-named-area-implicit-lines-precomparison-v1');
assert.deepEqual(inventory.viewport.deviceScaleFactors, [1, 2]);
assert.equal(inventory.cases.length, 5);
await access(outputPath).then(
  () => { throw new Error(`기존 Chrome 기준을 덮어쓰지 않습니다: ${outputRelativePath}`); },
  (error) => { if (error.code !== 'ENOENT') throw error; },
);

function rectFields(rect) {
  return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
}

function parsePxTracks(serialized) {
  const tracks = serialized.trim().split(/\s+/).filter(Boolean).map((value) => {
    const match = value.match(/^(-?(?:\d+(?:\.\d*)?|\.\d+))px$/);
    if (!match) throw new Error(`Chrome resolved track이 고정 px가 아닙니다: ${serialized}`);
    const parsed = Number(match[1]);
    if (!Number.isFinite(parsed) || parsed < 0) throw new Error(`잘못된 resolved track: ${value}`);
    return parsed;
  });
  return tracks;
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
      const cases = [];
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
          + `const properties=Object.fromEntries(${JSON.stringify(inventory.comparison.computedProperties)}`
          + '.map(name=>[name,style.getPropertyValue(name).trim()]));'
          + 'return{id:element.id,parentId:element.parentElement?.closest("[data-node]")?.id??null,'
          + 'properties,rect:{x:rect.left-rootRect.left,y:rect.top-rootRect.top,width:rect.width,height:rect.height}};});'
          + 'const rootStyle=getComputedStyle(root);'
          + 'return JSON.stringify({id:caseId,rootId:root.id,viewport:{width:innerWidth,height:innerHeight,'
          + 'deviceScaleFactor:devicePixelRatio},environment:{direction:rootStyle.direction,'
          + 'writingMode:rootStyle.writingMode},rootRect:{x:rootRect.x,y:rootRect.y,'
          + 'width:rootRect.width,height:rootRect.height},nodes});})()';
        const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
        assert.equal(evaluation.exceptionDetails, undefined,
          `${fixtureCase.id}: ${evaluation.exceptionDetails?.text ?? 'Chrome 캡처 실패'}`);
        assert.equal(typeof evaluation.result?.value, 'string', `${fixtureCase.id}: observation JSON이 없습니다.`);
        const observation = JSON.parse(evaluation.result.value);
        assert.deepEqual(observation.nodes.map(({ id }) => id), fixtureCase.nodeIds);
        assert.deepEqual(observation.environment, {
          direction: inventory.environment.direction,
          writingMode: inventory.environment.writingMode,
        });
        assert.deepEqual(rectFields(observation.rootRect), { x: 0, y: 0, width: 160, height: 120 });
        const root = observation.nodes[0];
        const columns = parsePxTracks(root.properties['grid-template-columns']);
        const rows = parsePxTracks(root.properties['grid-template-rows']);
        const columnNode = observation.nodes.find(({ id }) => id !== fixtureCase.rootId
          && (id === fixtureCase.nodeIds.at(-1)));
        assert.ok(columnNode, `${fixtureCase.id}: target item이 없습니다.`);
        const actualRect = rectFields(columnNode.rect);
        assert.deepEqual(columns, fixtureCase.expectedColumns, `${fixtureCase.id}: columns CSSOM`);
        assert.deepEqual(rows, fixtureCase.expectedRows, `${fixtureCase.id}: rows CSSOM`);
        for (const field of inventory.comparison.rectFields) {
          assert.ok(Number.isFinite(actualRect[field]), `${fixtureCase.id}/${field}가 유한하지 않습니다.`);
          assert.ok(Math.abs(actualRect[field] - fixtureCase.expectedRect[field]) <= 0.5,
            `${fixtureCase.id}/${field}: ${actualRect[field]} != ${fixtureCase.expectedRect[field]}`);
        }
        observation.resolvedGrid = { columns, rows, derivedFromCssomResolvedValues: true };
        cases.push(observation);
        const screenshot = await page.send('Page.captureScreenshot', {
          format: 'png', fromSurface: true, captureBeyondViewport: false,
        });
        screenshots.push({ caseId: fixtureCase.id, deviceScaleFactor, data: screenshot.data });
      }
      observations.push({ deviceScaleFactor, cases });
    }
    for (let index = 0; index < observations[0].cases.length; index += 1) {
      const withoutDpr = ({ viewport, ...observation }) => ({
        ...observation,
        viewport: { width: viewport.width, height: viewport.height },
      });
      assert.deepEqual(withoutDpr(observations[0].cases[index]), withoutDpr(observations[1].cases[index]),
        `DPR 변경으로 CSSOM 또는 CSS px geometry가 달라졌습니다: ${observations[0].cases[index].id}`);
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
  const relativePath = join(evidenceDirectoryRelativePath, runId,
    `${screenshot.caseId}-dpr${screenshot.deviceScaleFactor}.png`).replaceAll('\\', '/');
  await writeFile(join(repositoryRoot, relativePath), bytes, { flag: 'wx' });
  screenshotRecords.push({
    caseId: screenshot.caseId,
    deviceScaleFactor: screenshot.deviceScaleFactor,
    path: relativePath,
    sha256: sha256(bytes),
  });
}

const captureToolSha256 = await sha256File(fileURLToPath(import.meta.url));
const reference = {
  schema: 'spinon-css-c11-3-3-unknown-named-area-implicit-lines-precomparison/v1',
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
