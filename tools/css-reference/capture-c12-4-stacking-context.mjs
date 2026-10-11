import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { arch, platform, release } from 'node:os';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const inventoryPath = 'tests/fixtures/css/c12/stacking-context-inventory.json';
const htmlPath = 'tests/fixtures/css/c12/stacking-context.html';
const capturePath = 'tools/css-reference/capture-c12-4-stacking-context.mjs';
const helperPath = 'tools/css-reference/chromium-session.mjs';
const referencePath = 'tests/fixtures/css/references/c12-4-stacking-context-v1.json';
const screenshotDirectory = 'spec/internal/evidence/c12-4-stacking-context-2026-10-11';
const inventoryAbsolutePath = join(repositoryRoot, inventoryPath);
const htmlAbsolutePath = join(repositoryRoot, htmlPath);
const referenceAbsolutePath = join(repositoryRoot, referencePath);
const screenshotAbsoluteDirectory = join(repositoryRoot, screenshotDirectory);
const replaceReference = process.argv.includes('--replace-reference');
if (process.argv.slice(2).some((argument) => argument !== '--replace-reference')) {
  throw new Error('인수는 --replace-reference만 허용합니다.');
}

const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const assertUnique = (values, label) => {
  if (new Set(values).size !== values.length) throw new Error(`${label} ID가 중복되었습니다.`);
};
const [inventoryBytes, htmlBytes] = await Promise.all([
  readFile(inventoryAbsolutePath),
  readFile(htmlAbsolutePath),
]);
const inventory = JSON.parse(inventoryBytes.toString('utf8'));
const cases = inventory.cases;
const controls = inventory.negativeControls;
const caseIds = cases?.map(({ id }) => id) ?? [];
const controlIds = controls?.map(({ id }) => id) ?? [];
const nodeIds = cases?.flatMap(({ nodes }) => nodes.map(({ id }) => id)) ?? [];
const caseCount = caseIds.length + controlIds.length;
const expectedChromeSha256 = 'ccffd5c5fe77ba3212e07c2c69091f8048b73c396b4647ecaa150e1e01a04954';

