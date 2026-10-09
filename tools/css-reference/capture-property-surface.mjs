import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { mkdir, mkdtemp, readFile, rename, rmdir, rm, writeFile } from 'node:fs/promises';
import { release as osRelease, tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  findChromiumExecutable,
  runChromiumPage,
  sha256File,
} from './chromium-session.mjs';
import {
  buildPropertySurfaceReferenceId,
  buildPropertyEntries,
  summarizePropertyEntries,
  validatePropertySurface,
} from './property-surface.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixtureRelativePath = 'tests/fixtures/css/c01/cssom-property-surface.html';
const fixturePath = join(repositoryRoot, fixtureRelativePath);
const captureScriptRelativePath = 'tools/css-reference/capture-property-surface.mjs';
const propertyModuleRelativePath = 'tools/css-reference/property-surface.mjs';
const sessionModuleRelativePath = 'tools/css-reference/chromium-session.mjs';
const fixtureId = 'C01-cssom-property-surface-v1';
const htmlNamespace = 'http://www.w3.org/1999/xhtml';
const fixedEmulation = {
  cssViewportPx: { width: 800, height: 600 },
  deviceScaleFactor: 1,
  locale: 'en-US',
  acceptLanguage: 'en-US',
  timeZone: 'UTC',
  media: {
    prefersColorScheme: 'light',
    prefersReducedMotion: 'no-preference',
    forcedColors: 'none',
  },
};
const uncovered = [
  'CSS 속성별 허용 값·함수·초기값 조합',
  'selector, pseudo-class, pseudo-element, at-rule 및 cascade 결과',
  '지원 HTML·SVG 요소 전체와 요소별 속성 적용성',
  'UA stylesheet 내용과 브라우저 origin 동작',
  '레이아웃·텍스트 shaping·GPU pixel 출력',
  'Android·iOS runtime 및 플랫폼 간 호환성',
];

function propertyNamesSha256(properties) {
  const names = properties.map(({ name }) => name);
  return createHash('sha256').update(`${names.join('\n')}\n`, 'utf8').digest('hex');
}

function detectHostVersion() {
  if (process.platform !== 'darwin') {
    return { os: `${process.platform} ${osRelease()}`, osBuild: null };
  }
  const product = spawnSync('/usr/bin/sw_vers', ['-productVersion'], { encoding: 'utf8', timeout: 5_000 });
  const build = spawnSync('/usr/bin/sw_vers', ['-buildVersion'], { encoding: 'utf8', timeout: 5_000 });
  if (product.status !== 0 || build.status !== 0) {
    throw new Error('macOS 버전과 build를 읽지 못했습니다.');
  }
  return { os: `macOS ${product.stdout.trim()}`, osBuild: build.stdout.trim() };
}

function readChromiumCliVersion(chromiumPath) {
  const result = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
  if (result.error || result.status !== 0) {
    throw new Error(`Chromium CLI version을 읽지 못했습니다: ${result.error?.message ?? result.stderr}`);
  }
  const version = result.stdout.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
  if (!version) throw new Error(`Chromium CLI version 형식을 확인할 수 없습니다: ${result.stdout.trim()}`);
  return { version, product: result.stdout.trim() };
}

