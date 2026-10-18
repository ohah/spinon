import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { link, mkdir, mkdtemp, readFile, rm, rmdir, writeFile } from 'node:fs/promises';
import { release as osRelease, tmpdir } from 'node:os';
import { join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  findChromiumExecutable,
  runChromiumPage,
  sha256File,
} from './chromium-session.mjs';
import {
  buildSelectorAtRuleReferenceId,
  equivalentSelectorAtRuleSnapshots,
  validateSelectorAtRuleInventory,
  validateSelectorAtRuleSnapshot,
} from './selector-at-rule-surface.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const fixtureRelativePath = 'tests/fixtures/css/c01/selector-at-rule-surface.html';
const fixturePath = join(repositoryRoot, fixtureRelativePath);
const inventoryRelativePath = 'tests/fixtures/css/c01/selector-at-rule-inventory.v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const captureScriptRelativePath = 'tools/css-reference/capture-selector-at-rule-surface.mjs';
const supportModulePaths = [
  'tools/css-reference/chromium-session.mjs',
  'tools/css-reference/selector-at-rule-surface.mjs',
];
const fixtureId = 'C01-selector-at-rule-surface-v1';
const htmlNamespace = 'http://www.w3.org/1999/xhtml';
const expectedBrowserVersion = '155.0.8059.85';
const expectedBrowserRevision = '@0e66b94549a8bd85f03fbdf189b752d6e7b63c32';
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

function sha256(value) {
  return createHash('sha256').update(value).digest('hex');
}

function detectHostVersion() {
  if (process.platform !== 'darwin') return { os: `${process.platform} ${osRelease()}`, osBuild: null };
  const product = spawnSync('/usr/bin/sw_vers', ['-productVersion'], { encoding: 'utf8', timeout: 5_000 });
  const build = spawnSync('/usr/bin/sw_vers', ['-buildVersion'], { encoding: 'utf8', timeout: 5_000 });
  if (product.status !== 0 || build.status !== 0) {
    throw new Error('macOS version/build를 읽지 못했습니다.');
  }
  return { os: `macOS ${product.stdout.trim()}`, osBuild: build.stdout.trim() };
}

function readChromiumCliVersion(chromiumPath) {
  const result = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
  if (result.error || result.status !== 0) {
    throw new Error(`Chromium CLI version을 읽지 못했습니다: ${result.error?.message ?? result.stderr}`);
  }
  const version = result.stdout.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
  if (!version) throw new Error(`Chromium CLI version 형식을 읽지 못했습니다: ${result.stdout.trim()}`);
  return { version, product: result.stdout.trim() };
}

function evaluatePage(page, expression) {
  return page.send('Runtime.evaluate', {
    expression,
    awaitPromise: true,
    returnByValue: true,
  }).then((result) => {
    if (result.exceptionDetails || !result.result?.value) {
      throw new Error(`Chromium CSS oracle을 관찰하지 못했습니다: ${result.exceptionDetails?.text ?? '결과 없음'}`);
    }
    return result.result.value;
  });
}