if (inventory.schema !== 'spinon-css-c12-4-stacking-inventory/v1'
  || inventory.fixtureId !== 'C12.4-stacking-context-v1'
  || inventory.viewport?.widthCssPx !== 1000 || inventory.viewport?.heightCssPx !== 1400
  || JSON.stringify(inventory.viewport?.deviceScaleFactors) !== '[1,2]'
  || inventory.environment?.locale !== 'en-US' || inventory.environment?.timeZone !== 'UTC'
  || inventory.environment?.writingMode !== 'horizontal-tb' || inventory.environment?.direction !== 'ltr'
  || !Array.isArray(cases) || cases.length < 20
  || !Array.isArray(controls) || controls.length < 20
  || caseCount > inventory.layout?.maximumItems
  || inventory.layout?.columns !== 4
  || inventory.layout?.stageWidthCssPx < 230
  || inventory.layout?.originCssPx?.x !== 10 || inventory.layout?.originCssPx?.y !== 10
  || inventory.comparison?.maximumAbsoluteRectErrorCssPx !== 0.5
  || inventory.comparison?.maximumPixelChannelError !== 1
  || inventory.comparison?.pixelColorSpace !== 'sRGB'
  || inventory.wpt?.revision !== 'c999c58338ee1d223df5ad62e48be1202ced0d37'
  || inventory.wpt?.execution !== 'not-run'
  || inventory.wpt.sources?.length !== 5
  || !htmlBytes.toString('utf8').includes('id="spinon-c124-mount"')
  || !htmlBytes.toString('utf8').includes('background: #112233')) {
  throw new Error('C12.4 inventory의 고정 계약, viewport, WPT 기준 또는 HTML root가 올바르지 않습니다.');
}
assertUnique(caseIds, 'case');
assertUnique(controlIds, 'negative control');
assertUnique([...nodeIds, ...controlIds, ...caseIds], 'fixture');
if (nodeIds.length === 0 || new Set(nodeIds).size !== nodeIds.length) {
  throw new Error('C12.4 node ID가 비어 있거나 중복되었습니다.');
}
for (const fixtureCase of cases) {
  const ids = fixtureCase.nodes.map(({ id }) => id);
  if (ids.length === 0 || new Set(ids).size !== ids.length || !Array.isArray(fixtureCase.samples)) {
    throw new Error(`C12.4 case node 또는 sample 목록이 올바르지 않습니다: ${fixtureCase.id}`);
  }
  const present = new Set();
  for (const node of fixtureCase.nodes) {
    if (!node.id || typeof node.style !== 'string' || (node.tag !== undefined && node.tag !== 'dialog' && node.tag !== 'div')) {
      throw new Error(`C12.4 node 선언이 올바르지 않습니다: ${fixtureCase.id}/${node.id}`);
    }
    if (node.parentId !== null && node.parentId !== undefined && !present.has(node.parentId)) {
      throw new Error(`C12.4 parent가 preorder에 없거나 앞서 생성되지 않았습니다: ${fixtureCase.id}/${node.id}`);
    }
    present.add(node.id);
  }
  for (const sample of fixtureCase.samples) {
    if (!Number.isFinite(sample.xCssPx) || !Number.isFinite(sample.yCssPx)
      || sample.xCssPx < 0 || sample.yCssPx < 0
      || sample.xCssPx >= inventory.layout.stageWidthCssPx
      || sample.yCssPx >= inventory.layout.stageHeightCssPx
      || !Array.isArray(sample.topToBottom) || !Array.isArray(sample.rgba)
      || sample.rgba.length !== 4 || sample.topToBottom.some((id) => !ids.includes(id))) {
      throw new Error(`C12.4 overlap sample 선언이 올바르지 않습니다: ${fixtureCase.id}/${sample.id}`);
    }
  }
}
for (const control of controls) {
  if (!control.id || (control.style !== undefined && typeof control.style !== 'string')) {
    throw new Error(`C12.4 negative control 선언이 올바르지 않습니다: ${control.id}`);
  }
}
for (const source of inventory.wpt.sources) {
  if (!/^[0-9a-f]{64}$/.test(source.sha256) || !Array.isArray(source.mapsTo) || source.mapsTo.length === 0) {
    throw new Error(`고정 WPT 원문의 hash 또는 case 연결이 올바르지 않습니다: ${source.path}`);
  }
}

if (!replaceReference) {
  for (const path of [referenceAbsolutePath, join(screenshotAbsoluteDirectory, 'chrome-dpr-1.png'),
    join(screenshotAbsoluteDirectory, 'chrome-dpr-2.png')]) {
    try {
      await access(path);
      throw new Error(`기존 Chrome 기준은 덮어쓰지 않습니다: ${path}`);
    } catch (error) {
      if (error?.code !== 'ENOENT') throw error;
    }
  }
}

