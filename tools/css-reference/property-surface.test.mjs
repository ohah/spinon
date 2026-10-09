import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import {
  buildPropertyEntries,
  buildPropertySurfaceReferenceId,
  cssomPropertyId,
  propertySurfaceSchema,
  summarizePropertyEntries,
  validatePropertySurface,
} from './property-surface.mjs';

const chromiumReferenceId = 'cssom-property-surface-v1-macos-arm64-macos-26.5.1-25f80-154.0.8037.98-node-v24.20.0-revision-b859317bf11f-binary-ccffd5c5fe77-fixture-b4ea33587c76-tools-a6651dbb324d';
const chromiumReferencePath = new URL(
  `../../tests/fixtures/css/references/${chromiumReferenceId}/cssom-property-surface.json`,
  import.meta.url,
);
const repositoryRoot = new URL('../../', import.meta.url);

async function sha256Path(relativePath) {
  const bytes = await readFile(new URL(relativePath, repositoryRoot));
  return createHash('sha256').update(bytes).digest('hex');
}

function makeSnapshot() {
  const properties = buildPropertyEntries(['display', '-webkit-line-clamp', '--Theme', 'z-index']);
  const propertyNamesSha256 = createHash('sha256')
    .update(`${properties.map(({ name }) => name).join('\n')}\n`)
    .digest('hex');
  const snapshot = {
    schema: propertySurfaceSchema,
    referenceId: '',
    capturedAtUtc: '2026-10-09T03:00:00.000Z',
    captureTool: {
      path: 'tools/css-reference/capture-property-surface.mjs',
      sha256: 'a'.repeat(64),
      supportModules: [
        { path: 'tools/css-reference/chromium-session.mjs', sha256: 'b'.repeat(64) },
        { path: 'tools/css-reference/property-surface.mjs', sha256: 'c'.repeat(64) },
      ],
    },
    oracle: {
      name: 'Chromium',
      product: 'Chrome/154.0.8037.98',
      version: '154.0.8037.98',
      revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33',
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Chrome/154.0.8037.98',
      executable: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
      executableSha256: 'c'.repeat(64),
    },
    environment: {
      os: 'macOS 26.5.1',
      osBuild: '25F80',
      architecture: 'arm64',
      nodeVersion: 'v24.20.0',
      flags: [
        '--headless=new',
        '--no-first-run',
        '--no-default-browser-check',
        '--disable-background-networking',
        '--lang=en-US',
        '--remote-debugging-port=0',
        '--user-data-dir=<temporary-profile>',
      ],
      emulation: {
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
      },
      userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Chrome/154.0.8037.98',
      observed: {
        viewport: { width: 800, height: 600, deviceScaleFactor: 1 },
        locale: 'en-US',
        intlLocale: 'en-US',
        timeZone: 'UTC',
        media: {
          prefersColorSchemeDark: false,
          prefersReducedMotion: false,
          forcedColors: false,
        },
      },
    },
    fixture: {
      id: 'C01-cssom-property-surface-v1',
      path: 'tests/fixtures/css/c01/cssom-property-surface.html',
      sha256: 'd'.repeat(64),
      element: {
        id: 'probe',
        tag: 'div',
        namespace: 'http://www.w3.org/1999/xhtml',
      },
      authorStyleSheetCount: 0,
      externalResourceCount: 0,
    },
    inventory: {
      id: 'C01-cssom-property-surface-v1',
      source: 'getComputedStyle(element).item(index)',
      completeness: 'partial',
      normalization: 'CSSOM 원문 이름을 보존하고 JavaScript 기본 문자열 순서로 정렬',
      properties,
      counts: summarizePropertyEntries(properties),
      propertyNamesSha256,
      uncovered: ['CSS 값 조합', 'selector와 at-rule', 'SVG 요소별 적용'],
    },
  };
  snapshot.referenceId = buildPropertySurfaceReferenceId(snapshot);
  return snapshot;
}