async function observeSelectorAtRuleSurface(page, browserVersion, inventory) {
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
  const encodedInventory = Buffer.from(JSON.stringify(inventory), 'utf8').toString('base64');
  const observation = await evaluatePage(page, `(async () => {
    const inventory = JSON.parse(atob('${encodedInventory}'));
    const shapeRule = (rule) => {
      const childRules = rule.cssRules;
      return {
        constructor: rule.constructor.name,
        type: rule.type,
        cssText: rule.cssText,
        selectorText: typeof rule.selectorText === 'string' ? rule.selectorText : null,
        name: typeof rule.name === 'string' ? rule.name : null,
        conditionText: typeof rule.conditionText === 'string' ? rule.conditionText : null,
        children: childRules ? Array.from(childRules, shapeRule) : null,
      };
    };
    const elements = Array.from(document.querySelectorAll('*'));
    const nodeIds = elements.map((element) => {
      const id = element.getAttribute('data-c01-node');
      if (!id) throw new Error('고정 fixture의 모든 요소에 data-c01-node가 있어야 합니다.');
      return id;
    });
    if (new Set(nodeIds).size !== nodeIds.length) throw new Error('fixture node ID가 중복됐습니다.');

    const selectorObservations = inventory.selectors.map((selector) => {
      if (selector.kind === 'pseudo-element-rule') {
        try {
          const sheet = new CSSStyleSheet();
          sheet.replaceSync(selector.source + ' { content: ""; }');
          const rules = Array.from(sheet.cssRules, shapeRule);
          return { id: selector.id, kind: selector.kind, source: selector.source, outcome: 'rule-parse', rule: rules[0] ?? null };
        } catch (error) {
          return { id: selector.id, kind: selector.kind, source: selector.source, outcome: 'exception', exceptionName: error.name };
        }
      }
      try {
        const matches = Array.from(document.querySelectorAll(selector.source), (element) => {
          const id = element.getAttribute('data-c01-node');
          if (!id) throw new Error('selector가 고정 ID가 없는 fixture 요소와 일치했습니다.');
          return id;
        });
        return { id: selector.id, kind: selector.kind, source: selector.source, outcome: 'matched', matches };
      } catch (error) {
        return { id: selector.id, kind: selector.kind, source: selector.source, outcome: 'exception', exceptionName: error.name };
      }
    });

    const atRuleObservations = inventory.atRules.map((atRule) => {
      try {
        const sheet = new CSSStyleSheet();
        sheet.replaceSync(atRule.source);
        const rules = Array.from(sheet.cssRules, shapeRule);
        return {
          id: atRule.id,
          kind: atRule.kind,
          source: atRule.source,
          outcome: rules.length === 0 ? 'dropped' : 'retained',
          rules,
        };
      } catch (error) {
        return {
          id: atRule.id,
          kind: atRule.kind,
          source: atRule.source,
          outcome: 'exception',
          exceptionName: error.name,
          rules: [],
        };
      }
    });

    return {
      fixture: {
        elementCount: elements.length,
        nodeIds,
        authorStyleSheetCount: document.styleSheets.length,
        externalResourceCount: performance.getEntriesByType('resource').length,
      },
      observed: {
        viewport: { width: innerWidth, height: innerHeight, deviceScaleFactor: devicePixelRatio },
        locale: navigator.language,
        intlLocale: Intl.DateTimeFormat().resolvedOptions().locale,
        timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone,
        media: {
          prefersColorSchemeDark: matchMedia('(prefers-color-scheme: dark)').matches,
          prefersReducedMotion: matchMedia('(prefers-reduced-motion: reduce)').matches,
          forcedColors: matchMedia('(forced-colors: active)').matches,
        },
        userAgent: navigator.userAgent,
      },
      selectorObservations,
      atRuleObservations,
    };
  })()`);

  if (observation.fixture.elementCount !== inventory.expectedNodeIds.length
    || JSON.stringify(observation.fixture.nodeIds) !== JSON.stringify(inventory.expectedNodeIds)) {
    throw new Error(`고정 fixture node ID가 inventory와 다릅니다: ${JSON.stringify(observation.fixture.nodeIds)}`);
  }
  if (observation.fixture.authorStyleSheetCount !== 0 || observation.fixture.externalResourceCount !== 0) {
    throw new Error(`fixture에 author stylesheet 또는 resource가 있습니다: ${JSON.stringify(observation.fixture)}`);
  }
  if (observation.observed.viewport.width !== fixedEmulation.cssViewportPx.width
    || observation.observed.viewport.height !== fixedEmulation.cssViewportPx.height
    || observation.observed.viewport.deviceScaleFactor !== fixedEmulation.deviceScaleFactor
    || observation.observed.locale !== fixedEmulation.locale
    || observation.observed.intlLocale !== fixedEmulation.locale
    || observation.observed.timeZone !== fixedEmulation.timeZone
    || observation.observed.media.prefersColorSchemeDark !== false
    || observation.observed.media.prefersReducedMotion !== false
    || observation.observed.media.forcedColors !== false) {
    throw new Error(`실제 browser emulation 관찰이 고정 입력과 다릅니다: ${JSON.stringify(observation.observed)}`);
  }
  return observation;
}

