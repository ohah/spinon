import { readFile, writeFile, access } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';
import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = 'tests/fixtures/css/c12/position-absolute-block-inventory.json';
const htmlPath = 'tests/fixtures/css/c12/position-absolute-block.html';
const runtimePath = 'tests/fixtures/css/c12/runtime-position-absolute-block.js';
const capturePath = 'tools/css-reference/capture-c12-2-absolute-block.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const outputPath = 'tests/fixtures/css/references/c12-2-position-absolute-block-v1.json';
const replaceReference = process.argv.includes('--replace-reference');

const inventory = JSON.parse(await readFile(join(repositoryRoot, inventoryPath), 'utf8'));
if (inventory.schema !== 'spinon-css-c12-2-position-absolute-inventory/v1'
  || inventory.fixtureId !== 'C12.2-block-absolute-v1'
  || inventory.viewport?.width !== 360 || inventory.viewport?.height !== 800
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.cases?.length !== 23 || inventory.nodes?.length !== 82
  || new Set(inventory.nodes.map(({ id }) => id)).size !== inventory.nodes.length
  || new Set(inventory.cases.map(({ id }) => id)).size !== inventory.cases.length) {
  throw new Error('C12.2 inventory의 버전, case/node 수 또는 고유 ID가 고정 계약과 다릅니다.');
}
if (inventory.nodes.some(({ caseId }) => caseId !== 'root'
  && !inventory.cases.some(({ id }) => id === caseId))) {
  throw new Error('C12.2 node가 존재하지 않는 case를 참조합니다.');
}
const [htmlBytes, runtimeBytes] = await Promise.all([
  readFile(join(repositoryRoot, htmlPath)),
  readFile(join(repositoryRoot, runtimePath)),
]);
if (!replaceReference) {
  try {
    await access(join(repositoryRoot, outputPath));
    throw new Error(`기존 Chromium 기준은 덮어쓰지 않습니다: ${outputPath}`);
  } catch (error) {
    if (error?.code !== 'ENOENT') throw error;
  }
}

const chromiumPath = await findChromiumExecutable();
const version = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (version.error || version.status !== 0 || version.stdout.trim() !== 'Google Chrome 154.0.8037.98') {
  throw new Error(`고정 Chromium 실행 파일이 아닙니다: ${version.stdout?.trim() ?? version.stderr}`);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error(`Chromium DevTools 버전이 고정 기준과 다릅니다: ${JSON.stringify(browserVersion)}`);
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
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: inventory.viewport.width,
        height: inventory.viewport.height,
        deviceScaleFactor,
        mobile: false,
        screenWidth: inventory.viewport.width,
        screenHeight: inventory.viewport.height,
      });
      const loaded = page.waitForEvent('Page.loadEventFired');
      await page.send('Page.navigate', { url: pathToFileURL(join(repositoryRoot, htmlPath)).href });
      await loaded;
      const expression = `(() => {
        const specs = ${JSON.stringify(inventory.nodes)};
        const propertyNames = ${JSON.stringify(inventory.comparison.computedProperties)};
        const readRect = element => {
          const rect = element.getBoundingClientRect();
          return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
        };
        const ownerOf = element => {
          if (element.getClientRects().length === 0) return 'none';
          for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
            if (ancestor.hasAttribute('data-c12-node')
              && getComputedStyle(ancestor).position !== 'static'
              && ancestor.getClientRects().length > 0) return ancestor.id;
          }
          return 'viewport';
        };
        const nodes = specs.map(spec => {
          const element = document.getElementById(spec.id);
          if (!element) throw new Error('fixture node missing: ' + spec.id);
          const computed = getComputedStyle(element);
          const parent = element.parentElement?.closest('[data-c12-node]') ?? null;
          return {
            id: spec.id,
            caseId: element.dataset.c12Case,
            parentId: parent?.id ?? null,
            owner: ownerOf(element),
            hasLayoutBox: element.getClientRects().length > 0,
            rect: readRect(element),
            properties: Object.fromEntries(propertyNames.map(name => [name, computed.getPropertyValue(name).trim()])),
          };
        });
        const probes = [];
        for (const id of ['flow-absolute', 'static-absolute', 'static-diff-absolute']) {
          const element = document.getElementById(id);
          const original = element.getAttribute('style');
          for (const property of ['position', 'top', 'right', 'bottom', 'left']) {
            element.style.setProperty(property, property === 'position' ? 'static' : 'auto', 'important');
          }
          probes.push({ id, rect: readRect(element) });
          if (original === null) element.removeAttribute('style');
          else element.setAttribute('style', original);
        }
        return {
          environment: {
            width: innerWidth,
            height: innerHeight,
            deviceScaleFactor: devicePixelRatio,
            locale: navigator.language,
            timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
            dark: matchMedia('(prefers-color-scheme: dark)').matches,
            forced: matchMedia('(forced-colors: active)').matches,
            coarsePointer: matchMedia('(pointer: coarse)').matches,
            hover: matchMedia('(hover: hover)').matches,
          },
          nodes,
          flowProbes: probes,
        };
      })()`;
      const result = await page.send('Runtime.evaluate', { expression, returnByValue: true });
      if (result.exceptionDetails) {
        throw new Error(`Chromium fixture JavaScript 예외: ${JSON.stringify(result.exceptionDetails)}`);
      }
      const value = result.result?.value;
      const observation = typeof value === 'string' ? JSON.parse(value) : value;
      if (!observation || observation.environment.width !== inventory.viewport.width
        || observation.environment.height !== inventory.viewport.height
        || observation.environment.deviceScaleFactor !== deviceScaleFactor
        || observation.environment.locale !== inventory.environment.locale
        || observation.environment.timeZone !== inventory.environment.timeZone
        || observation.environment.dark !== false || observation.environment.forced !== false
        || observation.environment.coarsePointer !== true || observation.environment.hover !== false) {
        throw new Error(`고정 Chromium 환경 또는 관측값을 확인할 수 없습니다: ${JSON.stringify(observation?.environment)}`);
      }
      observations.push(observation);
    }
    return { observations };
  },
});

