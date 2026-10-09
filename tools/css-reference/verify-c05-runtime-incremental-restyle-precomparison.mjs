import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, resolve } from 'node:path';

import { findChromiumExecutable, runChromiumPage } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const html = await readFile(join(repositoryRoot, 'tests/fixtures/css/c05/runtime-incremental-restyle.html'), 'utf8');
const script = await readFile(join(repositoryRoot, 'tests/fixtures/css/c05/runtime-incremental-restyle.js'), 'utf8');
const chromiumPath = await findChromiumExecutable();
const { captureResult } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    assert.equal(browserVersion.product, 'Chrome/154.0.8037.98');
    assert.equal(browserVersion.revision, '@b859317bf11f6be47f9b7799ec690a0a42a1fb33');
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: 320,
      height: 180,
      deviceScaleFactor: 1,
      mobile: false,
    });
    await page.send('Emulation.setLocaleOverride', { locale: 'en-US' });
    await page.send('Emulation.setTimezoneOverride', { timezoneId: 'UTC' });
    const pageLoaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', {
      url: `data:text/html,${encodeURIComponent(html)}`,
    });
    await pageLoaded;

    const evaluate = async (expression) => {
      const response = await page.send('Runtime.evaluate', {
        expression,
        returnByValue: true,
      });
      if (response.exceptionDetails) {
        throw new Error(`Chromium 비교를 실행하지 못했습니다: ${response.exceptionDetails.text}`);
      }
      return response.result.value;
    };
    const readScene = async () => JSON.parse(await evaluate(`(() => {
      const ids = ['c054-app', 'c054-left', 'c054-left-a', 'c054-left-b', 'c054-right', 'c054-right-a', 'c054-right-b'];
      return JSON.stringify(Object.fromEntries(ids.map((id) => {
        const node = document.getElementById(id);
        const style = getComputedStyle(node);
        const rect = node.getBoundingClientRect();
        return [id, {
          width: style.width,
          height: style.height,
          rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
        }];
      })));
    })()`));

    await evaluate(script);
    const before = await readScene();
    await evaluate(script);
    const after = await readScene();
    return { before, after };
  },
});

assert.equal(captureResult.before['c054-left-a'].width, '32px');
assert.equal(captureResult.before['c054-left-b'].width, '32px');
assert.equal(captureResult.before['c054-right-a'].width, '41px');
assert.equal(captureResult.before['c054-right-b'].width, '41px');
assert.equal(captureResult.after['c054-left-a'].width, '46px');
assert.equal(captureResult.after['c054-left-b'].width, '46px');
assert.equal(captureResult.after['c054-right-a'].width, '41px');
assert.equal(captureResult.after['c054-right-b'].width, '41px');
assert.equal(captureResult.after['c054-left-a'].rect.x, captureResult.before['c054-left-a'].rect.x);
assert.equal(captureResult.after['c054-right-a'].rect.x, captureResult.before['c054-right-a'].rect.x);
process.stdout.write(`${JSON.stringify(captureResult, null, 2)}\n`);
