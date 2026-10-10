import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c08/block-flow-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c08/block-flow.html';
const runtimeRelativePath = 'tests/fixtures/css/c08/runtime-block-paint.js';
const captureRelativePath = 'tools/css-reference/capture-c08-block-flow.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c08-block-flow-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const runtimePath = join(repositoryRoot, runtimeRelativePath);
const [inventoryBytes, htmlBytes, runtimeBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(runtimePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));

if (inventory.schema !== 'spinon-css-c08-block-flow-inventory/v1'
  || inventory.fixtureId !== 'C08-block-flow-v1'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 240
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US'
  || inventory.environment?.timeZone !== 'UTC'
  || !Array.isArray(inventory.nodes) || inventory.nodes.length !== 9
  || inventory.nodes.map(({ id }) => id).join(',') !== 'root,hero,hero-child,group,first,second,hidden,hidden-child,last'
  || inventory.nodes.some(({ properties }) => properties?.join(',')
    !== 'display,background-color,color,font-size,font-family')) {
  throw new Error('C08 inventory의 고정 계약이 올바르지 않습니다.');
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0
  || versionResult.stdout.trim() !== 'Google Chrome 154.0.8037.98') {
  throw new Error('Chromium 기준 버전이 고정값과 다릅니다: '
    + (versionResult.error?.message ?? versionResult.stdout.trim() ?? versionResult.stderr));
}

const inventoryScript = `Object.defineProperty(globalThis, '__SPINON_C08_BLOCK_FLOW_INVENTORY__', {
  configurable: false, enumerable: false, writable: false,
  value: Object.freeze(${JSON.stringify(inventory)}),
});`;
const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error('DevTools Chromium 기준 revision이 고정값과 다릅니다: '
        + JSON.stringify(browserVersion));
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Page.addScriptToEvaluateOnNewDocument', { source: inventoryScript });
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
      const loaded = page.waitForEvent('Page.loadEventFired');
      await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
      await loaded;
      const evaluation = await page.send('Runtime.evaluate', {
        expression: 'document.querySelector("#reference-result")?.textContent ?? ""',
        returnByValue: true,
      });
      const encoded = evaluation.result?.value;
      if (evaluation.exceptionDetails || typeof encoded !== 'string' || !encoded) {
        throw new Error('Chromium C08 결과를 읽지 못했습니다: '
          + (evaluation.exceptionDetails?.text ?? '결과 없음'));
      }
      const observation = JSON.parse(Buffer.from(encoded, 'base64').toString('utf8'));
      if (observation.fixtureId !== inventory.fixtureId
        || observation.viewport.width !== inventory.viewport.width
        || observation.viewport.height !== inventory.viewport.height
        || observation.viewport.deviceScaleFactor !== deviceScaleFactor
        || observation.environment?.locale !== 'en-US'
        || observation.environment?.intlLocale !== 'en-US'
        || observation.environment?.timeZone !== 'UTC'
        || observation.environment?.dark !== false
        || observation.environment?.coarsePointer !== true
        || observation.environment?.hover !== false
        || observation.nodes?.length !== inventory.nodes.length) {
        throw new Error('Chromium C08 환경 또는 관찰 node 수가 다릅니다: '
          + JSON.stringify(observation));
      }
      for (const [index, expected] of inventory.nodes.entries()) {
        const actual = observation.nodes[index];
        if (actual?.id !== expected.id
          || Object.keys(actual.properties ?? {}).join(',') !== expected.properties.join(',')
          || expected.properties.some((property) => typeof actual.properties[property] !== 'string'
            || actual.properties[property].length === 0)
          || !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(actual.rect?.[field]))) {
          throw new Error(`Chromium C08 node 관찰이 inventory와 다릅니다: ${expected.id}`);
        }
      }
      const byId = Object.fromEntries(observation.nodes.map((node) => [node.id, node]));
      if (byId.root.properties.display !== 'block'
        || byId.hero.rect.y !== byId.root.rect.y
        || byId['hero-child'].rect.y !== byId.hero.rect.y
        || byId['hero-child'].properties.color !== byId.hero.properties.color
        || byId['hero-child'].properties['font-size'] !== byId.hero.properties['font-size']
        || byId['hero-child'].properties['font-family'] !== byId.hero.properties['font-family']
        || byId.first.rect.y < byId.hero.rect.y + byId.hero.rect.height
        || byId.second.rect.y < byId.first.rect.y + byId.first.rect.height
        || byId.last.rect.y < byId.second.rect.y + byId.second.rect.height
        || byId.hidden.rect.width !== 0 || byId.hidden.rect.height !== 0
        || byId['hidden-child'].rect.width !== 0 || byId['hidden-child'].rect.height !== 0
        || byId.root.properties['font-size'] !== '16px'
        || byId.root.properties.color !== 'rgb(0, 0, 0)') {
        throw new Error('Chromium C08 기본 Block·숨김·font/color 기준이 예상과 다릅니다.');
      }
      observations.push(observation);
    }
    return {
      observations,
      oracle: {
        name: 'Chromium',
        product: browserVersion.product,
        revision: browserVersion.revision,
        userAgent: browserVersion.userAgent,
      },
    };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const [captureSha256, helperSha256, executableSha256] = await Promise.all([
  sha256File(fileURLToPath(import.meta.url)),
  sha256File(join(repositoryRoot, helperRelativePath)),
  sha256File(chromiumPath),
]);
let os = `${platform()} ${arch()} ${release()}`;
let osBuild = null;
if (platform() === 'darwin') {
  const product = spawnSync('/usr/bin/sw_vers', ['-productVersion'], { encoding: 'utf8' });
  const build = spawnSync('/usr/bin/sw_vers', ['-buildVersion'], { encoding: 'utf8' });
  if (product.status === 0 && build.status === 0) {
    os = `macOS ${product.stdout.trim()}`;
    osBuild = build.stdout.trim();
  }
}
const inventorySha256 = hashBytes(inventoryBytes);
const htmlSha256 = hashBytes(htmlBytes);
const reference = {
  schema: 'spinon-css-c08-block-flow-reference/v1',
  referenceId: `chromium-${platform()}-${arch()}-154.0.8037.98-${inventorySha256.slice(0, 12)}-${captureSha256.slice(0, 12)}-${executableSha256.slice(0, 12)}`,
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256,
    htmlPath: htmlRelativePath,
    htmlSha256,
    runtimeSourcePath: runtimeRelativePath,
    runtimeSourceSha256: hashBytes(runtimeBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: captureSha256,
    dependencies: [{ path: helperRelativePath, sha256: helperSha256 }],
  },
  oracle: {
    ...captureResult.oracle,
    executable: chromiumPath,
    executableSha256,
  },
  environment: {
    os,
    osBuild,
    flags: browserFlags,
    ...inventory.environment,
  },
  viewport: inventory.viewport,
  comparison: inventory.comparison,
  observations: captureResult.observations,
};
await writeFile(join(repositoryRoot, outputRelativePath), `${JSON.stringify(reference, null, 2)}\n`);
console.log(`C08 Chromium 기준 저장: ${outputRelativePath}`);