const reference = {
  schema: 'spinon-css-c12-2-position-absolute-reference/v1',
  referenceId: 'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-2',
  fixtureId: inventory.fixtureId,
  cssPosition: {
    url: 'https://www.w3.org/TR/2025/WD-css-position-3-20251007/',
    versionDate: '2025-10-07',
    status: 'W3C Working Draft',
  },
  chromium: {
    version: '154.0.8037.98',
    revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33',
    binarySha256: await sha256File(chromiumPath),
    executable: chromiumPath,
    host: process.platform + '-' + process.arch,
    flags: browserFlags,
  },
  wpt: {
    revision: '9ec154ff43db468923997c08bb08f905ceab62a5',
    execution: 'not-run',
    casePaths: [
      'css/css-position/position-absolute-padding-percentage.html',
      'css/css-position/position-absolute-percentage-height.html',
      'css/css-position/position-absolute-margin-auto-001.html',
      'css/css-position/position-absolute-dynamic-static-position.html',
      'css/css-position/position-absolute-dynamic-formatting-context.html',
    ],
  },
  viewport: inventory.viewport,
  environment: inventory.environment,
  comparison: inventory.comparison,
  fixture: {
    inventory: inventoryPath,
    inventorySha256: await sha256File(join(repositoryRoot, inventoryPath)),
    html: htmlPath,
    htmlSha256: await sha256File(join(repositoryRoot, htmlPath)),
    runtimeFixture: runtimePath,
    runtimeSourceSha256: await sha256File(join(repositoryRoot, runtimePath)),
  },
  captureTool: {
    path: capturePath,
    sha256: await sha256File(join(repositoryRoot, capturePath)),
    helper: helperPath,
    helperSha256: await sha256File(join(repositoryRoot, helperPath)),
  },
  observations: captureResult.observations,
};

await writeFile(join(repositoryRoot, outputPath), `${JSON.stringify(reference, null, 2)}\n`);
console.log(JSON.stringify({ outputPath, cases: inventory.cases.length, nodes: inventory.nodes.length, deviceScaleFactors: inventory.viewport.deviceScaleFactors }));
