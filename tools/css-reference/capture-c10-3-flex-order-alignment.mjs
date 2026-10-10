import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c10/flex-order-alignment-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c10/flex-order-alignment.html';
const runtimeRelativePath = 'tests/fixtures/css/c10/runtime-flex-order-alignment.js';
const captureRelativePath = 'tools/css-reference/capture-c10-3-flex-order-alignment.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c10-3-flex-order-alignment-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const runtimePath = join(repositoryRoot, runtimeRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inventoryBytes, htmlBytes, runtimeBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(runtimePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const caseIds = inventory.cases.map(({ id }) => id);
const nodeIds = inventory.cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));
const expectedSchema = 'spinon-css-c10-3-2-flex-order-inventory/v1';

if (inventory.schema !== expectedSchema
  || inventory.fixtureId !== 'C10.3.2-flex-order-v1'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 240
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || caseIds.length !== 11 || new Set(caseIds).size !== caseIds.length
  || new Set(nodeIds).size !== nodeIds.length
  || inventory.wpt?.revision !== 'd5a765f1089ce6d3f72300281481edf3dddff7f3'
  || !inventory.wpt.paths?.length
  || !inventory.cases.every(({ id, nodes, wptPaths }) => typeof id === 'string'
    && Array.isArray(nodes) && nodes.length > 0
    && nodes[0].id && Array.isArray(wptPaths))) {
  throw new Error('C10.3.2 inventory의 고정 계약이 올바르지 않습니다.');
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
          + 'const section = document.querySelector("[data-c1032-case=\\\"" + caseId + "\\\"]");'
          + 'for (const candidate of document.querySelectorAll("[data-c1032-case]")) '
          + 'candidate.style.display = candidate === section ? "block" : "none";'
          + 'if (!section) throw new Error("fixture case missing: " + caseId);'
          + 'const nodes = nodeIds.map(id => {'
          + 'const element = document.getElementById(id);'
          + 'if (!element) throw new Error("fixture node missing: " + id);'
          + 'const rect = element.getBoundingClientRect();'
          + 'const style = getComputedStyle(element);'
          + 'const typedOrder = element.computedStyleMap().get("order");'
          + 'if (!typedOrder || typedOrder.unit !== "number" || !Number.isInteger(typedOrder.value)) '
          + 'throw new Error("Typed OM order가 integer number가 아닙니다: " + id);'
          + 'return {id,children:Array.from(element.children, child => child.id),'
          + 'properties:Object.fromEntries(' + JSON.stringify(inventory.comparison.computedProperties)
          + '.map(property => [property,style.getPropertyValue(property).trim()])), '
          + 'typed:{order:typedOrder.value},'
          + 'rect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height}};'
          + '});'
          + 'return JSON.stringify({id:caseId,nodes});'
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
        const expectedNodes = fixtureCase.nodes;
        if (observation.id !== fixtureCase.id
          || observation.nodes.length !== expectedNodes.length
          || JSON.stringify(observation.nodes.map(({ id }) => id))
            !== JSON.stringify(expectedNodes.map(({ id }) => id))
          || !observation.nodes.every((node, index) =>
            JSON.stringify(node.children) === JSON.stringify(expectedNodes[index].children)
            && inventory.comparison.computedProperties.every(
              (property) => typeof node.properties[property] === 'string',
            )
            && typeof node.typed.order === 'number'
            && inventory.comparison.rectFields.every((field) => Number.isFinite(node.rect[field])))) {
          throw new Error('Chromium observation이 inventory와 다릅니다: '
            + JSON.stringify({ case: fixtureCase.id, observation }));
        }
        cases.push(observation);
      }
      const environmentResult = await page.send('Runtime.evaluate', {
        expression: 'JSON.stringify({width:innerWidth,height:innerHeight,'
          + 'deviceScaleFactor:devicePixelRatio,locale:navigator.language,'
          + 'timeZone:Intl.DateTimeFormat().resolvedOptions().timeZone,'
          + 'dark:matchMedia("(prefers-color-scheme: dark)").matches,'
          + 'forced:matchMedia("(forced-colors: active)").matches,'
          + 'coarsePointer:matchMedia("(pointer: coarse)").matches,'
          + 'hover:matchMedia("(hover: hover)").matches})',
        returnByValue: true,
      });
      const environment = JSON.parse(environmentResult.result.value);
      if (environment.width !== inventory.viewport.width
        || environment.height !== inventory.viewport.height
        || environment.deviceScaleFactor !== deviceScaleFactor
        || environment.locale !== inventory.environment.locale
        || environment.timeZone !== inventory.environment.timeZone
        || environment.dark !== false || environment.forced !== false
        || environment.coarsePointer !== true || environment.hover !== false) {
        throw new Error('Chromium 환경이 fixture 계약과 다릅니다: ' + JSON.stringify(environment));
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
          forcedColors: environment.forced,
          coarsePointer: environment.coarsePointer,
          hover: environment.hover,
        },
        cases,
      });
    }
    if (JSON.stringify(observations[0].cases) !== JSON.stringify(observations[1].cases)) {
      throw new Error('DPR 변경이 CSS computed value 또는 CSS px geometry를 바꿨습니다.');
    }
    return { observations, browserVersion };
  },
});

const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const capturePath = fileURLToPath(import.meta.url);
const reference = {
  schema: 'spinon-css-c10-3-2-flex-order-reference/v1',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + hash(inventoryBytes).slice(0, 12)
    + '-' + (await sha256File(capturePath)).slice(0, 12),
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: hash(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: hash(htmlBytes),
    runtimeSourcePath: runtimeRelativePath,
    runtimeSourceSha256: hash(runtimeBytes),
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
