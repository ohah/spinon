import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c05/runtime-registered-properties-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c05/runtime-registered-properties.html';
const runtimeFixtureRelativePath = 'tests/fixtures/css/c05/runtime-registered-properties.js';
const captureRelativePath = 'tools/css-reference/capture-c05-runtime-registered-properties.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c05-runtime-registered-properties-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inventoryBytes, htmlBytes, runtimeFixtureBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(join(repositoryRoot, runtimeFixtureRelativePath)),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const viewport = inventory.viewport;
const expectedNodeIds = [
  'app', 'sheet-one', 'declared', 'default', 'invalid', 'inherit-parent', 'not-inherited',
  'inherited', 'late', 'unknown', 'inline-priority', 'color-override', 'duplicate', 'sheet-two',
];

if (inventory.schema !== 'spinon-css-runtime-registered-properties-inventory/v1'
  || inventory.fixtureId !== 'C05.2-runtime-registered-properties-v1'
  || viewport?.width !== 301 || viewport?.height !== 100 || viewport?.deviceScaleFactor !== 1
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || inventory.environment?.colorScheme !== 'light' || inventory.environment?.pointer !== 'coarse'
  || inventory.environment?.hover !== 'none'
  || inventory.nodes?.map(({ id }) => id).join(',') !== expectedNodeIds.join(',')) {
  throw new Error('C05.2 inventory의 고정 계약이 올바르지 않습니다.');
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error(`Chromium 버전을 읽지 못했습니다: ${versionResult.error?.message ?? versionResult.stderr}`);
}
const cliVersion = versionResult.stdout.trim();
if (cliVersion !== 'Google Chrome 154.0.8037.98') {
  throw new Error(`Chromium 기준 버전이 고정값과 다릅니다: ${cliVersion}`);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error(`DevTools Chromium 기준 버전이 고정값과 다릅니다: ${JSON.stringify(browserVersion)}`);
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: viewport.width,
      height: viewport.height,
      deviceScaleFactor: viewport.deviceScaleFactor,
      mobile: false,
      screenWidth: viewport.width,
      screenHeight: viewport.height,
    });
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
    await page.send('Page.navigate', { url: `file://${htmlPath}` });
    await loaded;
    const expression = `(() => {
      const inventory = ${JSON.stringify(inventory)};
      const nodes = inventory.nodes.map(({ id, properties }) => {
        const element = document.getElementById(id);
        if (!element) throw new Error('fixture node를 찾지 못했습니다: ' + id);
        const style = getComputedStyle(element);
        const rect = element.getBoundingClientRect();
        return {
          id,
          properties: Object.fromEntries(properties.map((name) => [name, style.getPropertyValue(name).trim()])),
          rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
        };
      });
      return JSON.stringify({
        fixtureId: inventory.fixtureId,
        viewport: { width: innerWidth, height: innerHeight, deviceScaleFactor: devicePixelRatio },
        environment: {
          locale: navigator.language,
          intlLocale: Intl.DateTimeFormat().resolvedOptions().locale,
          timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
          dark: matchMedia('(prefers-color-scheme: dark)').matches,
          coarsePointer: matchMedia('(pointer: coarse)').matches,
          hover: matchMedia('(hover: hover)').matches,
        },
        nodes,
      });
    })()`;
    const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
    const observationText = evaluation.result?.value;
    if (evaluation.exceptionDetails || typeof observationText !== 'string' || observationText.length === 0) {
      throw new Error(`Chromium 관찰값을 읽지 못했습니다: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
    }
    const observation = JSON.parse(observationText);
    if (observation.fixtureId !== inventory.fixtureId
      || JSON.stringify(observation.viewport) !== JSON.stringify(viewport)
      || observation.environment?.locale !== 'en-US'
      || observation.environment?.intlLocale !== 'en-US'
      || observation.environment?.timeZone !== 'UTC'
      || observation.environment?.dark !== false
      || observation.environment?.coarsePointer !== true
      || observation.environment?.hover !== false
      || observation.nodes?.length !== inventory.nodes.length) {
      throw new Error(`Chromium 환경 또는 노드 수가 다릅니다: ${JSON.stringify(observation)}`);
    }
    for (const [index, expected] of inventory.nodes.entries()) {
      const actual = observation.nodes[index];
      if (actual?.id !== expected.id
        || Object.keys(actual.properties ?? {}).join(',') !== expected.properties.join(',')
        || expected.properties.some((name) => typeof actual.properties[name] !== 'string')
        || !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(actual.rect?.[field]))) {
        throw new Error(`Chromium observation이 inventory와 다릅니다: ${expected.id}`);
      }
    }
    const byId = Object.fromEntries(observation.nodes.map((node) => [node.id, node]));
    if (byId.app.properties.display !== 'flex'
      || byId['sheet-one'].properties.display !== 'none'
      || byId['sheet-two'].properties.display !== 'none'
      || byId.declared.properties.width !== '41px'
      || byId.default.properties.width !== '23px'
      || byId.invalid.properties.width !== '23px'
      || byId['not-inherited'].properties.width !== '23px'
      || byId.inherited.properties.width !== '19px'
      || byId.late.properties.width !== '31px'
      || byId.unknown.properties.width !== '13px'
      || byId['inline-priority'].properties.width !== '47px'
      || byId['color-override'].properties.width !== '11px'
      || byId['color-override'].properties['background-color'] !== 'rgb(255, 102, 0)'
      || byId.duplicate.properties['--duplicate'] !== '3'
      || byId.app.rect.x !== 0 || byId.app.rect.y !== 0
      || byId['color-override'].rect.width !== 11 || byId.duplicate.rect.width !== 0
      || byId['sheet-one'].rect.width !== 0 || byId['sheet-two'].rect.height !== 0) {
      throw new Error(`Chromium @property 동작이 사전 판정과 다릅니다: ${JSON.stringify(observation.nodes)}`);
    }
    const transitionEvaluation = await page.send('Runtime.evaluate', {
      expression: `(() => {
        const app = document.getElementById('app');
        const first = document.getElementById('sheet-one');
        const second = document.getElementById('sheet-two');
        first.textContent = first.textContent.replace('initial-value: 23px;', 'initial-value: 37px;');
        const target = document.getElementById('duplicate');
        app.insertBefore(second, first);
        const style = getComputedStyle(target);
        const rect = target.getBoundingClientRect();
        return JSON.stringify({
          customProperty: style.getPropertyValue('--duplicate').trim(),
          width: style.width,
          rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
        });
      })()`,
      returnByValue: true,
    });
    const transitionText = transitionEvaluation.result?.value;
    if (transitionEvaluation.exceptionDetails || typeof transitionText !== 'string') {
      throw new Error(`Chromium stylesheet 순서 변경 관찰에 실패했습니다: ${transitionEvaluation.exceptionDetails?.text ?? '결과 없음'}`);
    }
    const transition = JSON.parse(transitionText);
    if (transition.customProperty !== '29px' || transition.rect.width <= 0) {
      throw new Error(`Chromium stylesheet 이동 뒤 등록 승자가 예상과 다릅니다: ${JSON.stringify(transition)}`);
    }
    const detachEvaluation = await page.send('Runtime.evaluate', {
      expression: `(() => {
        const app = document.getElementById('app');
        const first = document.getElementById('sheet-one');
        const target = document.getElementById('duplicate');
        window.__spinonC052DetachedSheetOne = first;
        app.removeChild(first);
        const style = getComputedStyle(target);
        const rect = target.getBoundingClientRect();
        return JSON.stringify({
          customProperty: style.getPropertyValue('--duplicate').trim(),
          width: style.width,
          rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
        });
      })()`,
      returnByValue: true,
    });
    const detachText = detachEvaluation.result?.value;
    if (detachEvaluation.exceptionDetails || typeof detachText !== 'string') {
      throw new Error(`Chromium stylesheet 분리 관찰에 실패했습니다: ${detachEvaluation.exceptionDetails?.text ?? '결과 없음'}`);
    }
    const detached = JSON.parse(detachText);
    if (detached.customProperty !== '3' || detached.rect.height !== 0) {
      throw new Error(`Chromium stylesheet 분리 뒤 등록값이 갱신되지 않았습니다: ${JSON.stringify(detached)}`);
    }
    const reinsertEvaluation = await page.send('Runtime.evaluate', {
      expression: `(() => {
        const app = document.getElementById('app');
        const first = window.__spinonC052DetachedSheetOne;
        const target = document.getElementById('duplicate');
        app.appendChild(first);
        window.__spinonC052DetachedSheetOne = null;
        const style = getComputedStyle(target);
        const rect = target.getBoundingClientRect();
        return JSON.stringify({
          customProperty: style.getPropertyValue('--duplicate').trim(),
          width: style.width,
          rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
        });
      })()`,
      returnByValue: true,
    });
    const reinsertText = reinsertEvaluation.result?.value;
    if (reinsertEvaluation.exceptionDetails || typeof reinsertText !== 'string') {
      throw new Error(`Chromium stylesheet 재삽입 관찰에 실패했습니다: ${JSON.stringify(reinsertEvaluation.exceptionDetails ?? '결과 없음')}`);
    }
    const reinserted = JSON.parse(reinsertText);
    if (reinserted.customProperty !== '29px' || reinserted.rect.width <= 0) {
      throw new Error(`Chromium stylesheet 재삽입 뒤 등록 승자가 예상과 다릅니다: ${JSON.stringify(reinserted)}`);
    }
    return {
      observation,
      transitions: {
        sheetTwoMovedBeforeSheetOne: transition,
        sheetOneDetached: detached,
        sheetOneReinsertedAfterSheetTwo: reinserted,
      },
      browserVersion,
    };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const reference = {
  schema: 'spinon-css-runtime-registered-properties-reference/v1',
  referenceId: `chromium-${platform()}-${arch()}-${captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')}-${hashBytes(inventoryBytes).slice(0, 12)}-${(await sha256File(fileURLToPath(import.meta.url))).slice(0, 12)}`,
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: hashBytes(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: hashBytes(htmlBytes),
    runtimeFixturePath: runtimeFixtureRelativePath,
    runtimeFixtureSha256: hashBytes(runtimeFixtureBytes),
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
    viewportCssPx: viewport,
    locale: 'en-US',
    timeZone: 'UTC',
    colorScheme: 'light',
    forcedColors: 'none',
    pointer: 'coarse',
    hover: 'none',
  },
  comparison: inventory.comparison,
  observation: captureResult.observation,
  transitions: captureResult.transitions,
};

try {
  await access(outputPath);
  throw new Error(`기준 reference는 덮어쓰지 않습니다: ${outputRelativePath}`);
} catch (error) {
  if (error?.code !== 'ENOENT') throw error;
}
await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`, { flag: 'wx' });
console.log(`저장: ${outputRelativePath}`);
console.log(`Chromium: ${captureResult.browserVersion.product} ${captureResult.browserVersion.revision}`);
console.log(`노드 ${captureResult.observation.nodes.length}개, viewport ${viewport.width}×${viewport.height} CSS px`);
