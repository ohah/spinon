import { writeFile } from 'node:fs/promises';
import { arch, platform, release } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const outputPath = join(
  repositoryRoot,
  'tests/fixtures/css/references/c07-2-border-width-high-dpr-v1.json',
);
const expectedVersion = {
  product: 'Chrome/154.0.8037.98',
  revision: '@b859317bf11f6be47f9b7799ec690a0a42a1fb33',
};
const deviceScaleFactors = [1, 2, 2.625, 3];
const specifiedWidths = [1.999, 1.9999, 2, 2.001];
const chromiumPath = await findChromiumExecutable();
const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    if (browserVersion.product !== expectedVersion.product
      || browserVersion.revision !== expectedVersion.revision) {
      throw new Error(`고정 Chromium 버전과 다릅니다: ${JSON.stringify(browserVersion)}`);
    }
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: 'about:blank' });
    await loaded;

    const observations = [];
    for (const deviceScaleFactor of deviceScaleFactors) {
      await page.send('Emulation.setDeviceMetricsOverride', {
        width: 800,
        height: 600,
        deviceScaleFactor,
        mobile: false,
        screenWidth: 800,
        screenHeight: 600,
      });
      const expression = `(() => {
        const values = ${JSON.stringify(specifiedWidths)};
        return {
          deviceScaleFactor: devicePixelRatio,
          widths: values.map((widthCssPx) => {
            const element = document.createElement('div');
            element.style.cssText = 'display:block;box-sizing:content-box;width:100px;height:20px;padding:0;border:solid ' + widthCssPx + 'px';
            document.body.replaceChildren(element);
            const rect = element.getBoundingClientRect();
            return {
              specifiedCssPx: widthCssPx,
              computedCssPx: getComputedStyle(element).borderLeftWidth,
              borderBoxWidthCssPx: rect.width,
            };
          }),
          relativeWidths: [
            { unit: 'em', css: 'display:block;box-sizing:content-box;width:100px;height:20px;padding:0;font-size:1px;border:solid 2em' },
            { unit: 'vw', css: 'display:block;box-sizing:content-box;width:100px;height:20px;padding:0;border:solid 0.25vw' },
            { unit: 'vh', css: 'display:block;box-sizing:content-box;width:100px;height:20px;padding:0;border:solid 0.3333333333vh' },
            { unit: 'var-vh', css: 'display:block;box-sizing:content-box;width:100px;height:20px;padding:0;--edge:0.3333333333vh;border:solid var(--edge)' },
          ].map(({ unit, css }) => {
            const element = document.createElement('div');
            element.style.cssText = css;
            document.body.replaceChildren(element);
            const rect = element.getBoundingClientRect();
            return {
              unit,
              computedCssPx: getComputedStyle(element).borderLeftWidth,
              borderBoxWidthCssPx: rect.width,
            };
          }),
        };
      })()`;
      const response = await page.send('Runtime.evaluate', {
        expression,
        returnByValue: true,
      });
      if (response.exceptionDetails) {
        throw new Error(`Chrome 경계 측정이 실패했습니다: ${JSON.stringify(response.exceptionDetails)}`);
      }
      observations.push(response.result.value);
    }
    return { browserVersion, observations };
  },
});

const capture = {
  schema: 'spinon-css-c07-2-border-width-boundary/v1',
  fixtureId: 'C07.2-border-width-high-dpr-v1',
  environment: {
    os: `${platform()} ${release()}`,
    architecture: arch(),
    node: process.version,
    viewportCssPx: { width: 800, height: 600 },
    deviceScaleFactors,
    locale: 'en-US',
    timeZone: 'UTC',
    browserFlags,
    chromiumExecutableSha256: await sha256File(chromiumPath),
  },
  captureTool: {
    path: 'tools/css-reference/capture-c07-2-border-width-high-dpr.mjs',
    sha256: await sha256File(fileURLToPath(import.meta.url)),
    dependencies: [{
      path: 'tools/css-reference/chromium-session.mjs',
      sha256: await sha256File(join(repositoryRoot, 'tools/css-reference/chromium-session.mjs')),
    }],
  },
  ...captureResult,
};

const expectedWidths = [
  ['1px', 102],
  ['1px', 102],
  ['2px', 104],
  ['2px', 104],
];
if (capture.observations.length !== deviceScaleFactors.length
  || capture.observations.some((observation, index) => (
    observation.deviceScaleFactor !== deviceScaleFactors[index]
      || observation.widths.length !== expectedWidths.length
      || observation.relativeWidths.length !== 4
      || observation.relativeWidths.some((relative) => (
        relative.computedCssPx !== '2px' || relative.borderBoxWidthCssPx !== 104
      ))
      || observation.widths.some((width, widthIndex) => (
        width.computedCssPx !== expectedWidths[widthIndex][0]
          || width.borderBoxWidthCssPx !== expectedWidths[widthIndex][1]
      ))
  ))) {
  throw new Error(`Chrome border-width 경계가 달라졌습니다: ${JSON.stringify(capture.observations)}`);
}

await writeFile(outputPath, `${JSON.stringify(capture, null, 2)}\n`);
console.log(`C07.2 Chrome DPR 경계 기준 저장: ${outputPath}`);