test('CSSOM 속성 이름을 원문·종류별로 보존하고 결정적으로 정렬한다', () => {
  const properties = buildPropertyEntries(['z-index', '--Theme', 'display', '-webkit-line-clamp']);
  assert.deepEqual(properties.map(({ name }) => name), ['--Theme', '-webkit-line-clamp', 'display', 'z-index']);
  assert.deepEqual(properties.map(({ category }) => category), ['custom', 'prefixed', 'unprefixed', 'unprefixed']);
  assert.equal(properties[0].id, cssomPropertyId('--Theme'));
  assert.notEqual(cssomPropertyId('--Theme'), cssomPropertyId('--theme'));
  assert.deepEqual(summarizePropertyEntries(properties), {
    total: 4,
    unprefixed: 2,
    prefixed: 1,
    custom: 1,
  });
});

test('CSSOM 속성 목록이 비었거나 중복·잘못된 이름이면 거부한다', () => {
  assert.throws(() => buildPropertyEntries([]), /비어 있지 않은 배열/);
  assert.throws(() => buildPropertyEntries(['display', 'display']), /중복/);
  assert.throws(() => buildPropertyEntries([' display']), /이름 형식/);
  assert.throws(() => buildPropertyEntries(['bad:name']), /이름 형식/);
  assert.throws(() => buildPropertyEntries(['-bad/name']), /prefixed 속성 이름/);
  assert.throws(() => buildPropertyEntries(['--']), /custom/);
});

test('고정 환경·부분 범위·정렬된 이름 digest가 있는 snapshot을 받는다', () => {
  const snapshot = makeSnapshot();
  assert.equal(validatePropertySurface(snapshot), snapshot);
});

test('ID·분류·정렬·목록 수·digest가 달라진 snapshot을 거부한다', () => {
  const badId = makeSnapshot();
  badId.inventory.properties[0].id = 'cssom.property.wrong';
  assert.throws(() => validatePropertySurface(badId), /ID 또는 category/);

  const badCategory = makeSnapshot();
  badCategory.inventory.properties[0].category = 'unprefixed';
  assert.throws(() => validatePropertySurface(badCategory), /ID 또는 category/);

  const badOrder = makeSnapshot();
  badOrder.inventory.properties.reverse();
  assert.throws(() => validatePropertySurface(badOrder), /정렬/);

  const badCount = makeSnapshot();
  badCount.inventory.counts.total += 1;
  assert.throws(() => validatePropertySurface(badCount), /counts.total/);

  const badDigest = makeSnapshot();
  badDigest.inventory.propertyNamesSha256 = 'e'.repeat(64);
  assert.throws(() => validatePropertySurface(badDigest), /이름 목록과 다릅니다/);

  const badReferenceId = makeSnapshot();
  badReferenceId.referenceId += '-copied';
  assert.throws(() => validatePropertySurface(badReferenceId), /referenceId/);

  const unknownTopLevelField = makeSnapshot();
  unknownTopLevelField.cssSupport = 'complete';
  assert.throws(() => validatePropertySurface(unknownTopLevelField), /고정 schema/);

  const unknownInventoryField = makeSnapshot();
  unknownInventoryField.inventory.supported = true;
  assert.throws(() => validatePropertySurface(unknownInventoryField), /고정 schema/);

  const unknownPropertyField = makeSnapshot();
  unknownPropertyField.inventory.properties[0].supported = true;
  assert.throws(() => validatePropertySurface(unknownPropertyField), /고정 schema/);
});

test('외부 환경 문자열이 reference 디렉터리 경로 구분자나 상위 경로가 되지 않는다', () => {
  const snapshot = makeSnapshot();
  snapshot.environment.os = '../../tmp/../../bad';
  snapshot.environment.architecture = '../../outside';
  snapshot.oracle.version = '../../root';
  snapshot.oracle.revision = '@../../outside';
  const referenceId = buildPropertySurfaceReferenceId(snapshot);
  assert.doesNotMatch(referenceId, /[\\/]/);
  assert.doesNotMatch(referenceId, /\.\./);
});

test('부분 inventory를 전체 CSS 지원으로 승격하거나 비교 경계를 흐리면 거부한다', () => {
  const complete = makeSnapshot();
  complete.inventory.completeness = 'complete';
  assert.throws(() => validatePropertySurface(complete), /부분 범위/);

  const noUncovered = makeSnapshot();
  noUncovered.inventory.uncovered = [];
  assert.throws(() => validatePropertySurface(noUncovered), /uncovered/);

  const wrongSource = makeSnapshot();
  wrongSource.inventory.source = 'CSS.getSupportedCSSProperties';
  assert.throws(() => validatePropertySurface(wrongSource), /관찰 출처/);
});