const args = process.argv.slice(2);
const verifyExisting = args.length === 1 && args[0] === '--verify-existing';
if (args.length > 0 && !verifyExisting) throw new Error('지원하는 인수는 --verify-existing 하나입니다.');
const inventoryBytes = await readFile(inventoryPath);
const inventory = validateSelectorAtRuleInventory(JSON.parse(inventoryBytes.toString('utf8')));
const fixtureBytes = await readFile(fixturePath);
const fixtureSha256 = sha256(fixtureBytes);
const inventorySha256 = sha256(inventoryBytes);
const chromiumPath = await findChromiumExecutable();
const cliVersion = readChromiumCliVersion(chromiumPath);
if (cliVersion.version !== expectedBrowserVersion) {
  throw new Error(`Chromium CLI version이 계획된 기준과 다릅니다: ${cliVersion.version}`);
}
const executableSha256 = await sha256File(chromiumPath);
const captureSha256 = await sha256File(fileURLToPath(import.meta.url));
const supportModules = await Promise.all(supportModulePaths.map(async (path) => ({
  path,
  sha256: await sha256File(join(repositoryRoot, path)),
})));
const hostVersion = detectHostVersion();

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion, browserFlags: activeFlags }) => {
    const version = browserVersion.product.match(/\d+\.\d+\.\d+\.\d+/)?.[0];
    if (version !== cliVersion.version || version !== expectedBrowserVersion) {
      throw new Error(`Chromium CLI/CDP version이 계획 기준과 다릅니다: ${cliVersion.version}/${browserVersion.product}`);
    }
    if (browserVersion.revision !== expectedBrowserRevision) {
      throw new Error(`Chromium revision이 계획된 기준과 다릅니다: ${browserVersion.revision}`);
    }
    const observed = await observeSelectorAtRuleSurface(page, browserVersion, inventory);
    const snapshot = {
      schema: 'spinon-css-selector-at-rule-surface/v1',
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
        userAgent: observed.observed.userAgent,
        observed: observed.observed,
      },
      fixture: {
        id: fixtureId,
        path: fixtureRelativePath,
        sha256: fixtureSha256,
        nodeIds: observed.fixture.nodeIds,
        elementCount: observed.fixture.elementCount,
        authorStyleSheetCount: observed.fixture.authorStyleSheetCount,
        externalResourceCount: observed.fixture.externalResourceCount,
      },
      inventory: {
        id: inventory.inventoryId,
        completeness: inventory.completeness,
        sha256: inventorySha256,
        uncovered: inventory.uncovered,
        selectorObservations: observed.selectorObservations,
        atRuleObservations: observed.atRuleObservations,
      },
    };
    snapshot.inventory.observationSha256 = sha256(`${JSON.stringify({
      selectorObservations: snapshot.inventory.selectorObservations,
      atRuleObservations: snapshot.inventory.atRuleObservations,
    }, null, 2)}\n`);
    snapshot.referenceId = buildSelectorAtRuleReferenceId(snapshot);
    validateSelectorAtRuleSnapshot(snapshot, inventory);
    return snapshot;
  },
});

const snapshot = captureResult;
const outputRoot = join(repositoryRoot, 'tests/fixtures/css/references');
const outputDirectory = join(outputRoot, snapshot.referenceId);
const outputFile = join(outputDirectory, 'selector-at-rule-surface.json');
await mkdir(outputRoot, { recursive: true });

if (verifyExisting) {
  let existing;
  try {
    existing = JSON.parse(await readFile(outputFile, 'utf8'));
  } catch (error) {
    throw new Error(`기존 reference를 읽지 못했습니다: ${outputFile}`, { cause: error });
  }
  validateSelectorAtRuleSnapshot(existing, inventory);
  if (!equivalentSelectorAtRuleSnapshots(existing, snapshot)) {
    throw new Error(`고정 Chromium 관찰이 기존 reference와 다릅니다: ${outputFile}`);
  }
  process.stdout.write(`C01.4 reference 재현 확인: ${snapshot.referenceId}\n`);
} else {
  const pendingDirectory = await mkdtemp(join(outputRoot, '.c01-selector-at-rule-pending-'));
  const pendingFile = join(pendingDirectory, 'selector-at-rule-surface.json');
  let outputDirectoryReserved = false;
  try {
    await writeFile(pendingFile, `${JSON.stringify(snapshot, null, 2)}\n`, { flag: 'wx' });
    validateSelectorAtRuleSnapshot(JSON.parse(await readFile(pendingFile, 'utf8')), inventory);
    await mkdir(outputDirectory);
    outputDirectoryReserved = true;
    await link(pendingFile, outputFile);
  } catch (error) {
    if (outputDirectoryReserved) {
      try {
        await rmdir(outputDirectory);
      } catch {
        // Preserve any unexpected file that appeared in the new output directory.
      }
    }
    throw error;
  } finally {
    await rm(pendingDirectory, { recursive: true, force: true });
  }
  process.stdout.write(`C01.4 reference 생성: ${snapshot.referenceId}\n`);
  process.stdout.write(`JSON: ${outputFile}\n`);
}
process.stdout.write(`selector=${snapshot.inventory.selectorObservations.length}, at-rule=${snapshot.inventory.atRuleObservations.length}, Chromium=${snapshot.oracle.version}\n`);