const stylesheetRules = inventory.stylesheetRules.join('\n');
const setupExpression = `(() => {
  const inventory = ${JSON.stringify(inventory)};
  const mount = document.getElementById('spinon-c124-mount');
  if (!mount) throw new Error('C12.4 mount가 없습니다.');
  const style = document.createElement('style');
  style.textContent = ${JSON.stringify(stylesheetRules)};
  document.head.appendChild(style);
  const { columns, columnStepCssPx, rowStepCssPx, stageWidthCssPx, stageHeightCssPx, originCssPx } = inventory.layout;
  const itemOrigins = Object.create(null);
  const positionItem = (element, index) => {
    const left = originCssPx.x + (index % columns) * columnStepCssPx;
    const top = originCssPx.y + Math.floor(index / columns) * rowStepCssPx;
    element.style.cssText += ';position:absolute;left:' + left + 'px;top:' + top + 'px;width:'
      + stageWidthCssPx + 'px;height:' + stageHeightCssPx + 'px;--tile-x:' + left + 'px;--tile-y:' + top + 'px';
    itemOrigins[element.dataset.c124Case ?? element.dataset.c124Control] = { x:left, y:top };
  };
  for (const [index, fixtureCase] of inventory.cases.entries()) {
    const stage = document.createElement('div');
    stage.id = 'c124-stage-' + fixtureCase.id;
    stage.dataset.c124Case = fixtureCase.id;
    positionItem(stage, index);
    mount.appendChild(stage);
    const refs = Object.create(null);
    for (const spec of fixtureCase.nodes) {
      const element = document.createElement(spec.tag ?? 'div');
      element.id = spec.id;
      element.dataset.c124Node = '';
      element.dataset.c124Case = fixtureCase.id;
      if (spec.paint) element.dataset.c124Paint = '';
      if (spec.className) element.className = spec.className;
      element.setAttribute('style', spec.style);
      const parent = spec.parentId === null || spec.parentId === undefined ? stage : refs[spec.parentId];
      if (!parent) throw new Error('C12.4 fixture parent가 없습니다: ' + fixtureCase.id + '/' + spec.id);
      parent.appendChild(element);
      refs[spec.id] = element;
    }
  }
  for (const [index, control] of inventory.negativeControls.entries()) {
    const element = document.createElement('div');
    element.id = 'c124-control-' + control.id;
    element.dataset.c124Control = control.id;
    element.className = control.className ?? '';
    element.setAttribute('style', 'margin:0;padding:0;border:0;background-color:#7f3fbf;' + (control.style ?? ''));
    positionItem(element, inventory.cases.length + index);
    mount.appendChild(element);
  }
  return { caseCount: inventory.cases.length, controlCount: inventory.negativeControls.length,
    itemOrigins, stageWidthCssPx, stageHeightCssPx };
})()`;

const chromiumPath = await findChromiumExecutable();
const versionResult = spawnSync(chromiumPath, ['--version'], { encoding: 'utf8', timeout: 5_000 });
const chromiumSha256 = await sha256File(chromiumPath);
if (versionResult.error || versionResult.status !== 0
  || versionResult.stdout.trim() !== 'Google Chrome 154.0.8037.98'
  || chromiumSha256 !== expectedChromeSha256) {
  throw new Error(`고정 Chrome 154 실행 파일이 아닙니다: ${versionResult.stdout?.trim() ?? versionResult.stderr}; sha256=${chromiumSha256}`);
}

