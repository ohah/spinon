import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c10/flex-reverse-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c10/flex-reverse.html';
const runtimeFixtureRelativePath = 'tests/fixtures/css/c10/runtime-flex-reverse.js';
const captureRelativePath = 'tools/css-reference/capture-c10-3-1-flex-reverse.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c10-3-1-flex-reverse-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const runtimeFixturePath = join(repositoryRoot, runtimeFixtureRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inventoryBytes, htmlBytes, runtimeFixtureBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(runtimeFixturePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const nodeIds = inventory.cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));
const caseIds = inventory.cases.map(({ id }) => id);

if (inventory.schema !== 'spinon-css-c10-3-1-flex-reverse-inventory/v1'
  || inventory.fixtureId !== 'C10.3.1-flex-reverse-v1'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 240
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || caseIds.length !== 17 || new Set(caseIds).size !== caseIds.length
  || new Set(nodeIds).size !== nodeIds.length
  || !inventory.wpt?.revision?.match(/^[a-f0-9]{40}$/)
  || !inventory.wpt.paths?.length
  || !inventory.cases.every(({ id, nodes, wptPaths }) => typeof id === 'string'
    && Array.isArray(nodes) && nodes.length > 0
    && nodes[0].id && Array.isArray(wptPaths))) {
  throw new Error('C10.3.1 inventory의 고정 계약이 올바르지 않습니다.');
}
if (!inventory.cases.some(({ id }) => id === 'runtime-row-wrap-reverse')) {
  throw new Error('Android/iOS runtime 비교 case가 inventory에 없습니다.');
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error('Chromium 버전을 읽지 못했습니다: '
    + (versionResult.error?.message ?? versionResult.stderr));
}
const cliVersion = versionResult.stdout.trim();
if (cliVersion !== 'Google Chrome 154.0.8037.98') {
  throw new Error('Chromium 기준 버전이 고정값과 다릅니다: ' + cliVersion);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error('DevTools Chromium 기준 버전이 고정값과 다릅니다: '
        + JSON.stringify(browserVersion));
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    await page.send('Emulation.setUserAgentOverride', {
      userAgent: browserVersion.userAgent,
      acceptLanguage: 'en-US',
    });
    await page.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
    await page.send('Emulation.setEmulatedMedia', {
      features: [
        { name: 'prefers-color-scheme', value: 'light' },
        { name: 'forced-colors', value: 'none' },
        { name: 'pointer', value: 'coarse' },
        { name: 'hover', value: 'none' },
      ],
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: 'file://' + htmlPath });
    await loaded;

    const observations = [];
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
          + 'const caseId = ' + JSON.stringify(fixtureCase.id) + ';'
          + 'const nodeIds = ' + JSON.stringify(fixtureCase.nodes.map(({ id }) => id)) + ';'
          + 'const section = document.querySelector("[data-c1031-case=\\\"" + caseId + "\\\"]");'
          + 'for (const candidate of document.querySelectorAll("[data-c1031-case]")) '
          + 'candidate.style.display = candidate === section '
          + '? (caseId === "runtime-row-wrap-reverse" ? "flow-root" : "block") : "none";'
          + 'const harness = section.getBoundingClientRect();'
          + 'const nodes = nodeIds.map(id => {'
          + 'const element = document.getElementById(id);'
          + 'if (!element) throw new Error("fixture node missing: " + id);'
          + 'const rect = element.getBoundingClientRect();'
          + 'const style = getComputedStyle(element);'
          + 'return {id, children:Array.from(element.children, child => child.id),'
          + 'sourceSize:{width:style.width,height:style.height},'
          + 'properties:Object.fromEntries('
          + JSON.stringify(inventory.comparison.computedProperties)
          + '.map(property => [property,style.getPropertyValue(property).trim()])),'
          + 'rect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height}};'
          + '});'
          + 'return JSON.stringify({id:caseId,harness:{x:harness.x,y:harness.y,'
          + 'width:harness.width,height:harness.height},nodes});'
          + '})()';
        const evaluation = await page.send('Runtime.evaluate', {
          expression,
          returnByValue: true,
        });
        const value = evaluation.result?.value;
        if (evaluation.exceptionDetails || typeof value !== 'string' || value.length === 0) {
          throw new Error('Chromium 관찰값을 읽지 못했습니다: '
            + (evaluation.exceptionDetails?.text ?? '결과 없음'));
        }
        const observation = JSON.parse(value);
        const expectedIds = fixtureCase.nodes.map(({ id }) => id);
        if (observation.id !== fixtureCase.id
          || observation.harness.x !== 0 || observation.harness.y !== 0
          || observation.nodes.length !== expectedIds.length
          || JSON.stringify(observation.nodes.map(({ id }) => id)) !== JSON.stringify(expectedIds)
          || !observation.nodes.every((node, index) =>
            JSON.stringify(node.children) === JSON.stringify(fixtureCase.nodes[index].children)
            && inventory.comparison.computedProperties.every(
              (property) => typeof node.properties[property] === 'string',
            )
            && inventory.comparison.rectFields.every((field) => Number.isFinite(node.rect[field])))) {
          throw new Error('Chromium observation이 inventory와 다릅니다: '
            + JSON.stringify({ case: fixtureCase.id, observation }));
        }
        cases.push(observation);
      }
      const result = await page.send('Runtime.evaluate', {
        expression: 'JSON.stringify({width:innerWidth,height:innerHeight,'
          + 'deviceScaleFactor:devicePixelRatio,locale:navigator.language,'
          + 'timeZone:Intl.DateTimeFormat().resolvedOptions().timeZone,'
          + 'dark:matchMedia("(prefers-color-scheme: dark)").matches,'
          + 'coarsePointer:matchMedia("(pointer: coarse)").matches,'
          + 'hover:matchMedia("(hover: hover)").matches})',
        returnByValue: true,
      });
      const environment = JSON.parse(result.result.value);
      if (environment.width !== inventory.viewport.width
        || environment.height !== inventory.viewport.height
        || environment.deviceScaleFactor !== deviceScaleFactor
        || environment.locale !== 'en-US' || environment.timeZone !== 'UTC'
        || environment.dark !== false || environment.coarsePointer !== true
        || environment.hover !== false) {
        throw new Error('Chromium 환경이 fixture 계약과 다릅니다: '
          + JSON.stringify(environment));
      }
      observations.push({
        viewport: {
          width: environment.width,
          height: environment.height,
          deviceScaleFactor: environment.deviceScaleFactor,
        },
        environment: {
          locale: environment.locale,
          timeZone: environment.timeZone,
          dark: environment.dark,
          coarsePointer: environment.coarsePointer,
          hover: environment.hover,
        },
        cases,
      });
    }
    for (const fixtureCase of inventory.cases) {
      const index = caseIds.indexOf(fixtureCase.id);
      if (JSON.stringify(observations[0].cases[index].nodes)
        !== JSON.stringify(observations[1].cases[index].nodes)) {
        throw new Error('DPR 변경이 CSS computed values 또는 CSS px geometry를 바꿨습니다: '
          + fixtureCase.id);
      }
    }
    return { observations, browserVersion };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const capturePath = fileURLToPath(import.meta.url);
