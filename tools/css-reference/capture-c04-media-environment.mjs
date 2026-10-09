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
const inputRelativePath = 'tests/fixtures/css/c04/media-environment.v1.json';
const htmlRelativePath = 'tests/fixtures/css/c04/media-environment.html';
const cssRelativePath = 'tests/fixtures/css/c04/media-environment.css';
const sessionHelperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c04-media-environment-v1.json';
const inputPath = join(repositoryRoot, inputRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const cssPath = join(repositoryRoot, cssRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const [inputBytes, htmlBytes, cssBytes] = await Promise.all([
  readFile(inputPath),
  readFile(htmlPath),
  readFile(cssPath),
]);
const input = JSON.parse(inputBytes.toString('utf8'));
const viewport = {
  width: input.viewport?.widthCssPx,
  height: input.viewport?.heightCssPx,
  deviceScaleFactor: input.viewport?.deviceScaleFactor,
};
const requiredProbes = [
  'scheme-light', 'scheme-dark',
  'pointer-none', 'pointer-coarse', 'pointer-fine',
  'hover-none', 'hover-hover',
  'any-pointer-none', 'any-pointer-coarse', 'any-pointer-fine',
  'any-hover-none', 'any-hover-hover',
];
if (input.schema !== 'spinon-css-c04-media-environment-input/v1'
  || input.fixtureId !== 'C04-media-environment-v1'
  || viewport.width !== 390
  || viewport.height !== 844
  || viewport.deviceScaleFactor !== 3
  || !Array.isArray(input.cases)
  || input.cases.length !== 4
  || JSON.stringify(input.probes) !== JSON.stringify(requiredProbes)) {
  throw new Error('C04 media environment fixture schema 또는 고정 입력이 올바르지 않습니다.');
}
const caseIds = input.cases.map(({ id }) => id);
if (caseIds.some((id) => typeof id !== 'string' || id.length === 0)
  || new Set(caseIds).size !== caseIds.length
  || input.cases.some((item) => !['desktop', 'mobile'].includes(item.mode)
    || !['light', 'dark'].includes(item.colorScheme)
    || !['none', 'coarse', 'fine'].includes(item.primaryPointer)
    || typeof item.primaryHover !== 'boolean'
    || ['coarse', 'fine', 'hover'].some((key) => typeof item.allPointers?.[key] !== 'boolean'))) {
  throw new Error('C04 media environment case의 ID·환경 입력이 잘못되었습니다.');
}

function expectedMedia(item) {
  const primary = item.primaryPointer;
  const all = item.allPointers;
  return {
    prefersColorSchemeLight: item.colorScheme === 'light',
    prefersColorSchemeDark: item.colorScheme === 'dark',
    pointerNone: primary === 'none',
    pointerCoarse: primary === 'coarse',
    pointerFine: primary === 'fine',
    hoverNone: !item.primaryHover,
    hoverHover: item.primaryHover,
    anyPointerNone: !all.coarse && !all.fine,
    anyPointerCoarse: all.coarse,
    anyPointerFine: all.fine,
    anyHoverNone: !all.hover,
    anyHoverHover: all.hover,
  };
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8' });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error(`Chromium 버전을 읽지 못했습니다: ${versionResult.error?.message ?? versionResult.stderr}`);
}
const version = versionResult.stdout.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
if (!version) throw new Error(`Chromium 버전 형식을 확인할 수 없습니다: ${versionResult.stdout.trim()}`);

const { captureResult } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion, browserFlags }) => {
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    await page.send('Emulation.setDeviceMetricsOverride', {
      ...viewport,
      mobile: false,
      screenWidth: viewport.width,
      screenHeight: viewport.height,
    });

    const observations = [];
    for (const item of input.cases) {
      await page.send('Emulation.setTouchEmulationEnabled', {
        enabled: item.mode === 'mobile',
        maxTouchPoints: 1,
      });
      await page.send('Emulation.setDeviceMetricsOverride', {
        ...viewport,
        mobile: item.mode === 'mobile',
        screenWidth: viewport.width,
        screenHeight: viewport.height,
      });
      await page.send('Emulation.setEmulatedMedia', {
        features: [
          { name: 'prefers-color-scheme', value: item.colorScheme },
          { name: 'prefers-reduced-motion', value: 'no-preference' },
          { name: 'forced-colors', value: 'none' },
        ],
      });
      const loaded = page.waitForEvent('Page.loadEventFired');
      await page.send('Page.navigate', {
        url: `${pathToFileURL(htmlPath).href}?case=${encodeURIComponent(item.id)}`,
      });
      await loaded;
      const evaluation = await page.send('Runtime.evaluate', {
        expression: `(() => {
          const queries = {
            prefersColorSchemeLight: '(prefers-color-scheme: light)',
            prefersColorSchemeDark: '(prefers-color-scheme: dark)',
            pointerNone: '(pointer: none)',
            pointerCoarse: '(pointer: coarse)',
            pointerFine: '(pointer: fine)',
            hoverNone: '(hover: none)',
            hoverHover: '(hover: hover)',
            anyPointerNone: '(any-pointer: none)',
            anyPointerCoarse: '(any-pointer: coarse)',
            anyPointerFine: '(any-pointer: fine)',
            anyHoverNone: '(any-hover: none)',
            anyHoverHover: '(any-hover: hover)',
          };
          const mediaMatches = Object.fromEntries(Object.entries(queries)
            .map(([name, query]) => [name, matchMedia(query).matches]));
          const displays = Object.fromEntries(${JSON.stringify(requiredProbes)}
            .map((id) => [id, getComputedStyle(document.getElementById(id)).display]));
          return JSON.stringify({ mediaMatches, displays });
        })()`,
        returnByValue: true,
      });
      const encoded = evaluation.result?.value;
      if (evaluation.exceptionDetails || typeof encoded !== 'string' || encoded.length === 0) {
        throw new Error(`Chromium case ${item.id} 관찰 결과를 읽지 못했습니다: ${evaluation.exceptionDetails?.text ?? '결과 없음'}`);
      }
      const observed = JSON.parse(encoded);
      const expected = expectedMedia(item);
      if (JSON.stringify(observed.mediaMatches) !== JSON.stringify(expected)) {
        throw new Error(`Chromium case ${item.id}의 matchMedia 결과가 입력 environment와 다릅니다: ${JSON.stringify(observed.mediaMatches)}`);
      }
      const expectedDisplays = Object.fromEntries(requiredProbes.map((probe) => {
        const mediaKey = {
          'scheme-light': 'prefersColorSchemeLight',
          'scheme-dark': 'prefersColorSchemeDark',
          'pointer-none': 'pointerNone',
          'pointer-coarse': 'pointerCoarse',
          'pointer-fine': 'pointerFine',
          'hover-none': 'hoverNone',
          'hover-hover': 'hoverHover',
          'any-pointer-none': 'anyPointerNone',
          'any-pointer-coarse': 'anyPointerCoarse',
          'any-pointer-fine': 'anyPointerFine',
          'any-hover-none': 'anyHoverNone',
          'any-hover-hover': 'anyHoverHover',
        }[probe];
        return [probe, expected[mediaKey] ? 'none' : 'block'];
      }));
      if (JSON.stringify(observed.displays) !== JSON.stringify(expectedDisplays)) {
        throw new Error(`Chromium case ${item.id}의 CSS media 적용 결과가 입력 environment와 다릅니다.`);
      }
      observations.push({ caseId: item.id, mediaMatches: observed.mediaMatches, displays: observed.displays });
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

const executableSha256 = await sha256File(chromiumPath);
const inputSha256 = createHash('sha256').update(inputBytes).digest('hex');
const htmlSha256 = createHash('sha256').update(htmlBytes).digest('hex');
const cssSha256 = createHash('sha256').update(cssBytes).digest('hex');
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
  schema: 'spinon-css-c04-media-environment-reference/v1',
  referenceId: `chromium-${captureResult.oracle.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')}-${inputSha256.slice(0, 12)}-${cssSha256.slice(0, 12)}-${captureSha256.slice(0, 12)}-${executableSha256.slice(0, 12)}`,
  fixture: {
    id: input.fixtureId,
    inputPath: inputRelativePath,
    inputSha256,
    htmlPath: htmlRelativePath,
    htmlSha256,
    cssPath: cssRelativePath,
    cssSha256,
  },
  captureTool: {
    path: 'tools/css-reference/capture-c04-media-environment.mjs',
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
  },
  comparison: input.comparison,
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
console.log(`case ${captureResult.observations.length}개, media probe ${requiredProbes.length}개`);