const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== 'Chrome/154.0.8037.98'
      || browserVersion.revision !== '@b859317bf11f6be47f9b7799ec690a0a42a1fb33') {
      throw new Error(`DevTools Chromium 기준이 고정값과 다릅니다: ${JSON.stringify(browserVersion)}`);
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Network.enable');
    await page.send('Network.emulateNetworkConditions', {
      offline: true, latency: 0, downloadThroughput: -1, uploadThroughput: -1,
    });
    await page.send('Emulation.setLocaleOverride', { locale: inventory.environment.locale });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: inventory.environment.timeZone });
    await page.send('Emulation.setUserAgentOverride', {
      userAgent: browserVersion.userAgent,
      acceptLanguage: inventory.environment.locale,
    });
    await page.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
    await page.send('Emulation.setEmulatedMedia', {
      media: 'screen',
      features: [
        { name: 'prefers-color-scheme', value: inventory.environment.colorScheme },
        { name: 'forced-colors', value: inventory.environment.forcedColors },
        { name: 'pointer', value: inventory.environment.pointer },
        { name: 'hover', value: inventory.environment.hover },
      ],
    });
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: inventory.viewport.widthCssPx,
      height: inventory.viewport.heightCssPx,
      deviceScaleFactor: inventory.viewport.deviceScaleFactors[0],
      mobile: false,
      screenWidth: inventory.viewport.widthCssPx,
      screenHeight: inventory.viewport.heightCssPx,
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: pathToFileURL(htmlAbsolutePath).href });
    await loaded;
    const setup = await page.send('Runtime.evaluate', {
      expression: setupExpression, returnByValue: true,
    });
    if (setup.exceptionDetails || !setup.result?.value
      || setup.result.value.caseCount !== cases.length
      || setup.result.value.controlCount !== controls.length) {
      throw new Error(`C12.4 fixture 구성 실패: ${JSON.stringify(setup.exceptionDetails ?? setup.result?.value)}`);
    }

    const observations = [];
    const screenshotBytes = [];
    const pixelMismatches = [];
    for (const deviceScaleFactor of inventory.viewport.deviceScaleFactors) {
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: inventory.viewport.widthCssPx,
        height: inventory.viewport.heightCssPx,
        deviceScaleFactor,
        mobile: false,
        screenWidth: inventory.viewport.widthCssPx,
        screenHeight: inventory.viewport.heightCssPx,
      });
      const captureExpression = `(() => {
        const inventory = ${JSON.stringify(inventory)};
        const readNode = (spec) => {
          const element = document.getElementById(spec.id);
          if (!element) throw new Error('C12.4 node missing: ' + spec.id);
          const style = getComputedStyle(element);
          const rect = element.getBoundingClientRect();
          const stage = document.getElementById('c124-stage-' + element.dataset.c124Case);
          const stageRect = stage.getBoundingClientRect();
          const properties = Object.fromEntries(inventory.computedProperties.map(name => [name, style.getPropertyValue(name).trim()]));
          const parentId = element.parentElement?.closest('[data-c124-node]')?.id ?? null;
          const hasLayoutBox = element.getClientRects().length > 0;
          const containingBlockOwner = (() => {
            if (!hasLayoutBox) return 'none';
            if (style.position === 'fixed') {
              for (let parent = element.parentElement; parent; parent = parent.parentElement) {
                const parentStyle = getComputedStyle(parent);
                const containsFixed = parentStyle.transform !== 'none' || parentStyle.perspective !== 'none'
                  || parentStyle.filter !== 'none' || parentStyle.backdropFilter !== 'none'
                  || parentStyle.contain.split(/\\s+/).some(value => ['layout','paint','strict','content'].includes(value))
                  || parentStyle.contentVisibility !== 'visible'
                  || parentStyle.willChange.split(/,\\s*/).some(value => ['transform','perspective','filter','backdrop-filter','contain'].includes(value));
                if (containsFixed) return parent.id || 'non-node-ancestor';
              }
              return 'viewport';
            }
            if (style.position === 'absolute') {
              for (let parent = element.parentElement; parent; parent = parent.parentElement) {
                if (getComputedStyle(parent).position !== 'static') {
                  return parent.hasAttribute('data-c124-node') ? parent.id : 'case-stage:' + element.dataset.c124Case;
                }
              }
              return 'initial-containing-block';
            }
            return null;
          })();
          return { id:spec.id, caseId:element.dataset.c124Case, tag:element.localName, parentId,
            className:element.className, inputStyle:element.getAttribute('style') ?? '', hasLayoutBox,
            containingBlockOwner, rect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height},
            frameRelativeToStage:{x:rect.x-stageRect.x,y:rect.y-stageRect.y,width:rect.width,height:rect.height},
            properties };
        };
        const cases = inventory.cases.map(fixtureCase => {
          for (const id of fixtureCase.activateTopLayer ?? []) {
            const dialog = document.getElementById(id);
            if (!(dialog instanceof HTMLDialogElement)) throw new Error('top-layer dialog가 아닙니다: ' + id);
            dialog.showModal();
          }
          const nodes = fixtureCase.nodes.map(readNode);
          const samples = fixtureCase.samples.map(sample => {
            const stageRect = document.getElementById('c124-stage-' + fixtureCase.id).getBoundingClientRect();
            const x = stageRect.x + sample.xCssPx;
            const y = stageRect.y + sample.yCssPx;
            const hitTestTopToBottom = document.elementsFromPoint(x, y)
              .filter(element => element.hasAttribute('data-c124-paint')).map(element => element.id);
            return { id:sample.id, cssPoint:{x,y}, hitTestTopToBottom };
          });
          const pseudo = fixtureCase.pseudoProbe ? (() => {
            const element = document.getElementById(fixtureCase.pseudoProbe.nodeId);
            const style = getComputedStyle(element, fixtureCase.pseudoProbe.pseudo);
            return { nodeId:fixtureCase.pseudoProbe.nodeId, pseudo:fixtureCase.pseudoProbe.pseudo,
              properties:Object.fromEntries(fixtureCase.pseudoProbe.properties.map(name => [name,style.getPropertyValue(name).trim()])) };
          })() : null;
          for (const id of fixtureCase.activateTopLayer ?? []) document.getElementById(id).close();
          return { id:fixtureCase.id, nodes, samples, pseudo };
        });
        const controls = inventory.negativeControls.map(control => {
          const element = document.getElementById('c124-control-' + control.id);
          if (!element) throw new Error('negative control missing: ' + control.id);
          const style = getComputedStyle(element);
          const rect = element.getBoundingClientRect();
          return { id:control.id, className:element.className, inputStyle:element.getAttribute('style') ?? '',
            rect:{x:rect.x,y:rect.y,width:rect.width,height:rect.height},
            properties:Object.fromEntries(inventory.computedProperties.map(name => [name,style.getPropertyValue(name).trim()])) };
        });
        return { environment:{width:innerWidth,height:innerHeight,clientWidth:document.documentElement.clientWidth,
          clientHeight:document.documentElement.clientHeight,deviceScaleFactor:devicePixelRatio,
          locale:navigator.language,timeZone:Intl.DateTimeFormat().resolvedOptions().timeZone,
          dark:matchMedia('(prefers-color-scheme: dark)').matches,
          forced:matchMedia('(forced-colors: active)').matches,
          coarsePointer:matchMedia('(pointer: coarse)').matches,
          hover:matchMedia('(hover: hover)').matches,
          screen:matchMedia('screen').matches},cases,controls};
      })()`;
      const evaluation = await page.send('Runtime.evaluate', { expression:captureExpression, returnByValue:true });
      if (evaluation.exceptionDetails || !evaluation.result?.value) {
        throw new Error(`C12.4 Chromium 관찰 실패(DPR ${deviceScaleFactor}): ${JSON.stringify(evaluation.exceptionDetails)}`);
      }
      const observation = evaluation.result.value;
      const environment = observation.environment;
      if (environment.width !== inventory.viewport.widthCssPx || environment.height !== inventory.viewport.heightCssPx
        || environment.clientWidth !== environment.width || environment.clientHeight !== environment.height
        || environment.deviceScaleFactor !== deviceScaleFactor || environment.locale !== inventory.environment.locale
        || environment.timeZone !== inventory.environment.timeZone || environment.dark !== false
        || environment.forced !== false || environment.coarsePointer !== true || environment.hover !== false
        || environment.screen !== true || observation.cases.length !== cases.length
        || observation.controls.length !== controls.length) {
        throw new Error(`고정 Chromium 환경 또는 결과 개수가 다릅니다(DPR ${deviceScaleFactor}): ${JSON.stringify(environment)}`);
      }
      for (const fixtureCase of cases) {
        const actual = observation.cases.find(({ id }) => id === fixtureCase.id);
        if (!actual || actual.nodes.length !== fixtureCase.nodes.length
          || JSON.stringify(actual.nodes.map(({ id }) => id)) !== JSON.stringify(fixtureCase.nodes.map(({ id }) => id))) {
          throw new Error(`case/node 순서가 fixture와 다릅니다(DPR ${deviceScaleFactor}/${fixtureCase.id}).`);
        }
        const actualById = new Map(actual.nodes.map((node) => [node.id, node]));
        for (const expectedNode of fixtureCase.nodes) {
          const node = actualById.get(expectedNode.id);
          if (node.parentId !== (expectedNode.parentId ?? null)) {
            throw new Error(`source parent가 다릅니다(${fixtureCase.id}/${expectedNode.id}): ${node.parentId}`);
          }
          if (!node.rect || inventory.comparison.rectFields.some((field) => !Number.isFinite(node.rect[field]))) {
            throw new Error(`유한하지 않은 Chrome frame입니다(${fixtureCase.id}/${expectedNode.id}).`);
          }
          for (const [property, value] of Object.entries(expectedNode.expectedComputed ?? {})) {
            if (node.properties[property] !== value) {
              throw new Error(`computed ${property} 불일치(${fixtureCase.id}/${expectedNode.id}): ${node.properties[property]} != ${value}`);
            }
          }
        }
        for (const expectedSample of fixtureCase.samples) {
          const sample = actual.samples.find(({ id }) => id === expectedSample.id);
          if (!sample) throw new Error(`Chrome hit-test 표본이 없습니다(${fixtureCase.id}/${expectedSample.id}).`);
          // elementsFromPoint는 보조 관측만 저장한다. paint rank 및 pixel oracle로 승격하지 않는다.
          for (const participantId of expectedSample.topToBottom) {
            const participant = actualById.get(participantId);
            const x = sample.cssPoint.x;
            const y = sample.cssPoint.y;
            if (!participant.hasLayoutBox || x <= participant.rect.x || x >= participant.rect.x + participant.rect.width
              || y <= participant.rect.y || y >= participant.rect.y + participant.rect.height) {
              throw new Error(`sample이 paint box 내부에 있지 않습니다(${fixtureCase.id}/${expectedSample.id}/${participantId}).`);
            }
          }
        }
      }
      for (const control of controls) {
        const actual = observation.controls.find(({ id }) => id === control.id);
        if (!actual || !['x','y','width','height'].every((field) => Number.isFinite(actual.rect[field]))) {
          throw new Error(`negative control 관찰이 없거나 frame이 유한하지 않습니다: ${control.id}`);
        }
        for (const [property, value] of Object.entries(control.expectedComputed ?? {})) {
          if (actual.properties[property] !== value) {
            throw new Error(`negative control computed ${property} 불일치(${control.id}): ${actual.properties[property]} != ${value}`);
          }
        }
      }

      await page.send('Runtime.evaluate', {
        expression:'new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))',
        awaitPromise:true,
      });
      const topLayerIds = cases.flatMap((fixtureCase) => fixtureCase.activateTopLayer ?? []);
      const openTopLayer = await page.send('Runtime.evaluate', {
        expression:`(() => { for (const id of ${JSON.stringify(topLayerIds)}) document.getElementById(id).showModal(); return true; })()`,
        returnByValue:true,
      });
      if (openTopLayer.exceptionDetails || openTopLayer.result?.value !== true) {
        throw new Error(`screenshot용 top-layer 열기에 실패했습니다(DPR ${deviceScaleFactor}).`);
      }
      await page.send('Runtime.evaluate', {
        expression:'new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)))',
        awaitPromise:true,
      });
      const screenshot = await page.send('Page.captureScreenshot', {
        format:'png', fromSurface:true, captureBeyondViewport:false, optimizeForSpeed:false,
      });
      await page.send('Runtime.evaluate', {
        expression:`(() => { for (const id of ${JSON.stringify(topLayerIds)}) document.getElementById(id).close(); return true; })()`,
        returnByValue:true,
      });
      const screenshotBuffer = Buffer.from(screenshot.data, 'base64');
      const screenshotSamples = [...cases.flatMap((fixtureCase) => fixtureCase.samples.map((sample) => {
        const origin = setup.result.value.itemOrigins[fixtureCase.id];
        const xCssPx = origin.x + sample.xCssPx;
        const yCssPx = origin.y + sample.yCssPx;
        return { fixtureId:fixtureCase.id, sampleId:sample.id,
          xPixel:Math.floor((xCssPx + 0.5) * deviceScaleFactor),
          yPixel:Math.floor((yCssPx + 0.5) * deviceScaleFactor), expectedRgba:sample.rgba };
      }))];
      const pixelExpression = `(async() => {
        const image = new Image(); image.src = 'data:image/png;base64,${screenshot.data}'; await image.decode();
        const canvas = document.createElement('canvas'); canvas.width = image.naturalWidth; canvas.height = image.naturalHeight;
        const context = canvas.getContext('2d', {willReadFrequently:true}); context.drawImage(image, 0, 0);
        const points = ${JSON.stringify(screenshotSamples)};
        return {width:image.naturalWidth,height:image.naturalHeight,colorSpace:context.getImageData(0,0,1,1).colorSpace,
          samples:points.map(point => ({fixtureId:point.fixtureId,sampleId:point.sampleId,xPixel:point.xPixel,yPixel:point.yPixel,
            expectedRgba:point.expectedRgba,rgba:Array.from(context.getImageData(point.xPixel,point.yPixel,1,1).data)}))};
      })()`;
      const pixelEvaluation = await page.send('Runtime.evaluate', {
        expression:pixelExpression, awaitPromise:true, returnByValue:true,
      });
      if (pixelEvaluation.exceptionDetails || !pixelEvaluation.result?.value) {
        throw new Error(`Chrome screenshot PNG decode 실패(DPR ${deviceScaleFactor}): ${JSON.stringify(pixelEvaluation.exceptionDetails)}`);
      }
      const decoded = pixelEvaluation.result.value;
      const expectedWidth = inventory.viewport.widthCssPx * deviceScaleFactor;
      const expectedHeight = inventory.viewport.heightCssPx * deviceScaleFactor;
      if (decoded.width !== expectedWidth || decoded.height !== expectedHeight || decoded.colorSpace !== 'srgb'
        || decoded.samples.length !== screenshotSamples.length) {
        throw new Error(`Chrome screenshot 크기·색공간·표본 수 불일치(DPR ${deviceScaleFactor}): ${JSON.stringify(decoded)}`);
      }
      for (const sample of decoded.samples) {
        if (!sample.rgba.every((channel, index) => Math.abs(channel - sample.expectedRgba[index])
          <= inventory.comparison.maximumPixelChannelError)) {
          pixelMismatches.push({ deviceScaleFactor, fixtureId:sample.fixtureId, sampleId:sample.sampleId,
            actualRgba:sample.rgba, expectedRgba:sample.expectedRgba });
        }
      }
      screenshotBytes.push({ deviceScaleFactor, bytes:screenshotBuffer, sha256:sha256(screenshotBuffer),
        pixelSize:{width:decoded.width,height:decoded.height}, samples:decoded.samples });
      observations.push({ environment, cases:observation.cases, controls:observation.controls });
    }
    if (JSON.stringify(observations[0].cases) !== JSON.stringify(observations[1].cases)
      || JSON.stringify(observations[0].controls) !== JSON.stringify(observations[1].controls)
      || JSON.stringify(screenshotBytes[0].samples.map(({fixtureId,sampleId,rgba}) => [fixtureId,sampleId,rgba]))
        !== JSON.stringify(screenshotBytes[1].samples.map(({fixtureId,sampleId,rgba}) => [fixtureId,sampleId,rgba]))) {
      throw new Error('DPR 변경이 CSS frame·computed style·hit-test·PNG 표본 색을 바꿨습니다.');
    }
    if (pixelMismatches.length > 0) {
      throw new Error(`Chrome screenshot 표본 색 불일치 ${pixelMismatches.length}건: ${JSON.stringify(pixelMismatches)}`);
    }
    return { observations, screenshotBytes, browserVersion };
  },
});

