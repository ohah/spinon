import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c09/block-formatting-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c09/block-formatting.html';
const captureRelativePath = 'tools/css-reference/capture-c09-block-formatting.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c09-block-formatting-v2.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inventoryBytes, htmlBytes] = await Promise.all([readFile(inventoryPath), readFile(htmlPath)]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));

function flatten(tree, parentId = null, result = []) {
  if (!tree || typeof tree.id !== 'string' || tree.id.length === 0) {
    throw new Error('inventory tree node에는 id가 필요합니다.');
  }
  result.push({ id: tree.id, parentId });
  if (tree.children !== undefined && !Array.isArray(tree.children)) {
    throw new Error('inventory children은 배열이어야 합니다: ' + tree.id);
  }
  for (const child of tree.children ?? []) flatten(child, tree.id, result);
  return result;
}

if (inventory.schema !== 'spinon-css-c09-block-formatting-inventory/v2'
  || inventory.fixtureId !== 'C09-block-formatting-precomparison-v2'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 240
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || inventory.environment?.colorScheme !== 'light'
  || !Array.isArray(inventory.cases) || inventory.cases.length !== 30
  || !Array.isArray(inventory.comparison?.computedProperties)
  || !Array.isArray(inventory.comparison?.typedProperties)) {
  throw new Error('C09 inventory의 고정 계약이 올바르지 않습니다.');
}

