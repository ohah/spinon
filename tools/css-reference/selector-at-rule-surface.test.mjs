import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import {
  buildSelectorAtRuleReferenceId,
  equivalentSelectorAtRuleSnapshots,
  validateSelectorAtRuleInventory,
  validateSelectorAtRuleSnapshot,
} from './selector-at-rule-surface.mjs';

const inventory = JSON.parse(await readFile(
  new URL('../../tests/fixtures/css/c01/selector-at-rule-inventory.v1.json', import.meta.url),
  'utf8',
));
const zeroHash = '0'.repeat(64);
const chromiumRevision = '@0e66b94549a8bd85f03fbdf189b752d6e7b63c32';
const userAgent = 'Mozilla/5.0 HeadlessChrome/155.0.0.0';

function styleRule(selectorText = '.target', cssText = '.target { color: red; }') {
  return {
    constructor: 'CSSStyleRule',
    type: 1,
    cssText,
    selectorText,
    name: null,
    conditionText: null,
    children: [],
  };
}

function topRule(atRule) {
  const childCount = atRule.expectedChildrenCount;
  let children = null;
  if (childCount !== null) {
    children = Array.from({ length: childCount }, (_, index) => {
      if (atRule.id === 'at-rule.keyframes') {
        return {
          constructor: 'CSSKeyframeRule',
          type: 8,
          cssText: index === 0 ? '0% { opacity: 0; }' : '100% { opacity: 1; }',
          selectorText: null,
          name: null,
          conditionText: null,
          children: null,
        };
      }
      return styleRule();
    });
  }
  return {
    constructor: atRule.expectedConstructor,
    type: atRule.expectedType,
    cssText: atRule.source,
    selectorText: atRule.id === 'at-rule.page' ? '' : null,
    name: null,
    conditionText: null,
    children,
  };
}

function refreshObservationHash(snapshot) {
  const value = {
    selectorObservations: snapshot.inventory.selectorObservations,
    atRuleObservations: snapshot.inventory.atRuleObservations,
  };
  snapshot.inventory.observationSha256 = createHash('sha256')
    .update(`${JSON.stringify(value, null, 2)}\n`, 'utf8')
    .digest('hex');
}

function validSnapshot() {
  const selectorObservations = inventory.selectors.map((selector) => {
    if (selector.kind === 'query') {
      return {
        id: selector.id,
        kind: selector.kind,
        source: selector.source,
        outcome: 'matched',
        matches: [...selector.expectedMatches],
      };
    }
    if (selector.kind === 'invalid-query') {
      return {
        id: selector.id,
        kind: selector.kind,
        source: selector.source,
        outcome: 'exception',
        exceptionName: 'SyntaxError',
      };
    }
    return {
      id: selector.id,
      kind: selector.kind,
      source: selector.source,
      outcome: 'rule-parse',
      rule: {
        constructor: 'CSSStyleRule',
        type: 1,
        cssText: `${selector.source} { content: ""; }`,
        selectorText: selector.source,
        name: null,
        conditionText: null,
        children: [],
      },
    };
  });
  const atRuleObservations = inventory.atRules.map((atRule) => ({
    id: atRule.id,
    kind: atRule.kind,
    source: atRule.source,
    outcome: atRule.expectedOutcome,
    rules: atRule.expectedOutcome === 'retained' ? [topRule(atRule)] : [],
  }));
  const snapshot = {
    schema: 'spinon-css-selector-at-rule-surface/v1',
    referenceId: 'pending',
    capturedAtUtc: '2026-10-19T00:00:00.000Z',
    captureTool: {
      path: 'tools/css-reference/capture-selector-at-rule-surface.mjs',
      sha256: zeroHash,
      supportModules: [
        { path: 'tools/css-reference/chromium-session.mjs', sha256: zeroHash },
        { path: 'tools/css-reference/selector-at-rule-surface.mjs', sha256: zeroHash },
      ],
    },
    oracle: {
      name: 'Chromium',
      product: 'Chrome/155.0.8059.85',
      version: '155.0.8059.85',
      revision: chromiumRevision,
      userAgent,
      executable: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
      executableSha256: zeroHash,
    },
    environment: {
      os: 'macOS 26.5.1',
      osBuild: '25F80',
      architecture: 'arm64',
      nodeVersion: 'v24.19.0',
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
      userAgent,
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
        userAgent,
      },
    },
    fixture: {
      id: inventory.fixtureId,
      path: 'tests/fixtures/css/c01/selector-at-rule-surface.html',
      sha256: zeroHash,
      nodeIds: [...inventory.expectedNodeIds],
      elementCount: inventory.expectedNodeIds.length,
      authorStyleSheetCount: 0,
      externalResourceCount: 0,
    },
    inventory: {
      id: inventory.inventoryId,
      completeness: 'partial',
      sha256: createHash('sha256').update(`${JSON.stringify(inventory, null, 2)}\n`).digest('hex'),
      uncovered: [...inventory.uncovered],
      selectorObservations,
      atRuleObservations,
      observationSha256: zeroHash,
    },
  };
  refreshObservationHash(snapshot);
  snapshot.referenceId = buildSelectorAtRuleReferenceId(snapshot);
  return snapshot;
}

function assertInvalidSnapshot(change, pattern) {
  const snapshot = structuredClone(validSnapshot());
  change(snapshot);
  if (snapshot.inventory?.selectorObservations || snapshot.inventory?.atRuleObservations) refreshObservationHash(snapshot);
  assert.throws(() => validateSelectorAtRuleSnapshot(snapshot, inventory), pattern);
}

