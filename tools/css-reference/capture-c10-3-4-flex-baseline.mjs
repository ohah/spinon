import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { arch, platform, release } from 'node:os';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryRelativePath = 'tests/fixtures/css/c10/flex-baseline-inventory.json';
const htmlRelativePath = 'tests/fixtures/css/c10/flex-baseline.html';
const runtimeRelativePath = 'tests/fixtures/css/c10/runtime-flex-baseline.js';
const captureRelativePath = 'tools/css-reference/capture-c10-3-4-flex-baseline.mjs';
const helperRelativePath = 'tools/css-reference/chromium-session.mjs';
const outputRelativePath = 'tests/fixtures/css/references/c10-3-4-flex-baseline-v1.json';
const inventoryPath = join(repositoryRoot, inventoryRelativePath);
const htmlPath = join(repositoryRoot, htmlRelativePath);
const runtimePath = join(repositoryRoot, runtimeRelativePath);
const outputPath = join(repositoryRoot, outputRelativePath);
const replaceReference = process.argv.includes('--replace-reference');
const unknownArguments = process.argv.slice(2).filter((argument) => argument !== '--replace-reference');
if (unknownArguments.length > 0) {
  throw new Error('알 수 없는 인수: ' + unknownArguments.join(' '));
}
const [inventoryBytes, htmlBytes, runtimeBytes] = await Promise.all([
  readFile(inventoryPath), readFile(htmlPath), readFile(runtimePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const caseIds = inventory.cases.map(({ id }) => id);
const expectedCaseIds = [
  'first-group', 'first-explicit-and-place-self', 'last-group', 'mixed-groups',
  'single-first-participant', 'single-last-participant', 'last-cross-margins',
  'border-padding-synthesized', 'column-fallback', 'align-content-first-baseline',
  'align-content-last-invalid-fallback', 'nested-first-wrap', 'nested-last-wrap',
  'nested-first-wrap-reverse-order', 'wrapped-first-parent-stretch',
  'wrapped-last-parent-stretch', 'wrapped-last-parent-wrap-reverse-order',
  'wrapped-last-parent-single-item-lines',
];
const allIds = inventory.cases.flatMap(({ nodes }) => nodes.map(({ id }) => id));
if (inventory.schema !== 'spinon-css-c10-3-4-flex-baseline-inventory/v1'
  || inventory.fixtureId !== 'C10.3.4-flex-baseline-v1'
  || inventory.viewport?.width !== 320 || inventory.viewport?.height !== 640
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || inventory.environment?.writingMode !== 'horizontal-tb'
  || inventory.environment?.direction !== 'ltr'
  || JSON.stringify(caseIds) !== JSON.stringify(expectedCaseIds)
  || !inventory.cases.every(({ id, nodes }) => typeof id === 'string'
    && Array.isArray(nodes) && nodes.length > 1
    && nodes.every(({ id: nodeId, children }) => typeof nodeId === 'string'
      && Array.isArray(children)))
  || inventory.wpt?.revision !== 'd5a765f1089ce6d3f72300281481edf3dddff7f3'
  || !inventory.wpt.paths?.length) {
  throw new Error('C10.3.4 inventory의 고정 계약이 올바르지 않습니다.');
}
for (const fixtureCase of inventory.cases) {
  const ids = fixtureCase.nodes.map(({ id }) => id);
  if (new Set(ids).size !== ids.length) throw new Error(`중복 node ID: ${fixtureCase.id}`);
  for (const { id, children } of fixtureCase.nodes) {
    if (children.some((child) => !ids.includes(child))) {
      throw new Error(`fixture 밖의 자식 node: ${fixtureCase.id}/${id}`);
    }
  }
}
if (new Set(allIds).size !== allIds.length) throw new Error('case 사이 node ID가 중복됩니다.');
let referenceExists = true;
try {
  await access(outputPath);
} catch (error) {
  if (error.code !== 'ENOENT') throw error;
  referenceExists = false;
}
if (referenceExists && !replaceReference) {
  throw new Error('기존 Chromium reference를 덮어쓰지 않습니다: ' + outputRelativePath);
}

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
if (versionResult.error || versionResult.status !== 0) {
  throw new Error('Chromium 버전을 읽지 못했습니다: '
    + (versionResult.error?.message ?? versionResult.stderr));
}
const cliVersion = versionResult.stdout.trim();
if (cliVersion !== 'Google Chrome 154.0.8037.98') {
  throw new Error('Chromium 기준 버전이 고정값과 다릅니다: ' + cliVersion);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error('DevTools Chromium 기준 버전이 고정값과 다릅니다: '
        + JSON.stringify(browserVersion));
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setLocaleOverride', { locale: inventory.environment.locale });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: inventory.environment.timeZone });
    await page.send('Emulation.setUserAgentOverride', {
      userAgent: browserVersion.userAgent,
      acceptLanguage: inventory.environment.locale,
    });
    await page.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
    await page.send('Emulation.setEmulatedMedia', {
      features: [
        { name: 'prefers-color-scheme', value: inventory.environment.colorScheme },
        { name: 'forced-colors', value: inventory.environment.forcedColors },
        { name: 'pointer', value: inventory.environment.pointer },
        { name: 'hover', value: inventory.environment.hover },
      ],
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: pathToFileURL(htmlPath).href });
    await loaded;

    const observations = [];
    for (const deviceScaleFactor of inventory.viewport.deviceScaleFactors) {
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: inventory.viewport.width,
        height: inventory.viewport.height,
        deviceScaleFactor,
        mobile: false,
        screenWidth: inventory.viewport.width,
        screenHeight: inventory.viewport.height,
      });
      const cases = [];
      for (const fixtureCase of inventory.cases) {
        const expression = '(() => {'
          + 'const caseId=' + JSON.stringify(fixtureCase.id) + ';'
          + 'const expected=' + JSON.stringify(fixtureCase.nodes) + ';'
          + 'const section=document.querySelector("[data-c1034-case=\\""+caseId+"\\"]");'
          + 'if(!section)throw new Error("fixture case missing: "+caseId);'
          + 'for(const candidate of document.querySelectorAll("[data-c1034-case]"))'
          + 'candidate.style.display=candidate===section?"block":"none";'
          + 'const found=Array.from(section.querySelectorAll("[data-c1034-node]"));'
          + 'const nodes=expected.map((spec,index)=>{'
          + 'const element=found[index];'
          + 'if(!element||element.id!==spec.id)throw new Error("fixture node order mismatch: "+caseId+"/"+spec.id);'
          + 'const rect=element.getBoundingClientRect();const style=getComputedStyle(element);'
          + 'const children=Array.from(element.children).filter(child=>child.hasAttribute("data-c1034-node")).map(child=>child.id);'
          + 'if(JSON.stringify(children)!==JSON.stringify(spec.children))throw new Error("fixture child order mismatch: "+caseId+"/"+spec.id);'
          + 'return {id:element.id,input:{tag:element.localName,style:element.getAttribute("style")??"",'
          + 'parentId:element.parentElement?.closest("[data-c1034-node]")?.id??null},children,'
          + 'properties:Object.fromEntries(' + JSON.stringify(inventory.comparison.computedProperties)
          + '.map(property=>[property,style.getPropertyValue(property).trim()])), '
          + 'rect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height}};});'
          + 'if(found.length!==expected.length)throw new Error("unexpected fixture nodes: "+caseId);'
          + 'const root=section.querySelector("[data-c1034-node]");const rootStyle=getComputedStyle(root);'
          + 'return JSON.stringify({id:caseId,nodes,rootStyle:{writingMode:rootStyle.writingMode,direction:rootStyle.direction}});'
          + '})()';
        const evaluation = await page.send('Runtime.evaluate', {
          expression,
          returnByValue: true,
        });
        const value = evaluation.result?.value;
        if (evaluation.exceptionDetails || typeof value !== 'string' || value.length === 0) {
          throw new Error('Chromium 관찰값을 읽지 못했습니다: '
            + (evaluation.exceptionDetails?.text ?? '결과 없음'));
        }
        const observation = JSON.parse(value);
        if (observation.id !== fixtureCase.id
          || JSON.stringify(observation.nodes.map(({ id }) => id))
            !== JSON.stringify(fixtureCase.nodes.map(({ id }) => id))
          || JSON.stringify(observation.nodes.map(({ children }) => children))
            !== JSON.stringify(fixtureCase.nodes.map(({ children }) => children))
          || observation.rootStyle.writingMode !== inventory.environment.writingMode
          || observation.rootStyle.direction !== inventory.environment.direction
          || !observation.nodes.every(({ input }) => typeof input?.tag === 'string'
            && typeof input.style === 'string'
            && (input.parentId === null || typeof input.parentId === 'string'))
          || !observation.nodes.every((node) => inventory.comparison.computedProperties.every(
            (property) => typeof node.properties[property] === 'string',
          ) && inventory.comparison.rectFields.every((field) => Number.isFinite(node.rect[field])))) {
          throw new Error('Chromium observation이 inventory/environment와 다릅니다: '
            + JSON.stringify({ case: fixtureCase.id, observation }));
        }
        cases.push(observation);
      }
      const environmentResult = await page.send('Runtime.evaluate', {
        expression: 'JSON.stringify({width:innerWidth,height:innerHeight,'
          + 'deviceScaleFactor:devicePixelRatio,locale:navigator.language,'
          + 'timeZone:Intl.DateTimeFormat().resolvedOptions().timeZone,'
          + 'dark:matchMedia("(prefers-color-scheme: dark)").matches,'
          + 'forced:matchMedia("(forced-colors: active)").matches,'
          + 'coarsePointer:matchMedia("(pointer: coarse)").matches,'
          + 'hover:matchMedia("(hover: hover)").matches})',
        returnByValue: true,
      });
      const environment = JSON.parse(environmentResult.result.value);
      if (environment.width !== inventory.viewport.width
        || environment.height !== inventory.viewport.height
        || environment.deviceScaleFactor !== deviceScaleFactor
        || environment.locale !== inventory.environment.locale
        || environment.timeZone !== inventory.environment.timeZone
        || environment.dark !== false || environment.forced !== false
        || environment.coarsePointer !== true || environment.hover !== false) {
        throw new Error('Chromium 환경이 inventory와 다릅니다: ' + JSON.stringify(environment));
      }
      observations.push({
        viewport: { width: environment.width, height: environment.height,
          deviceScaleFactor: environment.deviceScaleFactor },
        environment: { locale: environment.locale, timeZone: environment.timeZone,
          dark: environment.dark, forcedColors: environment.forced,
          coarsePointer: environment.coarsePointer, hover: environment.hover },
        cases,
      });
    }
    if (JSON.stringify(observations[0].cases) !== JSON.stringify(observations[1].cases)) {
      throw new Error('DPR 변경이 CSS computed value 또는 CSS px geometry를 바꿨습니다.');
    }
    return { observations, browserVersion };
  },
});