async function observeCssomPropertySurface(page, browserVersion) {
  await page.send('Page.enable');
  await page.send('Runtime.enable');
  await page.send('Emulation.setDeviceMetricsOverride', {
    width: fixedEmulation.cssViewportPx.width,
    height: fixedEmulation.cssViewportPx.height,
    deviceScaleFactor: fixedEmulation.deviceScaleFactor,
    mobile: false,
    screenWidth: fixedEmulation.cssViewportPx.width,
    screenHeight: fixedEmulation.cssViewportPx.height,
  });
  await page.send('Emulation.setLocaleOverride', { locale: fixedEmulation.locale });
  if (typeof browserVersion.userAgent !== 'string' || browserVersion.userAgent === '') {
    throw new Error('Browser.getVersion에서 user agent를 읽지 못했습니다.');
  }
  await page.send('Emulation.setUserAgentOverride', {
    userAgent: browserVersion.userAgent,
    acceptLanguage: fixedEmulation.acceptLanguage,
  });
  await page.send('Emulation.setTimezoneOverride', { timezoneId: fixedEmulation.timeZone });
  await page.send('Emulation.setEmulatedMedia', {
    features: [
      { name: 'prefers-color-scheme', value: fixedEmulation.media.prefersColorScheme },
      { name: 'prefers-reduced-motion', value: fixedEmulation.media.prefersReducedMotion },
      { name: 'forced-colors', value: fixedEmulation.media.forcedColors },
    ],
  });

  const loadEvent = page.waitForEvent('Page.loadEventFired');
  await page.send('Page.navigate', { url: pathToFileURL(fixturePath).href });
  await loadEvent;
  const result = await page.send('Runtime.evaluate', {
    expression: `(() => {
      const element = document.querySelector('#probe');
      if (!element) throw new Error('고정 CSSOM fixture의 #probe를 찾지 못했습니다.');
      const declaration = getComputedStyle(element);
      return {
        element: {
          id: element.id,
          tag: element.localName,
          namespace: element.namespaceURI,
        },
        propertyNames: Array.from({ length: declaration.length }, (_, index) => declaration.item(index)),
        authorStyleSheetCount: document.styleSheets.length,
        externalResourceCount: performance.getEntriesByType('resource').length,
        viewport: {
          width: window.innerWidth,
          height: window.innerHeight,
          deviceScaleFactor: window.devicePixelRatio,
        },
        locale: navigator.language,
        intlLocale: Intl.DateTimeFormat().resolvedOptions().locale,
        timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
        media: {
          prefersColorSchemeDark: matchMedia('(prefers-color-scheme: dark)').matches,
          prefersReducedMotion: matchMedia('(prefers-reduced-motion: reduce)').matches,
          forcedColors: matchMedia('(forced-colors: active)').matches,
        },
        userAgent: navigator.userAgent,
      };
    })()`,
    returnByValue: true,
  });
  if (result.exceptionDetails || !result.result?.value) {
    throw new Error(`CSSOM property surface를 관찰하지 못했습니다: ${result.exceptionDetails?.text ?? '결과 없음'}`);
  }
  const observation = result.result.value;
  if (observation.element?.id !== 'probe' || observation.element.tag !== 'div'
    || observation.element.namespace !== htmlNamespace) {
    throw new Error(`fixture 요소가 HTML namespace div가 아닙니다: ${JSON.stringify(observation.element)}`);
  }
  if (observation.authorStyleSheetCount !== 0 || observation.externalResourceCount !== 0) {
    throw new Error(`fixture에 stylesheet 또는 외부 resource가 있습니다: ${JSON.stringify({
      authorStyleSheetCount: observation.authorStyleSheetCount,
      externalResourceCount: observation.externalResourceCount,
    })}`);
  }
  if (observation.viewport.width !== 800 || observation.viewport.height !== 600
    || observation.viewport.deviceScaleFactor !== 1) {
    throw new Error(`CSS viewport/scale이 고정값과 다릅니다: ${JSON.stringify(observation.viewport)}`);
  }
  if (observation.locale !== 'en-US' || observation.intlLocale !== 'en-US' || observation.timeZone !== 'UTC') {
    throw new Error(`locale/time zone이 고정값과 다릅니다: ${JSON.stringify({
      locale: observation.locale,
      intlLocale: observation.intlLocale,
      timeZone: observation.timeZone,
    })}`);
  }
  if (observation.media.prefersColorSchemeDark !== false
    || observation.media.prefersReducedMotion !== false
    || observation.media.forcedColors !== false) {
    throw new Error(`미디어 상태가 고정값과 다릅니다: ${JSON.stringify(observation.media)}`);
  }
  return observation;
}

const chromiumPath = await findChromiumExecutable();
const cliVersion = readChromiumCliVersion(chromiumPath);
const hostVersion = detectHostVersion();
const fixtureBytes = await readFile(fixturePath);
const fixtureSha256 = createHash('sha256').update(fixtureBytes).digest('hex');
const executableSha256 = await sha256File(chromiumPath);
const captureSha256 = await sha256File(fileURLToPath(import.meta.url));
const supportModules = await Promise.all([
  propertyModuleRelativePath,
  sessionModuleRelativePath,
].map(async (path) => ({
  path,
  sha256: await sha256File(join(repositoryRoot, path)),
})));

