import { createHash } from 'node:crypto';
import { isAbsolute } from 'node:path';

export const selectorAtRuleSchema = 'spinon-css-selector-at-rule-surface/v1';
export const selectorAtRuleInventorySchema = 'spinon-c01-selector-at-rule-inventory/v1';

const inventoryId = 'C01-selector-at-rule-surface-v1';
const fixturePath = 'tests/fixtures/css/c01/selector-at-rule-surface.html';
const captureToolPath = 'tools/css-reference/capture-selector-at-rule-surface.mjs';
const supportModulePaths = [
  'tools/css-reference/chromium-session.mjs',
  'tools/css-reference/selector-at-rule-surface.mjs',
].sort();
const expectedBrowserVersion = '155.0.8059.85';
const expectedInventorySha256 = '970e9f0aab8134b603e6e4f4c790ad2c4b59709beb7eee927b276fa728ece7f8';
const expectedBrowserRevision = '@0e66b94549a8bd85f03fbdf189b752d6e7b63c32';
const expectedFlags = [
  '--headless=new',
  '--no-first-run',
  '--no-default-browser-check',
  '--disable-background-networking',
  '--lang=en-US',
  '--remote-debugging-port=0',
  '--user-data-dir=<temporary-profile>',
];
const stableIdPattern = /^(?:selector|at-rule)\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/;
const nodeIdPattern = /^[a-z][a-z0-9-]*$/;
const digestPattern = /^[a-f0-9]{64}$/;
const maximumRuleDepth = 16;
const maximumRuleCount = 256;

function invalid(message) {
  throw new Error(`C01.4 selector·at-rule 기준 자료가 올바르지 않습니다: ${message}`);
}

function assertExactKeys(value, expectedKeys, field) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) invalid(`${field}는 객체여야 합니다.`);
  const actualKeys = Object.keys(value).sort();
  const expected = [...expectedKeys].sort();
  if (actualKeys.length !== expected.length || actualKeys.some((key, index) => key !== expected[index])) {
    invalid(`${field} 필드가 고정 schema와 다릅니다.`);
  }
}

function assertDigest(value, field) {
  if (typeof value !== 'string' || !digestPattern.test(value)) invalid(`${field}는 SHA-256 64자리 16진수여야 합니다.`);
}

function assertString(value, field) {
  if (typeof value !== 'string' || value.trim() === '') invalid(`${field}는 비어 있지 않은 문자열이어야 합니다.`);
}

function sha256(value) {
  return createHash('sha256').update(value, 'utf8').digest('hex');
}

function digestJson(value) {
  return sha256(`${JSON.stringify(value, null, 2)}\n`);
}

function safeReferenceIdPart(value) {
  return String(value)
    .toLowerCase()
    .replace(/[^a-z0-9.-]+/g, '-')
    .replace(/\.{2,}/g, '-')
    .replace(/^[.-]+|[.-]+$/g, '');
}