const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const capturePath = fileURLToPath(import.meta.url);
const reference = {
  schema: 'spinon-css-c10-3-4-flex-baseline-reference/v1',
  referenceId: 'chromium-' + platform() + '-' + arch() + '-'
    + captureResult.browserVersion.product.replaceAll(/[^a-zA-Z0-9.-]/g, '-')
    + '-' + hash(inventoryBytes).slice(0, 12)
    + '-' + (await sha256File(capturePath)).slice(0, 12),
  fixture: {
    id: inventory.fixtureId,
    inventoryPath: inventoryRelativePath,
    inventorySha256: hash(inventoryBytes),
    htmlPath: htmlRelativePath,
    htmlSha256: hash(htmlBytes),
    runtimeSourcePath: runtimeRelativePath,
    runtimeSourceSha256: hash(runtimeBytes),
  },
  captureTool: {
    path: captureRelativePath,
    sha256: await sha256File(capturePath),
    dependencies: [{ path: helperRelativePath,
      sha256: await sha256File(join(repositoryRoot, helperRelativePath)) }],
  },
  wpt: inventory.wpt,
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
    viewportCssPx: { width: inventory.viewport.width, height: inventory.viewport.height },
    deviceScaleFactors: inventory.viewport.deviceScaleFactors,
    locale: inventory.environment.locale,
    timeZone: inventory.environment.timeZone,
    colorScheme: inventory.environment.colorScheme,
    forcedColors: inventory.environment.forcedColors,
    pointer: inventory.environment.pointer,
    hover: inventory.environment.hover,
    writingMode: inventory.environment.writingMode,
    direction: inventory.environment.direction,
  },
  comparison: inventory.comparison,
  observations: captureResult.observations,
};
await writeFile(
  outputPath,
  JSON.stringify(reference, null, 2) + '\n',
  replaceReference ? undefined : { flag: 'wx' },
);
console.log('Wrote ' + outputRelativePath);
