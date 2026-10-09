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
const inputRelativePath = 'tests/fixtures/css/c04/cascade-layers.v1.json';
const htmlRelativePath = 'tests/fixtures/css/c04/cascade-layers.html';
const baseCssRelativePath = 'tests/fixtures/css/c04/style-layout-bridge.css';
const sessionHelperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c04-cascade-layers-v1.json';
const inputPath = join(repositoryRoot, inputRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const baseCssPath = join(repositoryRoot, baseCssRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const viewport = { width: 301, height: 40, deviceScaleFactor: 1 };

const [inputBytes, htmlBytes, baseCssBytes] = await Promise.all([
  readFile(inputPath),
  readFile(htmlPath),
  readFile(baseCssPath),
]);
const input = JSON.parse(inputBytes.toString('utf8'));
if (input.schema !== 'spinon-css-c04-cascade-layers-input/v1'
  || input.fixtureId !== 'C04-cascade-layers-v1'
  || input.viewport.widthCssPx !== viewport.width
  || input.viewport.heightCssPx !== viewport.height
  || input.viewport.deviceScaleFactor !== viewport.deviceScaleFactor
  || !Array.isArray(input.cases)
  || input.cases.length < 10) {
  throw new Error('C04 Cascade Layers fixture schema 또는 고정 입력이 올바르지 않습니다.');
}
const caseIds = input.cases.map((item) => item.id);
if (caseIds.some((id) => typeof id !== 'string' || id.length === 0)
  || new Set(caseIds).size !== caseIds.length
  || input.cases.some((item) => !Array.isArray(item.authorStylesheets)
    || item.authorStylesheets.length === 0
    || item.authorStylesheets.some((css) => typeof css !== 'string' || css.length === 0))) {
  throw new Error('C04 Cascade Layers case ID 또는 author stylesheet 목록이 잘못되었습니다.');
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8' });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error(`Chromium 버전을 읽지 못했습니다: ${versionResult.error?.message ?? versionResult.stderr}`);
}
const version = versionResult.stdout.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
if (!version) throw new Error(`Chromium 버전 형식을 확인할 수 없습니다: ${versionResult.stdout.trim()}`);

const fixtureInput = {
  ...input,
  baseCss: baseCssBytes.toString('utf8'),
};
const fixtureScript = `Object.defineProperty(globalThis, '__SPINON_C04_CASCADE_LAYERS__', {
  configurable: false, enumerable: false, writable: false,
  value: Object.freeze(${JSON.stringify(fixtureInput)}),
});`;
const { captureResult } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion, browserFlags }) => {
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Page.addScriptToEvaluateOnNewDocument', { source: fixtureScript });
    await page.send('Emulation.setDeviceMetricsOverride', {
      ...viewport,
      mobile: false,
      screenWidth: viewport.width,
      screenHeight: viewport.height,
    });
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    await page.send('Emulation.setEmulatedMedia', {
      features: [
        { name: 'prefers-color-scheme', value: 'light' },
        { name: 'prefers-reduced-motion', value: 'no-preference' },
        { name: 'forced-colors', value: 'none' },
      ],
    });

    const cases = [];
    for (const item of input.cases) {
      const loaded = page.waitForEvent('Page.loadEventFired');
      await page.send('Page.navigate', {
        url: `${pathToFileURL(htmlPath).href}?case=${encodeURIComponent(item.id)}`,
      });
      await loaded;
      const evaluation = await page.send('Runtime.evaluate', {
        expression: 'document.querySelector("#reference-result")?.textContent ?? ""',
        returnByValue: true,
      });
      const encoded = evaluation.result?.value;
      if (evaluation.exceptionDetails || typeof encoded !== 'string' || !encoded) {
        throw new Error(`Chromium case ${item.id} 결과를 읽지 못했습니다: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
      }
      const observation = JSON.parse(Buffer.from(encoded, 'base64').toString('utf8'));
      if (observation.caseId !== item.id
        || observation.frames.length !== 4
        || observation.itemStyles.length !== 3
        || observation.frames[0].id !== 'flex-parent'
        || observation.frames.slice(1).map((frame) => frame.id).join(',') !== 'flex-a,flex-b,flex-c'
        || observation.itemStyles.map((style) => style.id).join(',') !== 'flex-a,flex-b,flex-c'
        || Object.values(observation.computed).some((value) => typeof value !== 'string')) {
        throw new Error(`Chromium case ${item.id} 관찰 결과가 fixture 구조와 다릅니다.`);
      }
      cases.push(observation);
    }

    return {
      cases,
      browserFlags,
      oracle: {
        product: browserVersion.product,
        revision: browserVersion.revision,
        userAgent: browserVersion.userAgent,
      },
    };
  },
});

const executableSha256 = await sha256File(chromiumPath);
const htmlSha256 = createHash('sha256').update(htmlBytes).digest('hex');
const inputSha256 = createHash('sha256').update(inputBytes).digest('hex');
const baseCssSha256 = createHash('sha256').update(baseCssBytes).digest('hex');
const captureSha256 = await sha256File(fileURLToPath(import.meta.url));
const sessionHelperSha256 = await sha256File(join(repositoryRoot, sessionHelperRelativePath));
let os = `${process.platform} ${osRelease()}`;
let osBuild = null;
if (process.platform === 'darwin') {
  const productVersion = spawnSync('/usr/bin/sw_vers', ['-productVersion'], { encoding: 'utf8' });
  const buildVersion = spawnSync('/usr/bin/sw_vers', ['-buildVersion'], { encoding: 'utf8' });
  if (productVersion.status === 0 && buildVersion.status === 0) {
    os = `macOS ${productVersion.stdout.trim()}`;
    osBuild = buildVersion.stdout.trim();
  }
}
const reference = {
  schema: 'spinon-css-c04-cascade-layers-reference/v1',
  referenceId: `chromium-${process.platform}-${process.arch}-${captureResult.oracle.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')}-${inputSha256.slice(0, 12)}-${baseCssSha256.slice(0, 12)}-${captureSha256.slice(0, 12)}-${executableSha256.slice(0, 12)}`,
  fixture: {
    id: input.fixtureId,
    inputPath: inputRelativePath,
    inputSha256,
    htmlPath: htmlRelativePath,
    htmlSha256,
    baseCssPath: baseCssRelativePath,
    baseCssSha256,
  },
  captureTool: {
    path: 'tools/css-reference/capture-c04-cascade-layers.mjs',
    sha256: captureSha256,
    dependencies: [{ path: sessionHelperRelativePath, sha256: sessionHelperSha256 }],
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
    viewportCssPx: viewport,
    locale: 'en-US',
    timeZone: 'UTC',
    media: {
      prefersColorScheme: 'light',
      prefersReducedMotion: 'no-preference',
      forcedColors: 'none',
    },
  },
  comparison: input.comparison,
  observations: captureResult.cases,
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
console.log(`case ${captureResult.cases.length}개, frame ${captureResult.cases.reduce((count, item) => count + item.frames.length, 0)}개`);