test('C01.4 inventory의 안정 ID·고정 입력·partial 범위를 검증한다', () => {
  assert.equal(validateSelectorAtRuleInventory(inventory), inventory);
});

test('C01.4 v1 inventory의 expected output과 입력 byte hash를 고정한다', () => {
  assert.equal(createHash('sha256').update(`${JSON.stringify(inventory, null, 2)}\n`, 'utf8').digest('hex'),
    '970e9f0aab8134b603e6e4f4c790ad2c4b59709beb7eee927b276fa728ece7f8');
  const candidate = structuredClone(inventory);
  candidate.selectors[0].source = 'section *';
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /고정 inventory SHA-256/);
});

test('고정 Chromium observation snapshot을 검증한다', () => {
  assert.equal(validateSelectorAtRuleSnapshot(validSnapshot(), inventory).schema, 'spinon-css-selector-at-rule-surface/v1');
});

test('inventory의 알 수 없는 최상위 필드를 거부한다', () => {
  const candidate = { ...inventory, supported: true };
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /고정 schema/);
});

test('inventory의 partial 표기를 전체로 바꾸지 못하게 한다', () => {
  const candidate = { ...inventory, completeness: 'complete' };
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /completeness/);
});

test('selector stable ID 중복을 거부한다', () => {
  const candidate = structuredClone(inventory);
  candidate.selectors[1].id = candidate.selectors[0].id;
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /중복/);
});

test('selector expected match의 미지 node ID를 거부한다', () => {
  const candidate = structuredClone(inventory);
  candidate.selectors[0].expectedMatches = ['unknown-node'];
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /미지/);
});

test('selector expected match의 document 순서 역전을 거부한다', () => {
  const candidate = structuredClone(inventory);
  candidate.selectors.find(({ id }) => id === 'selector.class.target').expectedMatches = ['sibling', 'target'];
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /document 순서/);
});

test('external CSS import와 URL을 inventory 입력으로 허용하지 않는다', () => {
  const candidate = structuredClone(inventory);
  candidate.atRules[0].source = '@import url("https://example.invalid/a.css");';
  assert.throws(() => validateSelectorAtRuleInventory(candidate), /외부 자원/);
});

test('관찰된 selector가 inventory와 다른 node set이면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.selectorObservations[0].matches = ['body']; }, /사전 비교 기준/);
});

test('관찰된 selector의 document order가 기준과 다르면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => {
    snapshot.inventory.selectorObservations[1].matches = ['sibling', 'target', 'nested-target'];
  }, /사전 비교 기준/);
});

test('invalid selector가 syntax error 대신 accepted 결과를 내면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => {
    snapshot.inventory.selectorObservations[20] = {
      id: inventory.selectors[20].id,
      kind: 'invalid-query',
      source: inventory.selectors[20].source,
      outcome: 'matched',
      matches: [],
    };
  }, /고정 schema/);
});

test('pseudo-element는 DOM match가 아니라 CSSStyleRule로 검증한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.selectorObservations[19].rule.selectorText = 'section'; }, /pseudo-element CSSOM selector/);
});

test('unknown at-rule drop과 parse exception을 구분한다', () => {
  assertInvalidSnapshot((snapshot) => {
    const atRule = snapshot.inventory.atRuleObservations[11];
    atRule.outcome = 'exception';
    atRule.exceptionName = 'SyntaxError';
  }, /사전 비교 기준/);
});

test('at-rule retained count가 기준과 다르면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.atRuleObservations[0].rules = []; }, /CSSOM rule count/);
});

test('at-rule CSSRule constructor/type 차이를 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.atRuleObservations[0].rules[0].type = 12; }, /top-level CSSRule shape/);
});

test('at-rule 하위 rule 수 차이를 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.atRuleObservations[0].rules[0].children = []; }, /top-level CSSRule shape/);
});

test('CSSRule shape의 알 수 없는 필드를 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.atRuleObservations[0].rules[0].supported = true; }, /고정 schema/);
});

test('observation 내용이 바뀌고 digest가 유지되면 거부한다', () => {
  const snapshot = structuredClone(validSnapshot());
  snapshot.inventory.selectorObservations[0].matches = [];
  assert.throws(() => validateSelectorAtRuleSnapshot(snapshot, inventory), /사전 비교 기준/);
});

test('inventory input hash가 다르면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.inventory.sha256 = 'f'.repeat(64); }, /source hash/);
});

test('Chromium revision이 계획 기준과 다르면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.oracle.revision = '@deadbeef'; }, /oracle/);
});

test('viewport 또는 locale emulation이 달라지면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.environment.emulation.locale = 'ko-KR'; }, /emulation/);
});

test('reference ID가 provenance와 일치하지 않으면 거부한다', () => {
  assertInvalidSnapshot((snapshot) => { snapshot.referenceId = 'wrong-reference'; }, /referenceId/);
});

test('snapshot 재현 비교는 capturedAtUtc만 무시한다', () => {
  const expected = validSnapshot();
  const repeated = structuredClone(expected);
  repeated.capturedAtUtc = '2026-10-19T00:01:00.000Z';
  assert.equal(equivalentSelectorAtRuleSnapshots(expected, repeated), true);
  repeated.oracle.executableSha256 = 'f'.repeat(64);
  assert.equal(equivalentSelectorAtRuleSnapshots(expected, repeated), false);
});
