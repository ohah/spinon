import { createHash } from 'node:crypto';
import { isAbsolute } from 'node:path';

const schema = 'spinon-cssom-property-surface/v1';
const propertyInventoryId = 'C01-cssom-property-surface-v1';
const fixturePath = 'tests/fixtures/css/c01/cssom-property-surface.html';
const captureToolPath = 'tools/css-reference/capture-property-surface.mjs';
const supportModulePaths = [
  'tools/css-reference/chromium-session.mjs',
  'tools/css-reference/property-surface.mjs',
].sort();
const expectedBrowserFlags = [
  '--headless=new',
  '--no-first-run',
  '--no-default-browser-check',
  '--disable-background-networking',
  '--lang=en-US',
  '--remote-debugging-port=0',
  '--user-data-dir=<temporary-profile>',
];
const htmlNamespace = 'http://www.w3.org/1999/xhtml';
const digestPattern = /^[a-f0-9]{64}$/;

function invalid(message) {
  throw new Error(`C01 CSSOM 속성 기준 자료가 올바르지 않습니다: ${message}`);
}

function classifyPropertyName(name) {
  if (typeof name !== 'string' || name.length === 0 || name.trim() !== name
    || /[\u0000-\u0020:;{}]/u.test(name)) {
    invalid(`CSSOM 속성 이름 형식이 잘못됐습니다: ${String(name)}`);
  }
  if (name.startsWith('--')) {
    if (name.length <= 2) invalid(`custom property 이름 형식이 잘못됐습니다: ${name}`);
    return 'custom';
  }
  if (name.startsWith('-')) {
    if (!/^-[a-z0-9-]+$/.test(name)) invalid(`prefixed 속성 이름 형식이 잘못됐습니다: ${name}`);
    return 'prefixed';
  }
  if (!/^[a-z][a-z0-9-]*$/.test(name)) invalid(`unprefixed 속성 이름 형식이 잘못됐습니다: ${name}`);
  return 'unprefixed';
}

export function cssomPropertyId(name) {
  if (typeof name !== 'string' || name.length === 0) {
    invalid(`CSSOM 속성 ID 입력이 비어 있습니다: ${String(name)}`);
  }
  return `cssom.property.${Buffer.from(name, 'utf8').toString('hex')}`;
}

export function buildPropertyEntries(rawNames) {
  if (!Array.isArray(rawNames) || rawNames.length === 0) {
    invalid('CSSOM 속성 이름 목록은 비어 있지 않은 배열이어야 합니다.');
  }

  const seen = new Set();
  const properties = rawNames.map((name) => {
    const category = classifyPropertyName(name);
    if (seen.has(name)) invalid(`CSSOM 속성 이름이 중복됐습니다: ${name}`);
    seen.add(name);
    return { id: cssomPropertyId(name), name, category };
  });
  return properties.sort((left, right) => (left.name < right.name ? -1 : left.name > right.name ? 1 : 0));
}

export function summarizePropertyEntries(properties) {
  const counts = { total: properties.length, unprefixed: 0, prefixed: 0, custom: 0 };
  for (const property of properties) counts[property.category] += 1;
  return counts;
}

function safeReferenceIdPart(value) {
  return value.toLowerCase()
    .replace(/[^a-z0-9.-]+/g, '-')
    .replace(/\.{2,}/g, '-')
    .replace(/^[.-]+|[.-]+$/g, '');
}

