import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';
import { release as osRelease } from 'node:os';

import {
  findChromiumExecutable,
  runChromiumPage,
  sha256File,
} from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c04/runtime-css-to-gpu-resize-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c04/runtime-css-to-gpu-resize.html';
const javascriptRelativePath = 'tests/fixtures/css/c04/runtime-css-to-gpu-resize.js';
const captureRelativePath = 'tools/css-reference/capture-runtime-css-to-gpu-resize.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c04-runtime-css-to-gpu-resize-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const javascriptPath = join(repositoryRoot, javascriptRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);

const [inventoryBytes, htmlBytes, javascriptBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(javascriptPath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const javascriptStyles = [...javascriptBytes.toString('utf8').matchAll(/setAttribute\('style','([^']+)'\)/g)]
  .map(([, style]) => style);
const htmlStyles = [...htmlBytes.toString('utf8').matchAll(/\sstyle="([^"]+)"/g)]
  .map(([, style]) => style);
if (JSON.stringify(javascriptStyles) !== JSON.stringify(htmlStyles)) {
  throw new Error('resize fixture JavaScript와 Chromium HTML의 inline style 순서가 다릅니다.');
}
const expectedViewports = [
  { id: 'baseline', width: 301, height: 100, deviceScaleFactor: 1 },
  { id: 'expanded', width: 341, height: 128, deviceScaleFactor: 1 },
];
if (inventory.schema !== 'spinon-css-runtime-gpu-resize-inventory/v1'
  || inventory.fixtureId !== 'C04-runtime-css-to-gpu-resize-v1'
  || JSON.stringify(inventory.viewports) !== JSON.stringify(expectedViewports)
  || inventory.environment?.locale !== 'en-US'
  || inventory.environment?.timeZone !== 'UTC'
  || inventory.environment?.colorScheme !== 'light'
  || inventory.environment?.pointer !== 'coarse'
  || inventory.environment?.hover !== 'none'
  || !Array.isArray(inventory.nodes) || inventory.nodes.length !== 4
  || inventory.nodes.map(({ id }) => id).join(',') !== 'root,opaque,transparent,hidden'
  || inventory.nodes.some(({ properties }) => !Array.isArray(properties)
    || properties.join(',') !== 'display,width,height,background-color')) {
  throw new Error('C04 runtime CSS resize inventory의 고정 계약이 올바르지 않습니다.');
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8' });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error(`Chromium 버전을 읽지 못했습니다: ${versionResult.error?.message ?? versionResult.stderr}`);
}
const version = versionResult.stdout.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
if (version !== '154.0.8037.98') {
  throw new Error(`Chromium 기준 버전이 고정값과 다릅니다: ${version ?? versionResult.stdout.trim()}`);
}

const inventoryScript = `Object.defineProperty(globalThis, '__SPINON_C04_RUNTIME_GPU_RESIZE_INVENTORY__', {
  configurable: false, enumerable: false, writable: false,
  value: Object.freeze(${JSON.stringify(inventory)}),
});`;
const { captureResult } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion, browserFlags }) => {
    if (browserVersion.product.match(/\d+\.\d+\.\d+\.\d+/)?.[0] !== '154.0.8037.98') {
      throw new Error(`Chromium CDP 기준 버전이 고정값과 다릅니다: ${browserVersion.product}`);
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Page.addScriptToEvaluateOnNewDocument', { source: inventoryScript });
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setUserAgentOverride', {
      userAgent: browserVersion.userAgent,
      acceptLanguage: 'en-US',
    });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    await page.send('Emulation.setTouchEmulationEnabled', {
      enabled: true,
      maxTouchPoints: 1,
    });
    await page.send('Emulation.setEmulatedMedia', {
      features: [
        { name: 'prefers-color-scheme', value: 'light' },
        { name: 'prefers-reduced-motion', value: 'no-preference' },
        { name: 'forced-colors', value: 'none' },
        { name: 'pointer', value: 'coarse' },
        { name: 'hover', value: 'none' },
      ],
    });

    const observations = {};
    for (const [index, viewport] of inventory.viewports.entries()) {
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: viewport.width,
        height: viewport.height,
        deviceScaleFactor: viewport.deviceScaleFactor,
        mobile: true,
        screenWidth: viewport.width,
        screenHeight: viewport.height,
      });
      if (index === 0) {
        const loaded = page.waitForEvent('Page.loadEventFired');
        await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
        await loaded;
      } else {
        const reloaded = page.waitForEvent('Page.loadEventFired');
        await page.send('Page.reload');
        await reloaded;
      }
      const evaluation = await page.send('Runtime.evaluate', {
        expression: 'document.querySelector("#reference-result")?.textContent ?? ""',
        returnByValue: true,
      });
      const encoded = evaluation.result?.value;
      if (evaluation.exceptionDetails || typeof encoded !== 'string' || !encoded) {
        throw new Error(`Chromium resize 관찰을 읽지 못했습니다: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
      }
      const observation = JSON.parse(Buffer.from(encoded, 'base64').toString('utf8'));
      const expectedViewport = {
        width: viewport.width,
        height: viewport.height,
        deviceScaleFactor: viewport.deviceScaleFactor,
      };
      const actualViewport = {
        width: observation.viewport?.width,
        height: observation.viewport?.height,
        deviceScaleFactor: observation.viewport?.deviceScaleFactor,
      };
      if (observation.fixtureId !== inventory.fixtureId
        || JSON.stringify(actualViewport) !== JSON.stringify(expectedViewport)
        || observation.environment?.locale !== 'en-US'
        || observation.environment?.intlLocale !== 'en-US'
        || observation.environment?.timeZone !== 'UTC'
        || observation.environment?.dark !== false
        || observation.environment?.coarsePointer !== true
        || observation.environment?.hover !== false
        || !Array.isArray(observation.nodes) || observation.nodes.length !== inventory.nodes.length) {
        throw new Error(`Chromium resize 환경 또는 노드 수가 다릅니다: ${JSON.stringify({ expectedViewport, actualViewport, observation })}`);
      }
      for (const [nodeIndex, expected] of inventory.nodes.entries()) {
        const observed = observation.nodes[nodeIndex];
        if (observed?.id !== expected.id
          || Object.keys(observed.properties ?? {}).join(',') !== expected.properties.join(',')
          || expected.properties.some((property) => typeof observed.properties[property] !== 'string'
            || observed.properties[property].length === 0)
          || !['x', 'y', 'width', 'height'].every((field) => Number.isFinite(observed.rect?.[field]))) {
          throw new Error(`Chromium resize node observation이 inventory와 다릅니다: ${expected.id}`);
        }
      }
      const root = observation.nodes[0];
      if (root.properties.width !== `${viewport.width}px`
        || root.properties.height !== `${viewport.height}px`
        || JSON.stringify(root.rect) !== JSON.stringify({
          x: 0, y: 0, width: viewport.width, height: viewport.height,
        })
        || observation.nodes[1].rect.width !== 51
        || observation.nodes[2].rect.x !== 62
        || observation.nodes[3].properties.display !== 'none') {
        throw new Error(`Chromium resize 레이아웃이 사전 판정과 다릅니다: ${JSON.stringify(observation.nodes)}`);
      }
      observations[viewport.id] = observation;
    }
    return {
      observations,
      browserFlags,
      oracle: {
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
const inventorySha256 = hashBytes(inventoryBytes);
const htmlSha256 = hashBytes(htmlBytes);
const javascriptSha256 = hashBytes(javascriptBytes);
let os = `${process.platform} ${osRelease()}`;
let osBuild = null;
if (process.platform === 'darwin') {
  const [product, build] = [
    spawnSync('/usr/bin/sw_vers', ['-productVersion'], { encoding: 'utf8' }),
    spawnSync('/usr/bin/sw_vers', ['-buildVersion'], { encoding: 'utf8' }),
  ];
  if (product.status === 0 && build.status === 0) {
    os = `macOS ${product.stdout.trim()}`;
    osBuild = build.stdout.trim();
  }
}
const reference = {
  schema: 'spinon-css-runtime-css-to-gpu-resize-reference/v1',
  referenceId: `chromium-${process.platform}-${process.arch}-${captureResult.oracle.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')}-${inventorySha256.slice(0, 12)}-${captureSha256.slice(0, 12)}-${executableSha256.slice(0, 12)}`,
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256,
    htmlPath: htmlRelativePath,
    htmlSha256,
    javascriptPath: javascriptRelativePath,
    javascriptSha256,
  },
  captureTool: {
    path: captureRelativePath,
    sha256: captureSha256,
    dependencies: [{ path: helperRelativePath, sha256: helperSha256 }],
  },
  oracle: {
    name: 'Chromium',
    ...captureResult.oracle,
    executable: chromiumPath,
    executableSha256,
  },
  environment: {
    os,
    osBuild,
    architecture: process.arch,
    nodeVersion: process.version,
    browserFlags: captureResult.browserFlags,
    viewports: inventory.viewports,
    locale: 'en-US',
    timeZone: 'UTC',
    media: inventory.environment,
  },
  comparison: inventory.comparison,
  observations: captureResult.observations,
};
try {
  await access(outputPath);
  throw new Error(`기준 reference는 덮어쓰지 않습니다: ${outputRelativePath}`);
} catch (error) {
  if (error?.code !== 'ENOENT') throw error;
}
await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`, { flag: 'wx' });
console.log(`저장: ${outputRelativePath}`);
console.log(`Chromium: ${captureResult.oracle.product} ${captureResult.oracle.revision}`);
for (const viewport of inventory.viewports) {
  const observation = captureResult.observations[viewport.id];
  const root = observation.nodes[0].rect;
  console.log(`${viewport.id}: ${viewport.width}×${viewport.height} CSS px, root ${root.width}×${root.height}`);
}
