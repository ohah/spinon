import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';
import { arch, platform, release } from 'node:os';
import { spawnSync } from 'node:child_process';

import {
  findChromiumExecutable,
  runChromiumPage,
  sha256File,
} from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inputRelativePath = 'tests/fixtures/css/c05/runtime-custom-properties.v1.json';
const htmlRelativePath = 'tests/fixtures/css/c05/runtime-custom-properties.html';
const appScriptRelativePath = 'tests/fixtures/css/c05/runtime-custom-properties-app.js';
const captureRelativePath = 'tools/css-reference/capture-c05-runtime-custom-properties.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c05-runtime-custom-properties-v1.json';
const inputPath = join(repositoryRoot, inputRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inputBytes, htmlBytes, appScriptBytes] = await Promise.all([
  readFile(inputPath), readFile(htmlPath), readFile(join(repositoryRoot, appScriptRelativePath)),
]);
const input = JSON.parse(inputBytes.toString('utf8'));

if (input.schema !== 'spinon-css-c05-runtime-custom-properties-input/v1'
  || input.fixtureId !== 'C05-runtime-inline-custom-properties-v1'
  || input.viewport?.widthCssPx !== 390
  || input.viewport?.heightCssPx !== 844
  || input.viewport?.deviceScaleFactor !== 1
  || !Array.isArray(input.nodes)
  || new Set(input.nodes).size !== input.nodes.length
  || JSON.stringify(input.transitions) !== JSON.stringify([
    'initial', 'ancestor-value-updated', 'moved-to-different-parent',
  ])) {
  throw new Error('C05 custom properties fixture schema 또는 고정 입력이 올바르지 않습니다.');
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error(`Chromium 버전을 읽지 못했습니다: ${versionResult.error?.message ?? versionResult.stderr}`);
}
const cliVersion = versionResult.stdout.trim();
if (!/^Google Chrome \d+\.\d+\.\d+\.\d+$/.test(cliVersion)) {
  throw new Error(`고정 oracle이 Google Chrome이 아닙니다: ${cliVersion}`);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    await page.send('Emulation.setEmulatedMedia', {
      features: [
        { name: 'prefers-color-scheme', value: 'light' },
        { name: 'forced-colors', value: 'none' },
      ],
    });
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: input.viewport.widthCssPx,
      height: input.viewport.heightCssPx,
      deviceScaleFactor: input.viewport.deviceScaleFactor,
      mobile: false,
      screenWidth: input.viewport.widthCssPx,
      screenHeight: input.viewport.heightCssPx,
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
    await loaded;
    const evaluation = await page.send('Runtime.evaluate', {
      expression: `(() => {
        const read = (ids) => Object.fromEntries(ids.map((id) => {
          const element = document.getElementById(id);
          const style = getComputedStyle(element);
          const rect = element.getBoundingClientRect();
          return [id, {
            width: style.width,
            marginLeft: style.marginLeft,
            rowGap: style.rowGap,
            columnGap: style.columnGap,
            backgroundColor: style.backgroundColor,
            rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
          }];
        }));
        const result = { initial: read(${JSON.stringify(input.nodes)}) };
        const source = document.getElementById('source-parent');
        const target = document.getElementById('moving-target');
        source.style.setProperty('--size', '73px');
        result.ancestorValueUpdated = read(['moving-target']);
        document.getElementById('destination-parent').append(target);
        result.movedToDifferentParent = read(['moving-target']);
        return JSON.stringify(result);
      })()`,
      returnByValue: true,
    });
    const encoded = evaluation.result?.value;
    if (evaluation.exceptionDetails || typeof encoded !== 'string' || encoded.length === 0) {
      throw new Error(`Chromium C05 결과를 읽지 못했습니다: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
    }
    return {
      observations: JSON.parse(encoded),
      browserVersion,
    };
  },
});

const sha256 = (value) => createHash('sha256').update(value).digest('hex');
const observations = captureResult.observations;
const reference = {
  schema: 'spinon-css-c05-runtime-custom-properties-reference/v1',
  fixtureId: input.fixtureId,
  fixture: {
    inputPath: inputRelativePath,
    inputSha256: sha256(inputBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: sha256(htmlBytes),
    appJavascriptPath: appScriptRelativePath,
    appJavascriptSha256: sha256(appScriptBytes),
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
    viewportCssPx: { width: input.viewport.widthCssPx, height: input.viewport.heightCssPx },
    deviceScaleFactor: input.viewport.deviceScaleFactor,
    locale: 'en-US',
    timeZone: 'UTC',
    colorScheme: 'light',
    forcedColors: 'none',
  },
  comparison: input.comparison,
  observations,
};

await writeFile(outputPath, `${JSON.stringify(reference, null, 2)}\n`, { flag: 'wx' });
process.stdout.write(`Chromium ${cliVersion}\nC05 reference: ${outputRelativePath}\n`);