const screenshotFiles = [];
await mkdir(screenshotAbsoluteDirectory, { recursive:true });
for (const screenshot of captureResult.screenshotBytes) {
  const fileName = `chrome-dpr-${screenshot.deviceScaleFactor}.png`;
  const screenshotPath = join(screenshotAbsoluteDirectory, fileName);
  await writeFile(screenshotPath, screenshot.bytes, { flag:replaceReference ? 'w' : 'wx' });
  screenshotFiles.push({
    deviceScaleFactor:screenshot.deviceScaleFactor,
    path:`${screenshotDirectory}/${fileName}`,
    sha256:await sha256File(screenshotPath),
    pixelSize:screenshot.pixelSize,
    sampleCount:screenshot.samples.length,
    samples:screenshot.samples,
  });
}
const reference = {
  schema:'spinon-css-c12-4-stacking-reference/v1',
  referenceId:'chromium-darwin-arm64-Chrome-154.0.8037.98-c12-4',
  fixture:{
    id:inventory.fixtureId,
    inventoryPath,
    inventorySha256:sha256(inventoryBytes),
    htmlPath,
    htmlSha256:sha256(htmlBytes),
  },
  captureTool:{
    path:capturePath,
    sha256:await sha256File(join(repositoryRoot,capturePath)),
    helper:helperPath,
    helperSha256:await sha256File(join(repositoryRoot,helperPath)),
  },
  wpt:inventory.wpt,
  capture:{
    nodeVersion:process.version,
    operatingSystem:{platform:platform(),arch:arch(),release:spawnSync('uname',['-r'],{encoding:'utf8'}).stdout.trim()},
    locale:inventory.environment.locale,
    timeZone:inventory.environment.timeZone,
    network:'offline',
    screenshotColorSpace:'sRGB',
    browserFlags,
  },
  chromium:{
    version:captureResult.browserVersion.product,
    cliVersion:versionResult.stdout.trim(),
    revision:captureResult.browserVersion.revision,
    userAgent:captureResult.browserVersion.userAgent,
    executable:chromiumPath,
    binarySha256:chromiumSha256,
    javascriptVersion:captureResult.browserVersion.jsVersion,
  },
  viewport:inventory.viewport,
  comparison:inventory.comparison,
  computedProperties:inventory.computedProperties,
  inventorySummary:{cases:cases.length,nodes:nodeIds.length,negativeControls:controls.length,samples:cases.reduce((sum,fixtureCase)=>sum+fixtureCase.samples.length,0)},
  hitTestComparison:{
    role:'supplementary-observation-not-paint-order-oracle',
    deviceScaleFactors:captureResult.observations.map(({environment, cases:observedCases}) => ({
      deviceScaleFactor:environment.deviceScaleFactor,
      cases:observedCases.map((observedCase) => {
        const fixtureCase=cases.find(({id})=>id===observedCase.id);
        return {caseId:observedCase.id,samples:observedCase.samples.map((sample) => {
          const expected=fixtureCase.samples.find(({id})=>id===sample.id);
          return {sampleId:sample.id,expectedPaintOrderCandidate:expected.topToBottom,
            observedHitTestTopToBottom:sample.hitTestTopToBottom,
            sameSequence:JSON.stringify(expected.topToBottom)===JSON.stringify(sample.hitTestTopToBottom)};
        })};
      }),
    })),
  },
  screenshotFiles,
  observations:captureResult.observations,
};
await writeFile(referenceAbsolutePath, `${JSON.stringify(reference,null,2)}\n`, { flag:replaceReference ? 'w' : 'wx' });
console.log(`reference=${reference.referenceId}`);
console.log(`path=${referencePath}`);
console.log(`cases=${reference.inventorySummary.cases} nodes=${reference.inventorySummary.nodes} controls=${reference.inventorySummary.negativeControls}`);
console.log(`chrome=${reference.chromium.version} revision=${reference.chromium.revision}`);
console.log(`screenshots=${screenshotFiles.map(({path})=>path).join(',')}`);
console.log(`wpt_execution=${reference.wpt.execution}`);
