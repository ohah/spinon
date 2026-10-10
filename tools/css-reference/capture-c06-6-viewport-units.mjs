import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c06/viewport-units-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c06/viewport-units.html';
const captureRelativePath = 'tools/css-reference/capture-c06-6-viewport-units.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c06-viewport-units-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inventoryBytes, htmlBytes] = await Promise.all([readFile(inventoryPath), readFile(htmlPath)]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));

if (inventory.schema !== 'spinon-css-c06-viewport-units-inventory/v1'
  || inventory.fixtureId !== 'C06.6a-viewport-units-v1'
  || inventory.viewports?.length !== 3
  || JSON.stringify(inventory.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US'
  || inventory.environment?.timeZone !== 'UTC'
  || !Array.isArray(inventory.nodes)
  || new Set(inventory.nodes.map(({ id }) => id)).size !== inventory.nodes.length) {
  throw new Error('C06.6a inventory의 고정 계약이 올바르지 않습니다.');
}

const chromiumPath = await findChromiumExecutable();
const cli = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (cli.error || cli.status !== 0) {
  throw new Error('Chromium 버전을 읽지 못했습니다: ' + (cli.error?.message ?? cli.stderr));
}
const cliVersion = cli.stdout.trim();
if (cliVersion !== 'Google Chrome 154.0.8037.98') {
  throw new Error('Chromium 기준 버전이 고정값과 다릅니다: ' + cliVersion);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error('DevTools Chromium 기준 버전이 고정값과 다릅니다: ' + JSON.stringify(browserVersion));
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
    for (let viewportIndex = 0; viewportIndex < inventory.viewports.length; viewportIndex += 1) {
      const viewport = inventory.viewports[viewportIndex];
      for (const deviceScaleFactor of inventory.deviceScaleFactors) {
        await page.send('Emulation.setDeviceMetricsOverride', {
          width: viewport.width,
          height: viewport.height,
          deviceScaleFactor,
          mobile: false,
          screenWidth: viewport.width,
          screenHeight: viewport.height,
        });
        const expression = '(() => {'
          + 'const inventory = ' + JSON.stringify(inventory) + ';'
          + 'const nodes = inventory.nodes.map(({ id, properties }) => {'
          + 'const element = document.getElementById(id);'
          + 'if (!element) throw new Error("fixture node를 찾지 못했습니다: " + id);'
          + 'const style = getComputedStyle(element);'
          + 'const rect = element.getBoundingClientRect();'
          + 'return { id, properties: Object.fromEntries(properties.map(name => [name, style.getPropertyValue(name).trim()])), '
          + 'rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height } };'
          + '});'
          + 'return JSON.stringify({ fixtureId: inventory.fixtureId,'
          + 'viewport: { width: innerWidth, height: innerHeight, deviceScaleFactor: devicePixelRatio }, nodes });'
          + '})()';
        const result = await page.send('Runtime.evaluate', { expression, returnByValue: true });
        const value = result.result?.value;
        if (result.exceptionDetails || typeof value !== 'string' || value.length === 0) {
          throw new Error('Chromium 관찰값을 읽지 못했습니다: '
            + (result.exceptionDetails?.text ?? '결과 없음'));
        }
        const observation = JSON.parse(value);
        if (observation.fixtureId !== inventory.fixtureId
          || observation.viewport.width !== viewport.width
          || observation.viewport.height !== viewport.height
          || observation.viewport.deviceScaleFactor !== deviceScaleFactor
          || observation.nodes?.length !== inventory.nodes.length) {
          throw new Error('Chromium viewport 또는 node 수가 다릅니다: ' + JSON.stringify(observation));
        }
        for (const [index, expected] of inventory.nodes.entries()) {
          const actual = observation.nodes[index];
          if (actual?.id !== expected.id
            || expected.properties.some((name) => typeof actual.properties?.[name] !== 'string')
            || !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(actual.rect?.[field]))) {
            throw new Error('Chromium observation이 inventory와 다릅니다: ' + expected.id);
          }
        }
        observations.push(observation);
      }
    }

    for (let viewportIndex = 0; viewportIndex < inventory.viewports.length; viewportIndex += 1) {
      const one = observations[viewportIndex * 2];
      const two = observations[viewportIndex * 2 + 1];
      if (JSON.stringify(one.nodes.map(({ properties, rect }) => ({ properties, rect })))
        !== JSON.stringify(two.nodes.map(({ properties, rect }) => ({ properties, rect })))) {
        throw new Error('DPR 변경이 CSS 계산값 또는 CSS px geometry를 바꿨습니다.');
      }
      const byId = new Map(one.nodes.map((node) => [node.id, node]));
      for (const property of ['width', 'height']) {
        const values = ['small-units', 'large-units', 'dynamic-units']
          .map((id) => byId.get(id).properties[property]);
        if (new Set(values).size !== 1) {
          throw new Error('현재 native surface 비교 기준에서 small/large/dynamic 단위가 다릅니다: '
            + JSON.stringify({ viewport: one.viewport, property, values }));
        }
      }
    }
    return { observations, browserVersion };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const reference = {
  schema: 'spinon-css-c06-viewport-units-reference/v1',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + hashBytes(inventoryBytes).slice(0, 12)
    + '-' + (await sha256File(fileURLToPath(import.meta.url))).slice(0, 12),
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: hashBytes(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: hashBytes(htmlBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: await sha256File(fileURLToPath(import.meta.url)),
    dependencies: [{ path: helperRelativePath, sha256: await sha256File(join(repositoryRoot, helperRelativePath)) }],
  },
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
    viewportsCssPx: inventory.viewports,
    deviceScaleFactors: inventory.deviceScaleFactors,
    locale: inventory.environment.locale,
    timeZone: inventory.environment.timeZone,
    colorScheme: inventory.environment.colorScheme,
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
console.log('저장: ' + outputRelativePath);
console.log('Chromium: ' + captureResult.browserVersion.product + ' ' + captureResult.browserVersion.revision);
console.log('viewport ' + inventory.viewports.length + '개, DPR 1·2, node ' + inventory.nodes.length + '개');
