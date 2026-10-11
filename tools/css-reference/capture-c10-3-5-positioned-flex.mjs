import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { arch, platform, release } from 'node:os';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const root = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = 'tests/fixtures/css/c10/positioned-flex-inventory.json';
const htmlPath = 'tests/fixtures/css/c10/positioned-flex.html';
const capturePath = 'tools/css-reference/capture-c10-3-5-positioned-flex.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const outputPath = 'tests/fixtures/css/references/c10-3-5-positioned-flex-v1.json';
const replaceReference = process.argv.includes('--replace-reference');
if (process.argv.slice(2).some((argument) => argument !== '--replace-reference')) {
  throw new Error('인수는 --replace-reference만 허용합니다.');
}

const [inventoryBytes, htmlBytes] = await Promise.all([
  readFile(join(root, inventoryPath)),
  readFile(join(root, htmlPath)),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const expectedCases = [
  'direct-row-center-end', 'direct-row-reverse', 'direct-column-center',
  'mixed-insets', 'different-containing-block', 'block-wrapper-static-position',
  'paint-order', 'nested-paint-scope', 'direct-column-reverse', 'wrap-two-lines',
  'wrap-reverse-two-lines', 'direct-align-self-values', 'negative-space-around',
  'negative-space-evenly', 'safe-center-overflow', 'unsafe-center-overflow',
  'margin-border-content-edge', 'auto-margin-zero', 'inset-axis-matrix', 'display-none',
];
const allIds = inventory.cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));
if (inventory.schema !== 'spinon-css-c10-3-5-positioned-flex-inventory/v1'
  || inventory.fixtureId !== 'C10.3.5-positioned-flex-v1'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 240
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || inventory.environment?.writingMode !== 'horizontal-tb'
  || inventory.environment?.direction !== 'ltr'
  || inventory.wpt?.revision !== 'd5a765f1089ce6d3f72300281481edf3dddff7f3'
  || inventory.wpt?.execution !== 'not-run'
  || inventory.cases.some(({ nodes }) => nodes.some(({ style }) => typeof style !== 'string'))
  || JSON.stringify(inventory.cases.map(({ id }) => id)) !== JSON.stringify(expectedCases)
  || new Set(allIds).size !== allIds.length) {
  throw new Error('C10.3.5 inventory의 고정 계약, WPT 기준 또는 고유 ID가 올바르지 않습니다.');
}
for (const fixtureCase of inventory.cases) {
  const ids = fixtureCase.nodes.map(({ id }) => id);
  if (new Set(ids).size !== ids.length) throw new Error(`case 안에 node ID가 중복됩니다: ${fixtureCase.id}`);
  for (const node of fixtureCase.nodes) {
    if (node.children.some((child) => !ids.includes(child))) {
      throw new Error(`case 밖의 자식 node입니다: ${fixtureCase.id}/${node.id}`);
    }
  }
}
if (!replaceReference) {
  try {
    await access(join(root, outputPath));
    throw new Error(`기존 Chromium 기준은 덮어쓰지 않습니다: ${outputPath}`);
  } catch (error) {
    if (error?.code !== 'ENOENT') throw error;
  }
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0
  || versionResult.stdout.trim() !== 'Google Chrome 154.0.8037.98') {
  throw new Error(`고정 Chrome 154 실행 파일이 아닙니다: ${versionResult.stdout?.trim() ?? versionResult.stderr}`);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error(`고정 DevTools Chrome 기준이 아닙니다: ${JSON.stringify(browserVersion)}`);
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
    await page.send('Page.navigate', { url: pathToFileURL(join(root, htmlPath)).href });
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
        const expression = `(() => {
          const caseId = ${JSON.stringify(fixtureCase.id)};
          const expected = ${JSON.stringify(fixtureCase.nodes)};
          const section = document.querySelector('[data-c1035-case="' + caseId + '"]');
          if (!section) throw new Error('fixture case missing: ' + caseId);
          for (const candidate of document.querySelectorAll('[data-c1035-case]')) {
            candidate.style.display = candidate === section ? 'block' : 'none';
          }
          const found = Array.from(section.querySelectorAll('[data-c1035-node]'));
          const readNode = element => {
            const rect = element.getBoundingClientRect();
            const style = getComputedStyle(element);
            const children = Array.from(element.children)
              .filter(child => child.hasAttribute('data-c1035-node')).map(child => child.id);
            return {
              id: element.id,
              input: { tag: element.localName, style: element.getAttribute('style') ?? '',
                parentId: element.parentElement?.closest('[data-c1035-node]')?.id ?? null },
              children,
              hasLayoutBox: element.getClientRects().length > 0,
              owner: (() => {
                if (!element.getClientRects().length) return 'none';
                for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
                  if (ancestor.hasAttribute('data-c1035-node')
                    && getComputedStyle(ancestor).position !== 'static'
                    && ancestor.getClientRects().length) return ancestor.id;
                }
                return 'viewport';
              })(),
              staticPositionOwner: !element.getClientRects().length
                || getComputedStyle(element).position !== 'absolute' ? null
                : (getComputedStyle(element.parentElement).display === 'flex'
                  ? element.parentElement.id
                  : element.parentElement?.closest('[data-c1035-node]')?.id ?? null),
              properties: Object.fromEntries(${JSON.stringify(inventory.comparison.computedProperties)}
                .map(property => [property, style.getPropertyValue(property).trim()])),
              rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
            };
          };
          const nodes = expected.map((spec, index) => {
            const element = found[index];
            if (!element || element.id !== spec.id) {
              throw new Error('fixture node order mismatch: ' + caseId + '/' + spec.id);
            }
            const node = readNode(element);
            if (node.input.style !== spec.style) {
              throw new Error('fixture style mismatch: ' + caseId + '/' + spec.id);
            }
            if (JSON.stringify(node.children) !== JSON.stringify(spec.children)) {
              throw new Error('fixture child order mismatch: ' + caseId + '/' + spec.id);
            }
            if (spec.expectedContainingBlock !== undefined && node.owner !== spec.expectedContainingBlock) {
              throw new Error('containing block mismatch: ' + caseId + '/' + spec.id
                + ' expected=' + spec.expectedContainingBlock + ' actual=' + node.owner);
            }
            if (spec.expectedHasLayoutBox !== undefined
              && node.hasLayoutBox !== spec.expectedHasLayoutBox) {
              throw new Error('layout box mismatch: ' + caseId + '/' + spec.id);
            }
            if (spec.expectedStaticPositionOwner !== undefined
              && node.staticPositionOwner !== spec.expectedStaticPositionOwner) {
              throw new Error('static-position owner mismatch: ' + caseId + '/' + spec.id
                + ' expected=' + spec.expectedStaticPositionOwner
                + ' actual=' + node.staticPositionOwner);
            }
            return node;
          });
          if (found.length !== expected.length) throw new Error('unexpected fixture nodes: ' + caseId);
          const probe = ${JSON.stringify(fixtureCase.paintProbe ?? null)};
          let paintOrder = null;
          if (probe) {
            paintOrder = document.elementsFromPoint(probe.x, probe.y)
              .filter(element => element.hasAttribute('data-c1035-node')).map(element => element.id);
          }
          const hasAbsoluteAncestor = element => {
            for (let ancestor = element.parentElement; ancestor && ancestor !== section;
              ancestor = ancestor.parentElement) {
              if (ancestor.hasAttribute('data-c1035-node')
                && getComputedStyle(ancestor).position === 'absolute') return true;
            }
            return false;
          };
          const absoluteElements = nodes
            .filter(node => node.properties.position === 'absolute' && node.hasLayoutBox)
            .map(node => document.getElementById(node.id));
          let flowInvariant = null;
          if (absoluteElements.length > 0) {
            const originals = absoluteElements.map(element => element.getAttribute('style'));
            try {
              absoluteElements.forEach(element => element.style.setProperty('display', 'none', 'important'));
              flowInvariant = expected.map(spec => document.getElementById(spec.id))
                .filter(element => element && getComputedStyle(element).position !== 'absolute'
                  && !hasAbsoluteAncestor(element))
                .map(element => ({ id: element.id, rect: (() => {
                  const rect = element.getBoundingClientRect();
                  return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
                })() }));
            } finally {
              absoluteElements.forEach((element, index) => {
                if (originals[index] === null) element.removeAttribute('style');
                else element.setAttribute('style', originals[index]);
              });
            }
            const baselineFlow = nodes
              .filter(node => node.properties.position !== 'absolute')
              .filter(node => {
                const element = document.getElementById(node.id);
                return element && !hasAbsoluteAncestor(element);
              })
              .map(node => ({ id: node.id, rect: node.rect }));
            if (JSON.stringify(flowInvariant) !== JSON.stringify(baselineFlow)) {
              throw new Error('absolute child 전체 제거가 in-flow frame을 바꿨습니다: ' + caseId);
            }
          }
          const root = found[0];
          const rootStyle = getComputedStyle(root);
          return JSON.stringify({ id: caseId, nodes, paintOrder, flowInvariant,
            rootStyle: { writingMode: rootStyle.writingMode, direction: rootStyle.direction } });
        })()`;
        const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
        const value = evaluation.result?.value;
        if (evaluation.exceptionDetails || typeof value !== 'string') {
          throw new Error(`Chromium 관찰 실패 ${fixtureCase.id}: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
        }
        const observation = JSON.parse(value);
        const expectedIds = fixtureCase.nodes.map(({ id }) => id);
        if (observation.id !== fixtureCase.id
          || JSON.stringify(observation.nodes.map(({ id }) => id)) !== JSON.stringify(expectedIds)
          || observation.rootStyle.writingMode !== inventory.environment.writingMode
          || observation.rootStyle.direction !== inventory.environment.direction
          || !observation.nodes.every((node) => inventory.comparison.computedProperties
            .every((property) => typeof node.properties[property] === 'string')
            && inventory.comparison.rectFields.every((field) => Number.isFinite(node.rect[field])))) {
          throw new Error(`Chromium observation이 inventory와 다릅니다: ${fixtureCase.id}`);
        }
        if (fixtureCase.paintProbe
          && JSON.stringify(observation.paintOrder.slice(0, fixtureCase.nodes.length - 1))
            !== JSON.stringify(fixtureCase.paintProbe.topToBottom)) {
          throw new Error(`paint order probe가 예상과 다릅니다: ${JSON.stringify(observation.paintOrder)}`);
        }
        cases.push(observation);
      }
      const envResult = await page.send('Runtime.evaluate', {
        expression: 'JSON.stringify({width:innerWidth,height:innerHeight,deviceScaleFactor:devicePixelRatio,locale:navigator.language,timeZone:Intl.DateTimeFormat().resolvedOptions().timeZone,dark:matchMedia("(prefers-color-scheme: dark)").matches,forced:matchMedia("(forced-colors: active)").matches,coarsePointer:matchMedia("(pointer: coarse)").matches,hover:matchMedia("(hover: hover)").matches})',
        returnByValue: true,
      });
      const environment = JSON.parse(envResult.result.value);
      if (environment.width !== inventory.viewport.width || environment.height !== inventory.viewport.height
        || environment.deviceScaleFactor !== deviceScaleFactor
        || environment.locale !== inventory.environment.locale
        || environment.timeZone !== inventory.environment.timeZone
        || environment.dark !== false || environment.forced !== false
        || environment.coarsePointer !== true || environment.hover !== false) {
        throw new Error(`Chromium 환경이 inventory와 다릅니다: ${JSON.stringify(environment)}`);
      }
      observations.push({
        viewport: { width: environment.width, height: environment.height,
          deviceScaleFactor: environment.deviceScaleFactor },
        environment: { locale: environment.locale, timeZone: environment.timeZone,
          dark: environment.dark, forcedColors: environment.forced,
          coarsePointer: environment.coarsePointer, hover: environment.hover },
        cases,
      });
    }
    if (JSON.stringify(observations[0].cases) !== JSON.stringify(observations[1].cases)) {
      throw new Error('DPR 변경이 CSS computed value, geometry 또는 owner 결과를 바꿨습니다.');
    }
    return { observations, browserVersion };
  },
});

const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const reference = {
  schema: 'spinon-css-c10-3-5-positioned-flex-reference/v1',
  referenceId: `chromium-${platform()}-${arch()}-${(await sha256File(join(root, inventoryPath))).slice(0, 12)}-${(await sha256File(join(root, capturePath))).slice(0, 12)}`,
  fixture: {
    id: inventory.fixtureId,
    inventoryPath,
    inventorySha256: hash(inventoryBytes),
    htmlPath,
    htmlSha256: hash(htmlBytes),
  },
  captureTool: {
    path: capturePath,
    sha256: await sha256File(join(root, capturePath)),
    helper: helperPath,
    helperSha256: await sha256File(join(root, helperPath)),
  },
  wpt: inventory.wpt,
  oracle: {
    name: 'Google Chrome',
    product: captureResult.browserVersion.product,
    cliVersion: versionResult.stdout.trim(),
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
    writingMode: inventory.environment.writingMode,
    direction: inventory.environment.direction,
  },
  comparison: inventory.comparison,
  observations: captureResult.observations,
};
await writeFile(join(root, outputPath), `${JSON.stringify(reference, null, 2)}\n`,
  replaceReference ? undefined : { flag: 'wx' });
console.log(`Wrote ${outputPath}`);