export function buildPropertySurfaceReferenceId(snapshot) {
  const platform = snapshot.environment?.os?.startsWith('macOS ')
    ? 'macos'
    : snapshot.environment?.os?.split(' ')[0]?.toLowerCase();
  const revision = safeReferenceIdPart(snapshot.oracle?.revision?.replace(/^@/, '').slice(0, 12) ?? '');
  const osId = safeReferenceIdPart(`${snapshot.environment?.os}-${snapshot.environment?.osBuild ?? 'no-build'}`);
  const toolDigests = [
    `${snapshot.captureTool?.path}:${snapshot.captureTool?.sha256}`,
    ...(snapshot.captureTool?.supportModules ?? []).map(({ path, sha256 }) => `${path}:${sha256}`),
  ].sort();
  const toolingSha256 = createHash('sha256').update(`${toolDigests.join('\n')}\n`).digest('hex');
  return [
    'cssom-property-surface-v1',
    safeReferenceIdPart(platform ?? ''),
    safeReferenceIdPart(snapshot.environment?.architecture ?? ''),
    osId,
    safeReferenceIdPart(snapshot.oracle?.version ?? ''),
    `node-${safeReferenceIdPart(snapshot.environment?.nodeVersion ?? '')}`,
    `revision-${revision}`,
    `binary-${snapshot.oracle?.executableSha256?.slice(0, 12)}`,
    `fixture-${snapshot.fixture?.sha256?.slice(0, 12)}`,
    `tools-${toolingSha256.slice(0, 12)}`,
  ].join('-');
}

function isNonEmptyString(value) {
  return typeof value === 'string' && value.trim() !== '';
}

function assertDigest(value, field) {
  if (!digestPattern.test(value ?? '')) invalid(`${field}는 SHA-256 64자리 16진수여야 합니다.`);
}

function assertExactKeys(value, expectedKeys, field) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    invalid(`${field}는 객체여야 합니다.`);
  }
  const actualKeys = Object.keys(value).sort();
  const expected = [...expectedKeys].sort();
  if (actualKeys.length !== expected.length || actualKeys.some((key, index) => key !== expected[index])) {
    invalid(`${field} 필드가 고정 schema와 다릅니다.`);
  }
}