export function validateSelectorAtRuleInventory(inventory) {
  assertExactKeys(inventory, [
    'schema', 'inventoryId', 'profileId', 'fixtureId', 'completeness', 'uncovered',
    'expectedNodeIds', 'selectors', 'atRules',
  ], 'inventory');
  if (inventory.schema !== selectorAtRuleInventorySchema || inventory.inventoryId !== inventoryId
    || inventory.fixtureId !== inventoryId || inventory.completeness !== 'partial') {
    invalid('inventory의 schema·identity·completeness가 고정 계약과 다릅니다.');
  }
  assertString(inventory.profileId, 'inventory.profileId');
  if (!Array.isArray(inventory.uncovered) || inventory.uncovered.length !== 6
    || inventory.uncovered.some((item) => typeof item !== 'string' || item.trim() === '')) {
    invalid('inventory.uncovered는 고정된 여섯 범위의 partial 설명이어야 합니다.');
  }
  if (!Array.isArray(inventory.expectedNodeIds) || inventory.expectedNodeIds.length !== 15) {
    invalid('inventory.expectedNodeIds는 fixture의 고정된 열다섯 node ID여야 합니다.');
  }
  const nodeIds = new Set();
  for (const nodeId of inventory.expectedNodeIds) {
    if (typeof nodeId !== 'string' || !nodeIdPattern.test(nodeId) || nodeIds.has(nodeId)) {
      invalid(`fixture node ID가 잘못됐거나 중복됐습니다: ${String(nodeId)}`);
    }
    nodeIds.add(nodeId);
  }

  if (!Array.isArray(inventory.selectors) || inventory.selectors.length !== 23) {
    invalid('inventory.selectors는 고정된 스물세 행이어야 합니다.');
  }
  const selectorIds = new Set();
  for (const [index, selector] of inventory.selectors.entries()) {
    assertString(selector?.id, `selectors[${index}].id`);
    if (!stableIdPattern.test(selector.id) || selectorIds.has(selector.id)) invalid(`selector ID가 잘못됐거나 중복됐습니다: ${selector.id}`);
    selectorIds.add(selector.id);
    assertString(selector.source, `selectors[${index}].source`);
    if (selector.kind === 'query') {
      assertExactKeys(selector, ['id', 'kind', 'source', 'expectedOutcome', 'expectedMatches'], `selectors[${index}]`);
      if (selector.expectedOutcome !== 'matched' || !Array.isArray(selector.expectedMatches)) {
        invalid(`${selector.id}의 expected query 결과가 없습니다.`);
      }
      let previousNodeIndex = -1;
      const matchIds = new Set();
      for (const nodeId of selector.expectedMatches) {
        const nodeIndex = inventory.expectedNodeIds.indexOf(nodeId);
        if (nodeIndex < 0 || matchIds.has(nodeId) || nodeIndex <= previousNodeIndex) {
          invalid(`${selector.id}의 expectedMatches에 미지·중복 node 또는 잘못된 document 순서가 있습니다.`);
        }
        matchIds.add(nodeId);
        previousNodeIndex = nodeIndex;
      }
    } else if (selector.kind === 'invalid-query') {
      assertExactKeys(selector, ['id', 'kind', 'source', 'expectedOutcome', 'expectedExceptionName'], `selectors[${index}]`);
      if (selector.expectedOutcome !== 'exception' || selector.expectedExceptionName !== 'SyntaxError') {
        invalid(`${selector.id}의 expected syntax error가 고정 계약과 다릅니다.`);
      }
    } else if (selector.kind === 'pseudo-element-rule') {
      assertExactKeys(selector, ['id', 'kind', 'source', 'expectedOutcome', 'expectedRuleConstructor', 'expectedSelectorText'], `selectors[${index}]`);
      if (selector.expectedOutcome !== 'rule-parse' || selector.expectedRuleConstructor !== 'CSSStyleRule'
        || selector.expectedSelectorText !== selector.source || !selector.source.includes('::')) {
        invalid(`${selector.id}의 pseudo-element parse 기준이 올바르지 않습니다.`);
      }
    } else {
      invalid(`${selector.id}의 selector kind가 올바르지 않습니다.`);
    }
  }

  if (!Array.isArray(inventory.atRules) || inventory.atRules.length !== 13) {
    invalid('inventory.atRules는 고정된 열세 행이어야 합니다.');
  }
  const atRuleIds = new Set();
  for (const [index, atRule] of inventory.atRules.entries()) {
    assertExactKeys(atRule, [
      'id', 'kind', 'source', 'expectedOutcome', 'expectedRuleCount',
      'expectedConstructor', 'expectedType', 'expectedChildrenCount',
    ], `atRules[${index}]`);
    assertString(atRule.id, `atRules[${index}].id`);
    if (!stableIdPattern.test(atRule.id) || atRuleIds.has(atRule.id)) invalid(`at-rule ID가 잘못됐거나 중복됐습니다: ${atRule.id}`);
    atRuleIds.add(atRule.id);
    assertString(atRule.source, `atRules[${index}].source`);
    if (!atRule.source.startsWith('@') || /@import\b|url\s*\(/i.test(atRule.source)) {
      invalid(`${atRule.id}는 외부 자원을 참조할 수 없습니다.`);
    }
    if (!['candidate', 'invalid-candidate'].includes(atRule.kind)) invalid(`${atRule.id}의 kind가 올바르지 않습니다.`);
    if (!['retained', 'dropped'].includes(atRule.expectedOutcome)
      || !Number.isInteger(atRule.expectedRuleCount) || atRule.expectedRuleCount < 0) {
      invalid(`${atRule.id}의 expected parse outcome/count가 올바르지 않습니다.`);
    }
    if (atRule.expectedOutcome === 'retained') {
      assertString(atRule.expectedConstructor, `${atRule.id}.expectedConstructor`);
      if (!Number.isInteger(atRule.expectedType) || atRule.expectedType < 0
        || (atRule.expectedChildrenCount !== null && (!Number.isInteger(atRule.expectedChildrenCount) || atRule.expectedChildrenCount < 0))
        || atRule.expectedRuleCount === 0) {
        invalid(`${atRule.id}의 expected CSSRule shape가 올바르지 않습니다.`);
      }
    } else if (atRule.expectedRuleCount !== 0 || atRule.expectedConstructor !== null
      || atRule.expectedType !== null || atRule.expectedChildrenCount !== null) {
      invalid(`${atRule.id}의 dropped rule 기준은 0개여야 합니다.`);
    }
  }
  if (digestJson(inventory) !== expectedInventorySha256) {
    invalid('C01.4 v1 input bytes가 고정 inventory SHA-256과 다릅니다.');
  }
  return inventory;
}

function validateRuleShape(rule, field, depth, state) {
  if (depth > maximumRuleDepth) invalid(`${field}의 중첩 깊이가 제한을 초과합니다.`);
  state.count += 1;
  if (state.count > maximumRuleCount) invalid(`${field}의 rule 수가 제한을 초과합니다.`);
  assertExactKeys(rule, [
    'constructor', 'type', 'cssText', 'selectorText', 'name', 'conditionText', 'children',
  ], field);
  assertString(rule.constructor, `${field}.constructor`);
  if (!Number.isInteger(rule.type) || rule.type < 0 || typeof rule.cssText !== 'string') {
    invalid(`${field}의 CSSRule type/cssText가 올바르지 않습니다.`);
  }
  for (const key of ['selectorText', 'name', 'conditionText']) {
    if (rule[key] !== null && typeof rule[key] !== 'string') invalid(`${field}.${key}는 문자열 또는 null이어야 합니다.`);
  }
  if (rule.children !== null && !Array.isArray(rule.children)) invalid(`${field}.children는 배열 또는 null이어야 합니다.`);
  rule.children?.forEach((child, index) => validateRuleShape(child, `${field}.children[${index}]`, depth + 1, state));
}

function assertObservationKeys(observation, keys, field) {
  assertExactKeys(observation, keys, field);
}

function validateObservedSnapshot(snapshot, inventory) {
  if (!snapshot || typeof snapshot !== 'object' || Array.isArray(snapshot)) invalid('최상위 값은 객체여야 합니다.');
  assertExactKeys(snapshot, [
    'schema', 'referenceId', 'capturedAtUtc', 'captureTool', 'oracle', 'environment', 'fixture', 'inventory',
  ], 'snapshot');
  if (snapshot.schema !== selectorAtRuleSchema) invalid(`schema는 ${selectorAtRuleSchema}여야 합니다.`);
  assertString(snapshot.referenceId, 'snapshot.referenceId');
  if (!Number.isFinite(Date.parse(snapshot.capturedAtUtc))
    || new Date(snapshot.capturedAtUtc).toISOString() !== snapshot.capturedAtUtc) {
    invalid('capturedAtUtc가 유효한 ISO timestamp가 아닙니다.');
  }

  assertExactKeys(snapshot.captureTool, ['path', 'sha256', 'supportModules'], 'captureTool');
  if (snapshot.captureTool.path !== captureToolPath) invalid('captureTool.path가 고정 수집기와 다릅니다.');
  assertDigest(snapshot.captureTool.sha256, 'captureTool.sha256');
  if (!Array.isArray(snapshot.captureTool.supportModules) || snapshot.captureTool.supportModules.length !== supportModulePaths.length) {
    invalid('captureTool.supportModules 목록이 고정 계약과 다릅니다.');
  }
  const modulePaths = snapshot.captureTool.supportModules.map((module) => module?.path).sort();
  if (JSON.stringify(modulePaths) !== JSON.stringify(supportModulePaths)) invalid('support module 경로가 고정 계약과 다릅니다.');
  for (const [index, module] of snapshot.captureTool.supportModules.entries()) {
    assertExactKeys(module, ['path', 'sha256'], `captureTool.supportModules[${index}]`);
    assertDigest(module.sha256, `captureTool.supportModules[${index}].sha256`);
  }

  assertExactKeys(snapshot.oracle, [
    'name', 'product', 'version', 'revision', 'userAgent', 'executable', 'executableSha256',
  ], 'oracle');
  if (snapshot.oracle.name !== 'Chromium' || snapshot.oracle.version !== expectedBrowserVersion
    || snapshot.oracle.revision !== expectedBrowserRevision || !snapshot.oracle.product.includes(expectedBrowserVersion)
    || !isAbsolute(snapshot.oracle.executable)) {
    invalid('Chromium product/version/revision/executable가 고정 oracle과 다릅니다.');
  }
  assertString(snapshot.oracle.userAgent, 'oracle.userAgent');
  assertDigest(snapshot.oracle.executableSha256, 'oracle.executableSha256');

  assertExactKeys(snapshot.environment, [
    'os', 'osBuild', 'architecture', 'nodeVersion', 'flags', 'emulation', 'userAgent', 'observed',
  ], 'environment');
  assertString(snapshot.environment.os, 'environment.os');
  if (snapshot.environment.osBuild !== null && typeof snapshot.environment.osBuild !== 'string') invalid('environment.osBuild가 올바르지 않습니다.');
  assertString(snapshot.environment.architecture, 'environment.architecture');
  assertString(snapshot.environment.nodeVersion, 'environment.nodeVersion');
  if (JSON.stringify(snapshot.environment.flags) !== JSON.stringify(expectedFlags)) invalid('Chromium flags가 고정 harness와 다릅니다.');
  assertExactKeys(snapshot.environment.emulation, ['cssViewportPx', 'deviceScaleFactor', 'locale', 'acceptLanguage', 'timeZone', 'media'], 'environment.emulation');
  assertExactKeys(snapshot.environment.emulation.cssViewportPx, ['width', 'height'], 'environment.emulation.cssViewportPx');
  assertExactKeys(snapshot.environment.emulation.media, ['prefersColorScheme', 'prefersReducedMotion', 'forcedColors'], 'environment.emulation.media');
  const emulation = snapshot.environment.emulation;
  if (emulation.cssViewportPx.width !== 800 || emulation.cssViewportPx.height !== 600
    || emulation.deviceScaleFactor !== 1 || emulation.locale !== 'en-US' || emulation.acceptLanguage !== 'en-US'
    || emulation.timeZone !== 'UTC' || emulation.media.prefersColorScheme !== 'light'
    || emulation.media.prefersReducedMotion !== 'no-preference' || emulation.media.forcedColors !== 'none') {
    invalid('emulation 조건이 고정 기준과 다릅니다.');
  }
  assertExactKeys(snapshot.environment.observed, [
    'viewport', 'locale', 'intlLocale', 'timeZone', 'media', 'userAgent',
  ], 'environment.observed');
  assertExactKeys(snapshot.environment.observed.viewport, ['width', 'height', 'deviceScaleFactor'], 'environment.observed.viewport');
  assertExactKeys(snapshot.environment.observed.media, ['prefersColorSchemeDark', 'prefersReducedMotion', 'forcedColors'], 'environment.observed.media');
  const observed = snapshot.environment.observed;
  if (observed.viewport.width !== 800 || observed.viewport.height !== 600 || observed.viewport.deviceScaleFactor !== 1
    || observed.locale !== 'en-US' || observed.intlLocale !== 'en-US' || observed.timeZone !== 'UTC'
    || observed.media.prefersColorSchemeDark !== false || observed.media.prefersReducedMotion !== false
    || observed.media.forcedColors !== false || observed.userAgent !== snapshot.oracle.userAgent) {
    invalid('실제 browser viewport/locale/time zone/media/userAgent/version 관찰이 고정 조건과 다릅니다.');
  }

  assertExactKeys(snapshot.fixture, [
    'id', 'path', 'sha256', 'nodeIds', 'elementCount', 'authorStyleSheetCount', 'externalResourceCount',
  ], 'fixture');
  if (snapshot.fixture.id !== inventory.fixtureId || snapshot.fixture.path !== fixturePath
    || JSON.stringify(snapshot.fixture.nodeIds) !== JSON.stringify(inventory.expectedNodeIds)
    || snapshot.fixture.elementCount !== inventory.expectedNodeIds.length
    || snapshot.fixture.authorStyleSheetCount !== 0 || snapshot.fixture.externalResourceCount !== 0) {
    invalid('fixture identity·node set·stylesheet/resource 경계가 고정 입력과 다릅니다.');
  }
  assertDigest(snapshot.fixture.sha256, 'fixture.sha256');

  assertExactKeys(snapshot.inventory, [
    'id', 'completeness', 'sha256', 'uncovered', 'selectorObservations', 'atRuleObservations', 'observationSha256',
  ], 'inventory result');
  if (snapshot.inventory.id !== inventory.inventoryId || snapshot.inventory.completeness !== 'partial'
    || JSON.stringify(snapshot.inventory.uncovered) !== JSON.stringify(inventory.uncovered)) {
    invalid('inventory result identity·partial scope가 고정 입력과 다릅니다.');
  }
  assertDigest(snapshot.inventory.sha256, 'inventory.sha256');
  if (snapshot.inventory.sha256 !== digestJson(inventory)) invalid('inventory source hash가 입력 JSON과 다릅니다.');

  const ruleState = { count: 0 };
  const selectorObservations = snapshot.inventory.selectorObservations;
  if (!Array.isArray(selectorObservations) || selectorObservations.length !== inventory.selectors.length) {
    invalid('selector observation count가 고정 입력과 다릅니다.');
  }
  for (const [index, expected] of inventory.selectors.entries()) {
    const actual = selectorObservations[index];
    if (!actual || actual.id !== expected.id || actual.kind !== expected.kind || actual.source !== expected.source) {
      invalid(`selector observation ${index}가 inventory 순서·입력과 다릅니다.`);
    }
    if (expected.kind === 'query') {
      assertObservationKeys(actual, ['id', 'kind', 'source', 'outcome', 'matches'], `selectorObservations[${index}]`);
      if (actual.outcome !== expected.expectedOutcome || JSON.stringify(actual.matches) !== JSON.stringify(expected.expectedMatches)) {
        invalid(`${expected.id}의 결과가 Chromium 사전 비교 기준과 다릅니다.`);
      }
      if (!Array.isArray(actual.matches)) invalid(`${expected.id}.matches가 배열이 아닙니다.`);
      const ranks = actual.matches.map((nodeId) => inventory.expectedNodeIds.indexOf(nodeId));
      if (ranks.some((rank) => rank < 0) || new Set(actual.matches).size !== actual.matches.length
        || ranks.some((rank, matchIndex) => matchIndex > 0 && rank <= ranks[matchIndex - 1])) {
        invalid(`${expected.id}의 match node ID가 미지·중복이거나 document order가 아닙니다.`);
      }
    } else if (expected.kind === 'invalid-query') {
      assertObservationKeys(actual, ['id', 'kind', 'source', 'outcome', 'exceptionName'], `selectorObservations[${index}]`);
      if (actual.outcome !== expected.expectedOutcome || actual.exceptionName !== expected.expectedExceptionName) {
        invalid(`${expected.id}의 syntax error 결과가 Chromium 사전 비교와 다릅니다.`);
      }
    } else {
      assertObservationKeys(actual, ['id', 'kind', 'source', 'outcome', 'rule'], `selectorObservations[${index}]`);
      if (actual.outcome !== expected.expectedOutcome) invalid(`${expected.id}의 pseudo-element parse outcome이 다릅니다.`);
      validateRuleShape(actual.rule, `selectorObservations[${index}].rule`, 0, ruleState);
      if (actual.rule.constructor !== expected.expectedRuleConstructor || actual.rule.selectorText !== expected.expectedSelectorText) {
        invalid(`${expected.id}의 pseudo-element CSSOM selector가 다릅니다.`);
      }
    }
  }

  const atRuleObservations = snapshot.inventory.atRuleObservations;
  if (!Array.isArray(atRuleObservations) || atRuleObservations.length !== inventory.atRules.length) {
    invalid('at-rule observation count가 고정 입력과 다릅니다.');
  }
  for (const [index, expected] of inventory.atRules.entries()) {
    const actual = atRuleObservations[index];
    if (!actual || actual.id !== expected.id || actual.kind !== expected.kind || actual.source !== expected.source) {
      invalid(`at-rule observation ${index}가 inventory 순서·입력과 다릅니다.`);
    }
    if (actual.outcome === 'exception') {
      assertObservationKeys(actual, ['id', 'kind', 'source', 'outcome', 'exceptionName', 'rules'], `atRuleObservations[${index}]`);
      assertString(actual.exceptionName, `${expected.id}.exceptionName`);
      if (!Array.isArray(actual.rules) || actual.rules.length !== 0 || actual.outcome !== expected.expectedOutcome) {
        invalid(`${expected.id}의 exception result가 사전 비교 기준과 다릅니다.`);
      }
    } else {
      assertObservationKeys(actual, ['id', 'kind', 'source', 'outcome', 'rules'], `atRuleObservations[${index}]`);
      if (!['retained', 'dropped'].includes(actual.outcome) || !Array.isArray(actual.rules)) invalid(`${expected.id} parse outcome이 올바르지 않습니다.`);
      const expectedOutcome = actual.rules.length === 0 ? 'dropped' : 'retained';
      if (actual.outcome !== expectedOutcome) invalid(`${expected.id} outcome과 CSSOM rule count가 모순됩니다.`);
      if (actual.outcome !== expected.expectedOutcome || actual.rules.length !== expected.expectedRuleCount) {
        invalid(`${expected.id}의 rule retention/count가 Chromium 사전 비교 기준과 다릅니다.`);
      }
      if (actual.outcome === 'retained') {
        const top = actual.rules[0];
        if (top.constructor !== expected.expectedConstructor || top.type !== expected.expectedType
          || (top.children === null ? null : top.children.length) !== expected.expectedChildrenCount) {
          invalid(`${expected.id}의 top-level CSSRule shape가 Chromium 사전 비교 기준과 다릅니다.`);
        }
      }
    }
    for (const [ruleIndex, rule] of (actual.rules ?? []).entries()) {
      validateRuleShape(rule, `atRuleObservations[${index}].rules[${ruleIndex}]`, 0, ruleState);
    }
  }

  const observationsForHash = {
    selectorObservations,
    atRuleObservations,
  };
  assertDigest(snapshot.inventory.observationSha256, 'inventory.observationSha256');
  if (snapshot.inventory.observationSha256 !== digestJson(observationsForHash)) {
    invalid('selector/at-rule observation hash가 결과 내용과 다릅니다.');
  }
  return snapshot;
}

export function buildSelectorAtRuleReferenceId(snapshot) {
  const platform = snapshot.environment?.os?.startsWith('macOS ')
    ? 'macos'
    : snapshot.environment?.os?.split(' ')[0]?.toLowerCase();
  const osId = safeReferenceIdPart(`${snapshot.environment?.os}-${snapshot.environment?.osBuild ?? 'no-build'}`);
  const toolDigests = [
    `${snapshot.captureTool?.path}:${snapshot.captureTool?.sha256}`,
    ...(snapshot.captureTool?.supportModules ?? []).map(({ path, sha256: digest }) => `${path}:${digest}`),
  ].sort();
  const toolsSha256 = sha256(`${toolDigests.join('\n')}\n`);
  return [
    'c01-selector-at-rule-surface-v1',
    safeReferenceIdPart(platform ?? ''),
    safeReferenceIdPart(snapshot.environment?.architecture ?? ''),
    osId,
    safeReferenceIdPart(snapshot.oracle?.version ?? ''),
    `node-${safeReferenceIdPart(snapshot.environment?.nodeVersion ?? '')}`,
    `revision-${safeReferenceIdPart(snapshot.oracle?.revision?.slice(1, 13) ?? '')}`,
    `binary-${snapshot.oracle?.executableSha256?.slice(0, 12) ?? ''}`,
    `fixture-${snapshot.fixture?.sha256?.slice(0, 12) ?? ''}`,
    `inventory-${snapshot.inventory?.sha256?.slice(0, 12) ?? ''}`,
    `tools-${toolsSha256.slice(0, 12)}`,
  ].join('-');
}

export function validateSelectorAtRuleSnapshot(snapshot, inventory) {
  validateSelectorAtRuleInventory(inventory);
  validateObservedSnapshot(snapshot, inventory);
  if (snapshot.referenceId !== buildSelectorAtRuleReferenceId(snapshot)) invalid('referenceId가 provenance와 다릅니다.');
  return snapshot;
}

export function equivalentSelectorAtRuleSnapshots(left, right) {
  const omitCaptureTime = ({ capturedAtUtc, ...snapshot }) => snapshot;
  return JSON.stringify(omitCaptureTime(left)) === JSON.stringify(omitCaptureTime(right));
}
