import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c06/typed-css-math-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c06/typed-css-math.html';
const captureRelativePath = 'tools/css-reference/capture-c06-typed-css-math.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c06-typed-css-math-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inventoryBytes, htmlBytes] = await Promise.all([readFile(inventoryPath), readFile(htmlPath)]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));

if (inventory.schema !== 'spinon-css-c06-typed-css-math-inventory/v1'
  || inventory.fixtureId !== 'C06.5-typed-css-math-v1'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 800
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || !Array.isArray(inventory.nodes) || inventory.nodes.length !== 26
  || new Set(inventory.nodes.map(({ id }) => id)).size !== inventory.nodes.length) {
  throw new Error('C06.5 inventory의 고정 계약이 올바르지 않습니다.');
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
      const expression = '(() => {'
        + 'const inventory = ' + JSON.stringify(inventory) + ';'
        + 'const nodes = inventory.nodes.map(({ id, properties }) => {'
        + 'const element = document.getElementById(id);'
        + 'if (!element) throw new Error("fixture node를 찾지 못했습니다: " + id);'
        + 'const style = getComputedStyle(element);'
        + 'const rect = element.getBoundingClientRect();'
        + 'const typed = element.computedStyleMap?.();'
        + 'return { id, properties: Object.fromEntries(properties.map(name => [name, style.getPropertyValue(name).trim()])), '
        + 'typed: Object.fromEntries(properties.map(name => { const value = typed?.get(name); return [name, value ? { type: value.constructor.name, text: value.toString() } : null]; })), '
        + 'rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height } };'
        + '});'
        + 'return JSON.stringify({ fixtureId: inventory.fixtureId,'
        + 'viewport: { width: innerWidth, height: innerHeight, deviceScaleFactor: devicePixelRatio },'
        + 'environment: { locale: navigator.language,'
        + 'intlLocale: Intl.DateTimeFormat().resolvedOptions().locale,'
        + 'timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,'
        + 'dark: matchMedia("(prefers-color-scheme: dark)").matches,'
        + 'coarsePointer: matchMedia("(pointer: coarse)").matches,'
        + 'hover: matchMedia("(hover: hover)").matches }, nodes });'
        + '})()';
      const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
      const observationText = evaluation.result?.value;
      if (evaluation.exceptionDetails || typeof observationText !== 'string' || observationText.length === 0) {
        throw new Error('Chromium 관찰값을 읽지 못했습니다: '
          + (evaluation.exceptionDetails?.text ?? '결과 없음'));
      }
      const observation = JSON.parse(observationText);
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
        throw new Error('Chromium 환경 또는 노드 수가 다릅니다: ' + JSON.stringify(observation));
      }
      for (const [index, expected] of inventory.nodes.entries()) {
        const actual = observation.nodes[index];
        if (actual?.id !== expected.id
          || Object.keys(actual.properties ?? {}).join(',') !== expected.properties.join(',')
          || expected.properties.some((name) => typeof actual.properties[name] !== 'string')
          || !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(actual.rect?.[field]))) {
          throw new Error('Chromium observation이 inventory와 다릅니다: ' + expected.id);
        }
      }
      observations.push(observation);
    }
    for (let index = 0; index < inventory.nodes.length; index += 1) {
      const base = observations[0].nodes[index];
      const scaled = observations[1].nodes[index];
      if (JSON.stringify(base.properties) !== JSON.stringify(scaled.properties)
        || JSON.stringify(base.rect) !== JSON.stringify(scaled.rect)) {
        throw new Error('DPR 변경이 CSS 계산값 또는 CSS px geometry를 바꿨습니다: ' + base.id);
      }
    }
    return { observations, browserVersion };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const reference = {
  schema: 'spinon-css-c06-typed-css-math-reference/v1',
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
    viewportCssPx: { width: inventory.viewport.width, height: inventory.viewport.height },
    deviceScaleFactors: inventory.viewport.deviceScaleFactors,
    locale: 'en-US',
    timeZone: 'UTC',
    colorScheme: 'light',
    forcedColors: 'none',
    pointer: 'coarse',
    hover: 'none',
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
console.log('노드 ' + captureResult.observations[0].nodes.length + '개, viewport '
  + inventory.viewport.width + '×' + inventory.viewport.height + ' CSS px, DPR 1·2');
