import { access, readFile, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c12/position-fixed-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c12/position-fixed.html';
const captureRelativePath = 'tools/css-reference/capture-c12-3-fixed-positioning.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c12-3-position-fixed-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const replaceReference = process.argv.includes('--replace-reference');

const [inventoryBytes, htmlBytes] = await Promise.all([
  readFile(inventoryPath),
  readFile(htmlPath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const nodeIds = inventory.nodes?.map(({ id }) => id) ?? [];
const caseIds = inventory.cases?.map(({ id }) => id) ?? [];
if (inventory.schema !== 'spinon-css-c12-3-position-fixed-inventory/v1'
  || inventory.fixtureId !== 'C12.3-viewport-fixed-v1'
  || inventory.viewportMatrix?.length !== 2
  || JSON.stringify(inventory.viewportMatrix.map(({ deviceScaleFactors }) => deviceScaleFactors))
    !== '[[1,2,2.625,3],[1,2,2.625,3]]'
  || !Array.isArray(inventory.cases) || caseIds.length < 10
  || !Array.isArray(inventory.nodes) || nodeIds.length < 30
  || new Set(nodeIds).size !== nodeIds.length || new Set(caseIds).size !== caseIds.length
  || inventory.comparison?.maximumAbsoluteRectErrorCssPx !== 0.5
  || inventory.resizeSequence?.length !== 4) {
  throw new Error('C12.3 inventory의 고정 계약, case/node 수, DPR 또는 resize sequence가 올바르지 않습니다.');
}
const caseById = new Map(inventory.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));
const caseNodeIds = new Set();
for (const fixtureCase of inventory.cases) {
  if (!Array.isArray(fixtureCase.nodeIds) || fixtureCase.nodeIds.length === 0) {
    throw new Error(`C12.3 case에 node가 없습니다: ${fixtureCase.id}`);
  }
  for (const id of fixtureCase.nodeIds) {
    const node = inventory.nodes.find((candidate) => candidate.id === id);
    if (!node || node.caseId !== fixtureCase.id || caseNodeIds.has(id)) {
      throw new Error(`C12.3 case/node 연결이 유효하지 않습니다: ${fixtureCase.id}/${id}`);
    }
    caseNodeIds.add(id);
  }
}
for (const node of inventory.nodes) {
  if (node.id === 'c12-root') {
    if (node.parentId !== null || node.caseId !== 'root') {
      throw new Error('C12.3 root의 부모 또는 case 연결이 유효하지 않습니다.');
    }
    continue;
  }
  if (!caseById.has(node.caseId) || !nodeIds.includes(node.parentId)
    || !caseById.get(node.caseId).nodeIds.includes(node.id)) {
    throw new Error(`C12.3 node의 case·parent 연결이 유효하지 않습니다: ${node.id}`);
  }
}
if (caseNodeIds.size !== nodeIds.length - 1) {
  throw new Error('C12.3 root 외 모든 node가 정확히 하나의 case에 속해야 합니다.');
}
if (!replaceReference) {
  try {
    await access(outputPath);
    throw new Error(`기존 Chromium 기준은 덮어쓰지 않습니다: ${outputRelativePath}`);
  } catch (error) {
    if (error?.code !== 'ENOENT') throw error;
  }
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0
  || versionResult.stdout.trim() !== 'Google Chrome 154.0.8037.98') {
  throw new Error(`Chromium 기준 버전이 고정값과 다릅니다: ${versionResult.stdout?.trim() ?? versionResult.stderr}`);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error(`DevTools Chromium 기준이 고정값과 다릅니다: ${JSON.stringify(browserVersion)}`);
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

    const initialMetrics = inventory.resizeSequence[0];
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: initialMetrics.width,
      height: initialMetrics.height,
      deviceScaleFactor: initialMetrics.deviceScaleFactor,
      mobile: false,
      screenWidth: initialMetrics.width,
      screenHeight: initialMetrics.height,
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
    await loaded;

    const setupExpression = `(() => {
      const inventory = ${JSON.stringify(inventory)};
      const style = document.createElement('style');
      style.textContent = inventory.stylesheetRules.join('\\n');
      document.head.appendChild(style);
      const mount = document.getElementById('spinon-c12-mount');
      for (const spec of inventory.nodes) {
        const element = document.createElement('div');
        element.id = spec.id;
        element.setAttribute('data-c12-node', '');
        element.setAttribute('data-c12-case', spec.caseId);
        if (spec.style) element.setAttribute('style', spec.style);
        const parent = spec.parentId === null ? mount : document.getElementById(spec.parentId);
        if (!parent) throw new Error('fixture parent missing: ' + spec.parentId);
        parent.appendChild(element);
      }
      return true;
    })()`;
    const setupResult = await page.send('Runtime.evaluate', { expression: setupExpression, returnByValue: true });
    if (setupResult.exceptionDetails || setupResult.result?.value !== true) {
      throw new Error(`C12.3 Chromium fixture 초기화 실패: ${JSON.stringify(setupResult.exceptionDetails)}`);
    }

    const captureExpression = `(() => {
      const inventory = ${JSON.stringify(inventory)};
      const specs = inventory.nodes;
      const effectProperties = new Set([
        'transform', 'rotate', 'scale', 'translate', 'perspective', 'filter', 'backdrop-filter',
      ]);
      const willChangeProperties = new Set([...effectProperties, 'contain']);
      const containmentValues = new Set(['layout', 'paint', 'strict', 'content']);
      const createsFixedContainingBlock = (style) => {
        if ([...effectProperties].some((name) => style.getPropertyValue(name).trim() !== 'none')) return true;
        if (style.getPropertyValue('transform-style').trim() === 'preserve-3d') return true;
        if (style.getPropertyValue('content-visibility').trim() !== 'visible') return true;
        if (style.getPropertyValue('contain').split(/\\s+/).some((value) => containmentValues.has(value))) return true;
        return style.getPropertyValue('will-change').split(/,\\s*/).some((value) => willChangeProperties.has(value));
      };
      const ownerOf = (element, position, hasLayoutBox) => {
        if (!hasLayoutBox) return 'none';
        for (let parent = element.parentElement; parent; parent = parent.parentElement) {
          if (!parent.hasAttribute('data-c12-node')) continue;
          const style = getComputedStyle(parent);
          if (position === 'fixed' && createsFixedContainingBlock(style)) return parent.id;
          if (position === 'absolute' && style.position !== 'static') return parent.id;
        }
        return 'viewport';
      };
      const properties = inventory.comparison.computedProperties;
      const nodes = specs.map((spec) => {
        const element = document.getElementById(spec.id);
        if (!element) throw new Error('fixture node missing: ' + spec.id);
        const style = getComputedStyle(element);
        const rect = element.getBoundingClientRect();
        const hasLayoutBox = element.getClientRects().length > 0;
        const position = style.position;
        const actualParent = element.parentElement?.closest('[data-c12-node]')?.id ?? null;
        const owner = ownerOf(element, position, hasLayoutBox);
        let ownerProbe = null;
        if (hasLayoutBox && ['absolute', 'fixed'].includes(position)) {
          const ownerElement = owner === 'viewport' ? null : document.getElementById(owner);
          if (owner !== 'viewport' && !ownerElement) throw new Error('fixture owner missing: ' + owner);
          const ownerRect = ownerElement?.getBoundingClientRect() ?? { x: 0, y: 0 };
          const left = Number.parseFloat(style.left);
          const top = Number.parseFloat(style.top);
          const marginLeft = Number.parseFloat(style.marginLeft);
          const marginTop = Number.parseFloat(style.marginTop);
          ownerProbe = {
            ownerOrigin: { x: ownerRect.x, y: ownerRect.y },
            usedOffset: { left, top, marginLeft, marginTop },
            coordinateError: {
              x: rect.x - ownerRect.x - left - marginLeft,
              y: rect.y - ownerRect.y - top - marginTop,
            },
          };
        }
        return {
          id: spec.id,
          caseId: element.dataset.c12Case,
          parentId: actualParent,
          owner,
          expectedOwner: spec.expectedOwner,
          expectedPosition: spec.expectedPosition,
          hasLayoutBox,
          inputStyle: element.getAttribute('style') ?? '',
          rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
          ownerProbe,
          properties: Object.fromEntries(properties.map((name) => [name, style.getPropertyValue(name).trim()])),
        };
      });
      return {
        environment: {
          width: innerWidth,
          height: innerHeight,
          clientWidth: document.documentElement.clientWidth,
          clientHeight: document.documentElement.clientHeight,
          deviceScaleFactor: devicePixelRatio,
          locale: navigator.language,
          timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
          dark: matchMedia('(prefers-color-scheme: dark)').matches,
          forced: matchMedia('(forced-colors: active)').matches,
          coarsePointer: matchMedia('(pointer: coarse)').matches,
          hover: matchMedia('(hover: hover)').matches,
        },
        nodes,
      };
    })()`;

    const captureAt = async ({ id, width, height, deviceScaleFactor }) => {
      await page.send('Emulation.setDeviceMetricsOverride', {
        width, height, deviceScaleFactor, mobile: false, screenWidth: width, screenHeight: height,
      });
      const evaluation = await page.send('Runtime.evaluate', { expression: captureExpression, returnByValue: true });
      if (evaluation.exceptionDetails) {
        throw new Error(`C12.3 Chromium 관찰 예외(${id}): ${JSON.stringify(evaluation.exceptionDetails)}`);
      }
      const observation = evaluation.result?.value;
      const environment = observation?.environment;
      if (!environment || environment.width !== width || environment.height !== height
        || environment.clientWidth !== width || environment.clientHeight !== height
        || environment.deviceScaleFactor !== deviceScaleFactor
        || environment.locale !== inventory.environment.locale
        || environment.timeZone !== inventory.environment.timeZone
        || environment.dark !== false || environment.forced !== false
        || environment.coarsePointer !== true || environment.hover !== false) {
        throw new Error(`고정 Chromium 환경을 확인하지 못했습니다(${id}): ${JSON.stringify(environment)}`);
      }
      for (const [index, node] of observation.nodes.entries()) {
        const expected = inventory.nodes[index];
        if (node.id !== expected.id || node.caseId !== expected.caseId || node.parentId !== expected.parentId
          || node.expectedOwner !== expected.expectedOwner || node.expectedPosition !== expected.expectedPosition
          || node.owner !== expected.expectedOwner || node.properties.position !== expected.expectedPosition
          || node.hasLayoutBox !== (expected.hasLayoutBox ?? true)) {
          throw new Error(`Chrome owner/position/DOM/hidden 기준 불일치(${id}/${expected.id}): ${JSON.stringify(node)}`);
        }
        for (const field of inventory.comparison.rectFields) {
          if (!Number.isFinite(node.rect[field])) throw new Error(`유한하지 않은 Chrome frame(${id}/${node.id}/${field})`);
        }
        if (node.ownerProbe && Math.max(
          Math.abs(node.ownerProbe.coordinateError.x),
          Math.abs(node.ownerProbe.coordinateError.y),
        ) > inventory.comparison.maximumAbsoluteRectErrorCssPx) {
          throw new Error(`Chrome frame이 확인한 owner origin/inset과 맞지 않습니다(${id}/${node.id}): ${JSON.stringify(node.ownerProbe)}`);
        }
      }
      return { id, ...observation };
    };

    const observations = [];
    for (const viewport of inventory.viewportMatrix) {
      for (const deviceScaleFactor of viewport.deviceScaleFactors) {
        observations.push(await captureAt({
          id: `matrix-${viewport.width}x${viewport.height}-dpr-${deviceScaleFactor}`,
          width: viewport.width,
          height: viewport.height,
          deviceScaleFactor,
        }));
      }
    }
    const resizeSequence = [];
    for (const step of inventory.resizeSequence) resizeSequence.push(await captureAt(step));
    const canonical = (observation) => JSON.stringify(observation.nodes);
    if (canonical(resizeSequence[0]) !== canonical(resizeSequence[3])
      || canonical(resizeSequence[1]) !== canonical(resizeSequence[2])) {
      throw new Error('viewport resize 왕복 frame 또는 동일 환경 no-op frame이 결정적이지 않습니다.');
    }
    return { observations, resizeSequence };
  },
});

const reference = {
  schema: 'spinon-css-c12-3-position-fixed-reference/v1',
  referenceId: 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-3',
  fixtureId: inventory.fixtureId,
  cssPosition: inventory.cssPosition,
  chromium: {
    version: '154.0.8037.98',
    revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33',
    binarySha256: await sha256File(chromiumPath),
    executable: chromiumPath,
    host: `${process.platform}-${process.arch}`,
    flags: browserFlags,
  },
  wpt: inventory.wpt,
  viewportMatrix: inventory.viewportMatrix,
  resizeSequence: inventory.resizeSequence,
  environment: inventory.environment,
  comparison: inventory.comparison,
  inventorySummary: {
    cases: inventory.cases.length,
    nodes: inventory.nodes.length,
    negativeBoundaries: inventory.negativeBoundaries.map(({ id }) => id),
  },
  fixture: {
    inventory: inventoryRelativePath,
    inventorySha256: await sha256File(inventoryPath),
    html: htmlRelativePath,
    htmlSha256: await sha256File(htmlPath),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: await sha256File(join(repositoryRoot, captureRelativePath)),
    helper: helperRelativePath,
    helperSha256: await sha256File(join(repositoryRoot, helperRelativePath)),
  },
  observations: captureResult.observations,
  resizeObservations: captureResult.resizeSequence,
};

await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`);
console.log(JSON.stringify({
  outputPath: outputRelativePath,
  cases: inventory.cases.length,
  nodes: inventory.nodes.length,
  observations: captureResult.observations.length,
  resizeObservations: captureResult.resizeSequence.length,
}));