const reference = {
  schema: 'spinon-css-c10-3-1-flex-reverse-reference/v1',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + hashBytes(inventoryBytes).slice(0, 12)
    + '-' + (await sha256File(capturePath)).slice(0, 12),
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: hashBytes(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: hashBytes(htmlBytes),
    runtimeSourcePath: runtimeFixtureRelativePath,
    runtimeSourceSha256: hashBytes(runtimeFixtureBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: await sha256File(capturePath),
    dependencies: [{
      path: helperRelativePath,
      sha256: await sha256File(join(repositoryRoot, helperRelativePath)),
    }],
  },
  wpt: inventory.wpt,
  oracle: {
    name: 'Google Chrome',
    product: captureResult.browserVersion.product,
    cliVersion,
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
  },
  comparison: inventory.comparison,
  observations: captureResult.observations,
};
try {
  await access(outputPath);
  throw new Error('기준 reference는 덮어쓰지 않습니다: ' + outputRelativePath);
} catch (error) {
  if (error?.code !== 'ENOENT') throw error;
}
await writeFile(outputPath, JSON.stringify(reference, null, 2) + '\n', { flag: 'wx' });
console.log(JSON.stringify({
  referenceId: reference.referenceId,
  outputPath: outputRelativePath,
  caseCount: inventory.cases.length,
  nodeCount: nodeIds.length,
  wptRevision: inventory.wpt.revision,
  browser: reference.oracle.product,
  observations: reference.observations.length,
}, null, 2));