test('외부 resource나 author stylesheet가 기준 fixture에 들어오면 거부한다', () => {
  const externalResource = makeSnapshot();
  externalResource.fixture.externalResourceCount = 1;
  assert.throws(() => validatePropertySurface(externalResource), /외부 자원/);

  const authorStyle = makeSnapshot();
  authorStyle.fixture.authorStyleSheetCount = 1;
  assert.throws(() => validatePropertySurface(authorStyle), /외부 자원/);
});

test('Chromium·환경·fixture provenance가 빠지거나 달라지면 거부한다', () => {
  const wrongRevision = makeSnapshot();
  wrongRevision.oracle.revision = 'unknown';
  assert.throws(() => validatePropertySurface(wrongRevision), /Chromium oracle/);

  const mismatchedProduct = makeSnapshot();
  mismatchedProduct.oracle.product = 'Chrome/153.0.8037.98';
  assert.throws(() => validatePropertySurface(mismatchedProduct), /product와 version/);

  const changedFlags = makeSnapshot();
  changedFlags.environment.flags.push('--disable-web-security');
  assert.throws(() => validatePropertySurface(changedFlags), /flags가 고정/);

  const changedToolPath = makeSnapshot();
  changedToolPath.captureTool.path = '../capture.mjs';
  assert.throws(() => validatePropertySurface(changedToolPath), /고정 수집기/);

  const changedFixturePath = makeSnapshot();
  changedFixturePath.fixture.path = '../../fixture.html';
  assert.throws(() => validatePropertySurface(changedFixturePath), /고정 계약/);

  const changedSupportModule = makeSnapshot();
  changedSupportModule.captureTool.supportModules[0].path = '../outside.mjs';
  assert.throws(() => validatePropertySurface(changedSupportModule), /고정 보조 모듈/);

  const malformedSupportModule = makeSnapshot();
  malformedSupportModule.captureTool.supportModules[0] = null;
  assert.throws(() => validatePropertySurface(malformedSupportModule), /고정 보조 모듈/);

  const wrongViewport = makeSnapshot();
  wrongViewport.environment.emulation.cssViewportPx.width = 390;
  assert.throws(() => validatePropertySurface(wrongViewport), /viewport/);

  const wrongNavigatorLocale = makeSnapshot();
  wrongNavigatorLocale.environment.observed.locale = 'ko-KR';
  assert.throws(() => validatePropertySurface(wrongNavigatorLocale), /다시 관찰한/);

  const wrongNamespace = makeSnapshot();
  wrongNamespace.fixture.element.namespace = 'http://www.w3.org/2000/svg';
  assert.throws(() => validatePropertySurface(wrongNamespace), /HTML namespace div/);
});

test('고정 Chromium reference가 현재 fixture·캡처 코드와 일치하고 부분 property surface로 남는다', async () => {
  const snapshot = JSON.parse(await readFile(chromiumReferencePath, 'utf8'));
  assert.equal(validatePropertySurface(snapshot), snapshot);
  assert.equal(snapshot.referenceId, chromiumReferenceId);
  assert.equal(snapshot.oracle.product, 'Chrome/154.0.8037.98');
  assert.equal(snapshot.oracle.version, '154.0.8037.98');
  assert.equal(snapshot.oracle.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
  assert.deepEqual(snapshot.inventory.counts, {
    total: 478,
    unprefixed: 442,
    prefixed: 36,
    custom: 0,
  });
  assert.equal(snapshot.inventory.completeness, 'partial');

  assert.equal(snapshot.fixture.sha256, await sha256Path(snapshot.fixture.path));
  assert.equal(snapshot.captureTool.sha256, await sha256Path(snapshot.captureTool.path));
  for (const supportModule of snapshot.captureTool.supportModules) {
    assert.equal(supportModule.sha256, await sha256Path(supportModule.path));
  }
});