const cases = inventory.cases.map((entry) => ({ ...entry, nodes: flatten(entry.tree) }));
const allNodeIds = cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));
const allHarnessIds = cases.map(({ harnessId }) => harnessId);
if (new Set(cases.map(({ id }) => id)).size !== cases.length
  || new Set(allNodeIds).size !== allNodeIds.length
  || new Set(allHarnessIds).size !== allHarnessIds.length
  || allNodeIds.some((id) => allHarnessIds.includes(id))) {
  throw new Error('C09 inventory node 또는 harness ID가 중복됩니다.');
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
    await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
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
        + 'const flatten = (tree, parentId = null, result = []) => {'
        + 'result.push({ id: tree.id, parentId });'
        + 'for (const child of tree.children ?? []) flatten(child, tree.id, result);'
        + 'return result; };'
        + 'const cases = inventory.cases.map(entry => ({ ...entry, nodes: flatten(entry.tree) }));'
        + 'const computedNames = inventory.comparison.computedProperties;'
        + 'const typedNames = inventory.comparison.typedProperties;'
        + 'const observations = cases.map(entry => {'
        + 'const wrapper = document.getElementById(entry.harnessId);'
        + 'if (!wrapper) throw new Error("fixture harness를 찾지 못했습니다: " + entry.harnessId);'
        + 'const wrapperRect = wrapper.getBoundingClientRect();'
        + 'const wrapperStyle = getComputedStyle(wrapper);'
        + 'const actualElements = Array.from(wrapper.querySelectorAll("[id]"));'
        + 'const expectedIds = entry.nodes.map(node => node.id);'
        + 'const actualIds = actualElements.map(element => element.id);'
        + 'if (JSON.stringify(actualIds) !== JSON.stringify(expectedIds)) throw new Error("DOM preorder가 inventory와 다릅니다: " + entry.id + " / " + JSON.stringify(actualIds));'
        + 'const nodes = entry.nodes.map(expected => {'
        + 'const element = document.getElementById(expected.id);'
        + 'const expectedParent = expected.parentId ?? entry.harnessId;'
        + 'if (element.parentElement?.id !== expectedParent) throw new Error("DOM parent가 inventory와 다릅니다: " + expected.id);'
        + 'const style = getComputedStyle(element);'
        + 'const typed = element.computedStyleMap?.();'
        + 'const rect = element.getBoundingClientRect();'
        + 'const readRect = box => ({ x: box.left - wrapperRect.left, y: box.top - wrapperRect.top, width: box.width, height: box.height });'
        + 'return { id: expected.id, parentId: expected.parentId, properties: Object.fromEntries(computedNames.map(name => [name, style.getPropertyValue(name).trim()])), '
        + 'typed: Object.fromEntries(typedNames.map(name => { const value = typed?.get(name); return [name, value ? { type: value.constructor.name, text: value.toString() } : null]; })), '
        + 'rect: readRect(rect), documentRect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height } }; });'
        + 'return { id: entry.id, harnessId: entry.harnessId, harness: { display: wrapperStyle.display, direction: wrapperStyle.direction, writingMode: wrapperStyle.writingMode, position: wrapperStyle.position, top: wrapperStyle.top, left: wrapperStyle.left, width: wrapperStyle.width, height: wrapperStyle.height, margin: [wrapperStyle.marginTop, wrapperStyle.marginRight, wrapperStyle.marginBottom, wrapperStyle.marginLeft], padding: [wrapperStyle.paddingTop, wrapperStyle.paddingRight, wrapperStyle.paddingBottom, wrapperStyle.paddingLeft], borderWidth: [wrapperStyle.borderTopWidth, wrapperStyle.borderRightWidth, wrapperStyle.borderBottomWidth, wrapperStyle.borderLeftWidth], rect: { x: wrapperRect.x, y: wrapperRect.y, width: wrapperRect.width, height: wrapperRect.height } }, nodes }; });'
        + 'return JSON.stringify({ fixtureId: inventory.fixtureId, viewport: { width: innerWidth, height: innerHeight, deviceScaleFactor: devicePixelRatio }, environment: { locale: navigator.language, intlLocale: Intl.DateTimeFormat().resolvedOptions().locale, timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone, dark: matchMedia("(prefers-color-scheme: dark)").matches, coarsePointer: matchMedia("(pointer: coarse)").matches, hover: matchMedia("(hover: hover)").matches }, cases: observations });'
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
        || observation.cases?.length !== cases.length) {
        throw new Error('Chromium 환경 또는 case 수가 다릅니다: ' + JSON.stringify(observation));
      }
      for (const [caseIndex, expectedCase] of cases.entries()) {
        const actualCase = observation.cases[caseIndex];
        if (actualCase?.id !== expectedCase.id
          || actualCase.harnessId !== expectedCase.harnessId
          || actualCase.nodes?.length !== expectedCase.nodes.length
          || actualCase.nodes.some((node, nodeIndex) => node.id !== expectedCase.nodes[nodeIndex].id
            || node.parentId !== expectedCase.nodes[nodeIndex].parentId
            || Object.keys(node.properties ?? {}).join(',') !== inventory.comparison.computedProperties.join(','))
          || !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(actualCase.harness.rect?.[field]))
          || actualCase.nodes.some((node) => !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(node.rect?.[field])))) {
          throw new Error('Chromium 관찰값이 inventory와 다릅니다: ' + expectedCase.id);
        }
        if (actualCase.harness.display !== 'flow-root'
          || actualCase.harness.direction !== 'ltr'
          || actualCase.harness.writingMode !== 'horizontal-tb'
          || actualCase.harness.position !== 'absolute'
          || actualCase.harness.top !== '0px' || actualCase.harness.left !== '0px'
          || actualCase.harness.width !== '320px'
          || actualCase.harness.height !== '240px'
          || actualCase.harness.margin.some((value) => value !== '0px')
          || actualCase.harness.padding.some((value) => value !== '0px')
          || actualCase.harness.borderWidth.some((value) => value !== '0px')
          || actualCase.harness.rect.x !== 0 || actualCase.harness.rect.y !== 0
          || actualCase.harness.rect.width !== 320 || actualCase.harness.rect.height !== 240) {
          throw new Error('Chromium fixture harness 경계가 달라졌습니다: ' + expectedCase.id);
        }
        if (actualCase.nodes.some((node) => node.properties.direction !== 'ltr'
          || node.properties['writing-mode'] !== 'horizontal-tb'
          || node.properties.position !== 'static'
          || node.properties.float !== 'none'
          || node.properties.clear !== 'none'
          || node.properties.transform !== 'none'
          || !['block', 'flow-root', 'none'].includes(node.properties.display))) {
          throw new Error('Chromium fixture node가 C09 입력 profile 밖입니다: ' + expectedCase.id);
        }
      }
      observations.push(observation);
    }
    for (let caseIndex = 0; caseIndex < cases.length; caseIndex += 1) {
      const base = observations[0].cases[caseIndex];
      const scaled = observations[1].cases[caseIndex];
      if (JSON.stringify(base.harness) !== JSON.stringify(scaled.harness)
        || JSON.stringify(base.nodes) !== JSON.stringify(scaled.nodes)) {
        throw new Error('DPR 변경이 CSS computed 값 또는 geometry를 바꿨습니다: ' + base.id);
      }
    }
    return { observations, browserVersion };
  },
});

const hashBytes = (bytes) => createHash('sha256').update(bytes).digest('hex');
const reference = {
  schema: 'spinon-css-c09-block-formatting-reference/v2',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + hashBytes(inventoryBytes).slice(0, 12)
    + '-' + hashBytes(htmlBytes).slice(0, 12)
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
console.log('저장: ' + outputRelativePath);
console.log('Chromium: ' + captureResult.browserVersion.product + ' ' + captureResult.browserVersion.revision);
console.log('case ' + cases.length + '개, app node ' + allNodeIds.length + '개, viewport '
  + inventory.viewport.width + '×' + inventory.viewport.height + ' CSS px, DPR 1·2');