export function validatePropertySurface(snapshot) {
  if (!snapshot || typeof snapshot !== 'object' || Array.isArray(snapshot)) {
    invalid('최상위 값은 객체여야 합니다.');
  }
  assertExactKeys(snapshot, [
    'schema', 'referenceId', 'capturedAtUtc', 'captureTool', 'oracle', 'environment', 'fixture', 'inventory',
  ], 'snapshot');
  if (snapshot.schema !== schema) invalid(`schema는 ${schema}여야 합니다.`);
  if (!isNonEmptyString(snapshot.referenceId)) invalid('referenceId가 비어 있습니다.');
  if (!isNonEmptyString(snapshot.capturedAtUtc) || Number.isNaN(Date.parse(snapshot.capturedAtUtc))
    || new Date(snapshot.capturedAtUtc).toISOString() !== snapshot.capturedAtUtc) {
    invalid('capturedAtUtc가 유효한 시각이 아닙니다.');
  }

  const captureTool = snapshot.captureTool;
  assertExactKeys(captureTool, ['path', 'sha256', 'supportModules'], 'captureTool');
  if (captureTool.path !== captureToolPath) invalid('captureTool.path가 고정 수집기와 다릅니다.');
  assertDigest(captureTool?.sha256, 'captureTool.sha256');
  if (!Array.isArray(captureTool?.supportModules) || captureTool.supportModules.length === 0) {
    invalid('captureTool.supportModules가 비어 있습니다.');
  }
  const actualSupportModulePaths = captureTool.supportModules.map((module) => module?.path).sort();
  if (JSON.stringify(actualSupportModulePaths) !== JSON.stringify(supportModulePaths)) {
    invalid('captureTool.supportModules가 고정 보조 모듈 목록과 다릅니다.');
  }
  for (const [index, module] of captureTool.supportModules.entries()) {
    assertExactKeys(module, ['path', 'sha256'], `supportModules[${index}]`);
    if (!isNonEmptyString(module?.path)) invalid(`supportModules[${index}].path가 비어 있습니다.`);
    assertDigest(module?.sha256, `supportModules[${index}].sha256`);
  }

  const oracle = snapshot.oracle;
  if (oracle?.name !== 'Chromium' || !/^\d+\.\d+\.\d+\.\d+$/.test(oracle?.version ?? '')
    || !isNonEmptyString(oracle?.product) || !/^@[a-f0-9]+$/.test(oracle?.revision ?? '')
    || !isNonEmptyString(oracle?.userAgent)
    || !isNonEmptyString(oracle?.executable) || !isAbsolute(oracle.executable)) {
    invalid('Chromium oracle의 product·version·revision·절대 경로가 올바르지 않습니다.');
  }
  if (oracle.product.match(/\d+\.\d+\.\d+\.\d+/)?.[0] !== oracle.version) {
    invalid('Chromium product와 version이 일치하지 않습니다.');
  }
  assertExactKeys(oracle, [
    'name', 'product', 'version', 'revision', 'userAgent', 'executable', 'executableSha256',
  ], 'oracle');
  assertDigest(oracle.executableSha256, 'oracle.executableSha256');

  const environment = snapshot.environment;
  if (!isNonEmptyString(environment?.os) || !isNonEmptyString(environment?.architecture)
    || !isNonEmptyString(environment?.nodeVersion) || !Array.isArray(environment?.flags)
    || environment.flags.some((flag) => !isNonEmptyString(flag))) {
    invalid('실행 환경의 OS·architecture·Node·flags가 불완전합니다.');
  }
  if (JSON.stringify(environment.flags) !== JSON.stringify(expectedBrowserFlags)) {
    invalid('Chromium 실행 flags가 고정 캡처 조건과 다릅니다.');
  }
  assertExactKeys(environment, [
    'os', 'osBuild', 'architecture', 'nodeVersion', 'flags', 'emulation', 'userAgent', 'observed',
  ], 'environment');
  const emulation = environment.emulation;
  if (emulation?.cssViewportPx?.width !== 800 || emulation.cssViewportPx?.height !== 600
    || emulation.deviceScaleFactor !== 1 || emulation.locale !== 'en-US' || emulation.acceptLanguage !== 'en-US'
    || emulation.timeZone !== 'UTC'
    || emulation.media?.prefersColorScheme !== 'light'
    || emulation.media?.prefersReducedMotion !== 'no-preference'
    || emulation.media?.forcedColors !== 'none') {
    invalid('CSS viewport·scale·locale·time zone·media emulation이 고정값과 다릅니다.');
  }
  assertExactKeys(emulation, ['cssViewportPx', 'deviceScaleFactor', 'locale', 'acceptLanguage', 'timeZone', 'media'], 'emulation');
  assertExactKeys(emulation.cssViewportPx, ['width', 'height'], 'emulation.cssViewportPx');
  assertExactKeys(emulation.media, ['prefersColorScheme', 'prefersReducedMotion', 'forcedColors'], 'emulation.media');
  assertExactKeys(environment.observed, ['viewport', 'locale', 'intlLocale', 'timeZone', 'media'], 'environment.observed');
  assertExactKeys(environment.observed.viewport, ['width', 'height', 'deviceScaleFactor'], 'environment.observed.viewport');
  assertExactKeys(environment.observed.media, [
    'prefersColorSchemeDark', 'prefersReducedMotion', 'forcedColors',
  ], 'environment.observed.media');
  if (environment.userAgent !== snapshot.oracle.userAgent
    || environment.observed?.viewport?.width !== 800 || environment.observed.viewport.height !== 600
    || environment.observed.viewport.deviceScaleFactor !== 1
    || environment.observed.locale !== 'en-US' || environment.observed.intlLocale !== 'en-US'
    || environment.observed.timeZone !== 'UTC'
    || environment.observed.media?.prefersColorSchemeDark !== false
    || environment.observed.media?.prefersReducedMotion !== false
    || environment.observed.media?.forcedColors !== false) {
    invalid('페이지에서 다시 관찰한 user agent·viewport·locale·time zone·media가 고정값과 다릅니다.');
  }

  const fixture = snapshot.fixture;
  if (fixture?.id !== propertyInventoryId || fixture?.path !== fixturePath) {
    invalid('fixture ID 또는 경로가 고정 계약과 다릅니다.');
  }
  assertExactKeys(fixture, [
    'id', 'path', 'sha256', 'element', 'authorStyleSheetCount', 'externalResourceCount',
  ], 'fixture');
  assertExactKeys(fixture.element, ['id', 'tag', 'namespace'], 'fixture.element');
  assertDigest(fixture.sha256, 'fixture.sha256');
  if (fixture.element?.id !== 'probe' || fixture.element.tag !== 'div'
    || fixture.element.namespace !== htmlNamespace || fixture.authorStyleSheetCount !== 0
    || fixture.externalResourceCount !== 0) {
    invalid('fixture는 외부 자원과 author stylesheet가 없는 HTML namespace div여야 합니다.');
  }

  const inventory = snapshot.inventory;
  if (inventory?.id !== propertyInventoryId || inventory.source !== 'getComputedStyle(element).item(index)'
    || inventory.completeness !== 'partial'
    || inventory.normalization !== 'CSSOM 원문 이름을 보존하고 JavaScript 기본 문자열 순서로 정렬') {
    invalid('inventory의 ID·관찰 출처·부분 범위·정규화 기준이 올바르지 않습니다.');
  }
  assertExactKeys(inventory, [
    'id', 'source', 'completeness', 'normalization', 'properties', 'counts', 'propertyNamesSha256', 'uncovered',
  ], 'inventory');
  if (!Array.isArray(inventory.properties) || inventory.properties.length === 0) {
    invalid('inventory.properties는 비어 있지 않은 배열이어야 합니다.');
  }
  const names = [];
  const ids = new Set();
  let previousName = null;
  for (const [index, property] of inventory.properties.entries()) {
    assertExactKeys(property, ['id', 'name', 'category'], `properties[${index}]`);
    const expectedCategory = classifyPropertyName(property?.name);
    if (property.id !== cssomPropertyId(property.name) || property.category !== expectedCategory) {
      invalid(`properties[${index}]의 ID 또는 category가 CSSOM 이름과 다릅니다.`);
    }
    if (ids.has(property.id)) invalid(`property ID가 중복됐습니다: ${property.id}`);
    ids.add(property.id);
    if (previousName !== null && !(previousName < property.name)) {
      invalid('properties가 기본 문자열 순서로 정렬되지 않았거나 이름이 중복됐습니다.');
    }
    previousName = property.name;
    names.push(property.name);
  }

  const expectedCounts = summarizePropertyEntries(inventory.properties);
  assertExactKeys(inventory.counts, ['total', 'unprefixed', 'prefixed', 'custom'], 'inventory.counts');
  for (const [field, expected] of Object.entries(expectedCounts)) {
    if (inventory.counts?.[field] !== expected) invalid(`inventory.counts.${field}가 목록과 다릅니다.`);
  }
  assertDigest(inventory.propertyNamesSha256, 'inventory.propertyNamesSha256');
  const expectedNamesHash = Buffer.from(`${names.join('\n')}\n`, 'utf8');
  const actualNamesHash = createHash('sha256').update(expectedNamesHash).digest('hex');
  if (inventory.propertyNamesSha256 !== actualNamesHash) {
    invalid('inventory.propertyNamesSha256가 정렬된 CSSOM 이름 목록과 다릅니다.');
  }
  if (!Array.isArray(inventory.uncovered) || inventory.uncovered.length === 0
    || inventory.uncovered.some((item) => !isNonEmptyString(item))) {
    invalid('미포함 범위를 명시하는 uncovered 항목이 필요합니다.');
  }
  if (snapshot.referenceId !== buildPropertySurfaceReferenceId(snapshot)) {
    invalid('referenceId가 브라우저·환경·입력 hash와 일치하지 않습니다.');
  }
  return snapshot;
}

export const propertySurfaceSchema = schema;
