import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c12/position-static-relative-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c12/position-static-relative.html';
const runtimeRelativePath = 'tests/fixtures/css/c12/runtime-position-static-relative.js';
const captureRelativePath = 'tools/css-reference/capture-c12-1-static-relative.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c12-1-position-static-relative-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const runtimePath = join(repositoryRoot, runtimeRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const replaceReference = process.argv.includes('--replace-reference');

const [inventoryBytes, htmlBytes, runtimeBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(runtimePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const nodeIds = inventory.nodes?.map(({ id }) => id) ?? [];
const caseIds = inventory.cases?.map(({ id }) => id) ?? [];
if (inventory.schema !== 'spinon-css-c12-1-position-inventory/v1'
  || inventory.fixtureId !== 'C12.1-static-relative-v1'
  || inventory.viewport?.width !== 360 || inventory.viewport?.height !== 800
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US'
  || inventory.environment?.timeZone !== 'UTC'
  || !Array.isArray(inventory.cases) || inventory.cases.length !== 12
  || !Array.isArray(inventory.nodes) || nodeIds.length !== 47
  || new Set(nodeIds).size !== nodeIds.length
  || new Set(caseIds).size !== caseIds.length) {
  throw new Error('C12.1 inventory의 고정 계약, case 수 또는 node 수가 올바르지 않습니다.');
}
for (const fixtureCase of inventory.cases) {
  if (!fixtureCase.nodeIds.length || fixtureCase.nodeIds.some((id) => !nodeIds.includes(id))) {
    throw new Error(`C12.1 case의 NodeId가 inventory와 맞지 않습니다: ${fixtureCase.id}`);
  }
}
const caseById = new Map(inventory.cases.map((fixtureCase) => [fixtureCase.id, fixtureCase]));
const caseNodeIds = new Set();
for (const fixtureCase of inventory.cases) {
  for (const id of fixtureCase.nodeIds) {
    const node = inventory.nodes.find((candidate) => candidate.id === id);
    if (node?.caseId !== fixtureCase.id || caseNodeIds.has(id)) {
      throw new Error(`C12.1 case와 node의 양방향 연결이 올바르지 않습니다: ${fixtureCase.id}/${id}`);
    }
    caseNodeIds.add(id);
  }
}
for (const node of inventory.nodes) {
  if (node.id === 'c12-root') {
    if (node.parentId !== null || node.caseId !== 'root') {
      throw new Error('C12.1 root의 부모 또는 case 연결이 올바르지 않습니다.');
    }
    continue;
  }
  const fixtureCase = caseById.get(node.caseId);
  if (!fixtureCase || !fixtureCase.nodeIds.includes(node.id)
    || !nodeIds.includes(node.parentId)) {
    throw new Error(`C12.1 node의 case·parent 연결이 inventory와 맞지 않습니다: ${node.id}`);
  }
}
if (caseNodeIds.size !== nodeIds.length - 1) {
  throw new Error('C12.1 root를 제외한 모든 node가 정확히 하나의 case에 속해야 합니다.');
}

if (!replaceReference) {
  try {
    await access(outputPath);
    throw new Error(`기존 Chromium reference를 덮어쓰지 않습니다: ${outputRelativePath}`);
  } catch (error) {
    if (error?.code !== 'ENOENT') throw error;
  }
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error('Chromium 버전을 읽지 못했습니다: '
    + (versionResult.error?.message ?? versionResult.stderr));
}
if (versionResult.stdout.trim() !== 'Google Chrome 154.0.8037.98') {
  throw new Error(`Chromium 기준 버전이 고정값과 다릅니다: ${versionResult.stdout.trim()}`);
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

    const observations = [];
    for (const deviceScaleFactor of inventory.viewport.deviceScaleFactors) {
      const loaded = page.waitForEvent('Page.loadEventFired');
      await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
      await loaded;
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: inventory.viewport.width,
        height: inventory.viewport.height,
        deviceScaleFactor,
        mobile: false,
        screenWidth: inventory.viewport.width,
        screenHeight: inventory.viewport.height,
      });

      const environmentResult = await page.send('Runtime.evaluate', {
        expression: 'JSON.stringify({width:innerWidth,height:innerHeight,deviceScaleFactor:devicePixelRatio,'
          + 'locale:navigator.language,timeZone:Intl.DateTimeFormat().resolvedOptions().timeZone,'
          + 'dark:matchMedia("(prefers-color-scheme: dark)").matches,'
          + 'forced:matchMedia("(forced-colors: active)").matches,'
          + 'coarsePointer:matchMedia("(pointer: coarse)").matches,'
          + 'hover:matchMedia("(hover: hover)").matches})',
        returnByValue: true,
      });
      const environment = JSON.parse(environmentResult.result?.value ?? 'null');
      if (environment?.width !== inventory.viewport.width
        || environment?.height !== inventory.viewport.height
        || environment?.deviceScaleFactor !== deviceScaleFactor
        || environment?.locale !== inventory.environment.locale
        || environment?.timeZone !== inventory.environment.timeZone
        || environment?.dark !== false || environment?.forced !== false
        || environment?.coarsePointer !== true || environment?.hover !== false) {
        throw new Error(`Chromium 환경이 inventory와 다릅니다: ${JSON.stringify(environment)}`);
      }

      const readExpression = `(() => {
        const specs = ${JSON.stringify(inventory.nodes)};
        const expectedComputedProperties = ${JSON.stringify(inventory.comparison.computedProperties)};
        const state = ${JSON.stringify('initial')};
        const read = () => specs.map((spec) => {
          const element = document.getElementById(spec.id);
          if (!element) throw new Error('fixture node missing: ' + spec.id);
          const rect = element.getBoundingClientRect();
          const computed = getComputedStyle(element);
          const actualParentId = element.parentElement?.closest('[data-c12-node]')?.id ?? null;
          const children = Array.from(element.children)
            .filter((child) => child.hasAttribute('data-c12-node'))
            .map((child) => child.id);
          const properties = Object.fromEntries(expectedComputedProperties.map((name) => [
            name, computed.getPropertyValue(name).trim(),
          ]));
          const hasLayoutBox = element.getClientRects().length > 0;
          let owner = hasLayoutBox ? 'initial' : 'none';
          if (hasLayoutBox) {
            for (let parent = element.parentElement; parent; parent = parent.parentElement) {
              if (!parent.hasAttribute('data-c12-node') || parent.getClientRects().length === 0) continue;
              const parentPosition = getComputedStyle(parent).position;
              if (parentPosition !== 'static') {
                owner = parent.id;
                break;
              }
            }
          }
          return {
            id: spec.id,
            parentId: actualParentId,
            children,
            input: {
              tag: element.localName,
              style: element.getAttribute('style') ?? '',
              className: element.getAttribute('class') ?? '',
            },
            properties,
            hasLayoutBox,
            owner,
            rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
          };
        });
        const finalNodes = read();
        const originals = specs.map(({ id }) => {
          const element = document.getElementById(id);
          return { id, hadStyle: element.hasAttribute('style'), style: element.getAttribute('style') };
        });
        for (const { id } of specs) {
          const element = document.getElementById(id);
          element.style.setProperty('position', 'static', 'important');
          for (const side of ['top', 'right', 'bottom', 'left']) {
            element.style.setProperty(side, 'auto', 'important');
          }
        }
        void document.getElementById('c12-root').offsetWidth;
        const flowNodes = read();
        for (const original of originals) {
          const element = document.getElementById(original.id);
          if (original.hadStyle) element.setAttribute('style', original.style);
          else element.removeAttribute('style');
        }
        const flowById = new Map(flowNodes.map((node) => [node.id, node]));
        const nodes = finalNodes.map((node) => ({
          ...node,
          flowRect: flowById.get(node.id).rect,
          flowHasLayoutBox: flowById.get(node.id).hasLayoutBox,
        }));
        const rootStyle = getComputedStyle(document.getElementById('c12-root'));
        return JSON.stringify({
          state,
          nodes,
          rootStyle: { direction: rootStyle.direction, writingMode: rootStyle.writingMode },
        });
      })()`;

      const captureState = async (state) => {
        const expression = readExpression.replace('const state = "initial";', `const state = ${JSON.stringify(state)};`);
        const evaluation = await page.send('Runtime.evaluate', { expression, returnByValue: true });
        const value = evaluation.result?.value;
        if (evaluation.exceptionDetails || typeof value !== 'string' || value.length === 0) {
          throw new Error(`Chromium C12.1 관찰값을 읽지 못했습니다 (${state}): `
            + (evaluation.exceptionDetails?.text ?? '결과 없음'));
        }
        const observation = JSON.parse(value);
        if (observation.state !== state
          || JSON.stringify(observation.nodes.map(({ id }) => id)) !== JSON.stringify(nodeIds)
          || observation.rootStyle.direction !== inventory.environment.direction
          || observation.rootStyle.writingMode !== inventory.environment.writingMode) {
          throw new Error(`Chromium 관찰 트리 또는 writing mode가 inventory와 다릅니다 (${state}).`);
        }
        for (let index = 0; index < inventory.nodes.length; index += 1) {
          const spec = inventory.nodes[index];
          const node = observation.nodes[index];
          const expectedOwner = state === 'ancestor-relative'
            ? (spec.ancestorMutationOwner ?? spec.expectedOwner)
            : spec.expectedOwner;
          const expectedChildren = inventory.nodes
            .filter((candidate) => candidate.parentId === spec.id)
            .map(({ id }) => id);
          if (node.parentId !== spec.parentId
            || JSON.stringify(node.children) !== JSON.stringify(expectedChildren)
            || node.owner !== expectedOwner
            || typeof node.input?.tag !== 'string'
            || typeof node.input?.style !== 'string'
            || !inventory.comparison.computedProperties.every((property) => typeof node.properties[property] === 'string')
            || !inventory.comparison.rectFields.every((field) => Number.isFinite(node.rect[field])
              && Number.isFinite(node.flowRect[field]))) {
            throw new Error(`Chromium node 관찰값이 inventory와 다릅니다 (${state}/${spec.id}).`);
          }
          if (spec.hasLayoutBox === false && (node.hasLayoutBox || node.flowHasLayoutBox)) {
            throw new Error(`display:none node에 layout rect가 생겼습니다 (${state}/${spec.id}).`);
          }
        }
        return observation;
      };

      const states = [await captureState('initial')];
      for (const mutation of inventory.mutations) {
        const applyResult = await page.send('Runtime.evaluate', {
          expression: `(() => { const changes = ${JSON.stringify(mutation.changes)}; `
            + 'for (const [id, style] of Object.entries(changes)) '
            + 'document.getElementById(id).setAttribute("style", style); '
            + 'void document.getElementById("c12-root").offsetWidth; return true; })()',
          returnByValue: true,
        });
        if (applyResult.result?.value !== true) {
          throw new Error(`Chromium mutation 적용 실패: ${mutation.state}`);
        }
        states.push(await captureState(mutation.state));
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
          forcedColors: inventory.environment.forcedColors,
          coarsePointer: environment.coarsePointer,
          hover: environment.hover,
        },
        states,
      });
    }
    return { observations };
  },
});

const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');
const reference = {
  schema: 'spinon-css-c12-1-position-reference/v1',
  referenceId: 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-1',
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: digest(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: digest(htmlBytes),
    runtimeSourcePath: runtimeRelativePath,
    runtimeSourceSha256: digest(runtimeBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: await sha256File(join(repositoryRoot, captureRelativePath)),
    dependencies: [{ path: helperRelativePath, sha256: await sha256File(join(repositoryRoot, helperRelativePath)) }],
  },
  chromium: {
    version: '154.0.8037.98',
    revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33',
    binarySha256: await sha256File(chromiumPath),
    platform: process.platform,
    architecture: process.arch,
    runtime: process.version,
    capture: captureResult,
    browserFlags,
  },
  wpt: inventory.wpt,
  cssPosition: inventory.cssPosition,
  observations: captureResult.observations,
};

await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`);
console.log(`C12.1 Chromium 정적·상대 위치 기준을 저장했습니다: ${outputRelativePath}`);