const { captureResult } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion, browserFlags: activeFlags }) => {
    const version = browserVersion.product.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
    if (version !== cliVersion.version) {
      throw new Error(`Chromium CLI/CDP version이 다릅니다: ${cliVersion.version}/${browserVersion.product}`);
    }
    if (!/^@[a-f0-9]+$/.test(browserVersion.revision ?? '')) {
      throw new Error(`Chromium revision을 읽지 못했습니다: ${browserVersion.revision}`);
    }
    const observation = await observeCssomPropertySurface(page, browserVersion);
    const properties = buildPropertyEntries(observation.propertyNames);
    if (properties.length !== observation.propertyNames.length) {
      throw new Error('CSSOM property 개수가 정규화 전후에 달라졌습니다.');
    }
    const inventoryCounts = summarizePropertyEntries(properties);
    const inventory = {
      id: fixtureId,
      source: 'getComputedStyle(element).item(index)',
      completeness: 'partial',
      normalization: 'CSSOM 원문 이름을 보존하고 JavaScript 기본 문자열 순서로 정렬',
      properties,
      counts: inventoryCounts,
      propertyNamesSha256: propertyNamesSha256(properties),
      uncovered,
    };
    const snapshot = {
      schema: 'spinon-cssom-property-surface/v1',
      capturedAtUtc: new Date().toISOString(),
      captureTool: {
        path: captureScriptRelativePath,
        sha256: captureSha256,
        supportModules,
      },
      oracle: {
        name: 'Chromium',
        product: browserVersion.product,
        version,
        revision: browserVersion.revision,
        userAgent: browserVersion.userAgent,
        executable: chromiumPath,
        executableSha256,
      },
      environment: {
        os: hostVersion.os,
        osBuild: hostVersion.osBuild,
        architecture: process.arch,
        nodeVersion: process.version,
        flags: activeFlags,
        emulation: fixedEmulation,
        userAgent: observation.userAgent,
        observed: {
          viewport: observation.viewport,
          locale: observation.locale,
          intlLocale: observation.intlLocale,
          timeZone: observation.timeZone,
          media: observation.media,
        },
      },
      fixture: {
        id: fixtureId,
        path: fixtureRelativePath,
        sha256: fixtureSha256,
        element: observation.element,
        authorStyleSheetCount: observation.authorStyleSheetCount,
        externalResourceCount: observation.externalResourceCount,
      },
      inventory,
    };
    snapshot.referenceId = buildPropertySurfaceReferenceId(snapshot);
    validatePropertySurface(snapshot);
    return snapshot;
  },
});

const snapshot = captureResult;
const snapshotBytes = Buffer.from(`${JSON.stringify(snapshot, null, 2)}\n`, 'utf8');
const outputRoot = join(repositoryRoot, 'tests/fixtures/css/references');
const outputDirectory = join(outputRoot, snapshot.referenceId);
const pendingDirectory = await mkdtemp(join(outputRoot, '.cssom-property-surface-pending-'));
const pendingFile = join(pendingDirectory, 'cssom-property-surface.json');
const outputFile = join(outputDirectory, 'cssom-property-surface.json');
let outputDirectoryReserved = false;

try {
  await writeFile(pendingFile, snapshotBytes, { flag: 'wx' });
  validatePropertySurface(JSON.parse(await readFile(pendingFile, 'utf8')));
  await mkdir(outputDirectory);
  outputDirectoryReserved = true;
  await rename(pendingFile, outputFile);
} catch (error) {
  if (outputDirectoryReserved) {
    try {
      await rmdir(outputDirectory);
    } catch {
      // 다른 파일이 생긴 경우에는 이번 실행의 소유물이 아니므로 보존합니다.
    }
  }
  throw new Error(`C01 CSSOM reference를 원자적으로 저장하지 못했습니다: ${error.message}`, { cause: error });
} finally {
  await rm(pendingDirectory, { recursive: true, force: true });
}

console.log(`Chromium ${snapshot.oracle.version} (${snapshot.oracle.revision}) CSSOM 기준 저장: ${outputFile}`);
console.log(`속성 이름: 총 ${snapshot.inventory.counts.total}개 · unprefixed ${snapshot.inventory.counts.unprefixed}개 · prefixed ${snapshot.inventory.counts.prefixed}개 · custom ${snapshot.inventory.counts.custom}개`);
console.log(`입력 SHA-256: fixture ${fixtureSha256} · Chromium ${executableSha256}`);
console.log('범위: HTML div의 계산 스타일 속성 이름만 관찰했습니다. CSS 값이나 기능 지원 판정이 아닙니다.');
