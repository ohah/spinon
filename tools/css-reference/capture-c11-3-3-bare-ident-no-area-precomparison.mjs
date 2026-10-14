import assert from 'node:assert/strict';
import { createHash, randomUUID } from 'node:crypto';
import { access, mkdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import { findChromiumExecutable, runChromiumPage, sha256File } from './chromium-session.mjs';

const repositoryRoot = resolve(fileURLToPath(new URL('../..', import.meta.url)));
const evidencePath = resolve(repositoryRoot,
  'spec/internal/evidence/c11-3-3-post-merge-followup-2026-10-14/chrome-154-bare-ident-no-area.json');
const html = `<!doctype html><meta charset="utf-8"><style>*{box-sizing:border-box}body{margin:0}.grid{display:grid;width:120px;height:90px;grid-template-columns:40px [panel] 40px [panel] 40px;grid-template-rows:30px [panel] 30px [panel] 30px;gap:0}.item{width:20px;height:20px;grid-area:panel}</style><div class="grid"><div id="panel-item" class="item"></div></div>`;
const htmlSha256 = createHash('sha256').update(html).digest('hex');
const captureToolPath = 'tools/css-reference/capture-c11-3-3-bare-ident-no-area-precomparison.mjs';
const captureToolSha256 = await sha256File(resolve(repositoryRoot, captureToolPath));
await access(evidencePath).then(
  () => { throw new Error(`기존 Chrome 기준 증거를 덮어쓰지 않습니다: ${evidencePath}`); },
  (error) => { if (error.code !== 'ENOENT') throw error; },
);

const chromiumPath = await findChromiumExecutable();
const { captureResult, browserFlags } = await runChromiumPage({
  chromiumPath,
  onPage: async ({ page, browserVersion }) => {
    assert.equal(browserVersion.product, 'Chrome/154.0.8037.92');
    assert.equal(browserVersion.revision, '@334b65d254ccc35df4fca82706d1753227b01039');
    await page.send('Page.enable');
    await page.send('Runtime.enable');
    await page.send('Emulation.setDeviceMetricsOverride', {
      width: 320, height: 240, deviceScaleFactor: 1, mobile: false,
      screenWidth: 320, screenHeight: 240,
    });
    const loaded = page.waitForEvent('Page.loadEventFired');
    await page.send('Page.navigate', { url: `data:text/html;charset=utf-8,${encodeURIComponent(html)}` });
    await loaded;
    const evaluated = await page.send('Runtime.evaluate', {
      expression: `(() => {
        const grid = document.querySelector('.grid');
        const item = document.querySelector('.item');
        const gridRect = grid.getBoundingClientRect();
        const itemRect = item.getBoundingClientRect();
        const gridStyle = getComputedStyle(grid);
        return {
          computed: {
            gridTemplateAreas: gridStyle.gridTemplateAreas,
            gridTemplateColumns: gridStyle.gridTemplateColumns,
            gridTemplateRows: gridStyle.gridTemplateRows,
            gridArea: getComputedStyle(item).gridArea,
          },
          grid: { x: gridRect.x, y: gridRect.y, width: gridRect.width, height: gridRect.height },
          item: {
            x: itemRect.x - gridRect.x, y: itemRect.y - gridRect.y,
            width: itemRect.width, height: itemRect.height,
          },
        };
      })()`,
      returnByValue: true,
    });
    return { browserVersion, result: evaluated.result.value };
  },
});
assert.equal(captureResult.result.computed.gridTemplateAreas, 'none');
assert.deepEqual(captureResult.result.item, { x: 40, y: 30, width: 20, height: 20 });

const record = {
  schema: 'spinon-css-c11-3-3-bare-ident-no-area-precomparison/v1',
  runId: `c1133-bare-ident-chrome154-${new Date().toISOString().replaceAll(/[^0-9TZ]/g, '')}-${randomUUID().slice(0, 8)}`,
  captureTool: { path: captureToolPath, sha256: captureToolSha256 },
  fixture: { html, htmlSha256 },
  oracle: {
    product: captureResult.browserVersion.product,
    revision: captureResult.browserVersion.revision,
    userAgent: captureResult.browserVersion.userAgent,
    executableSha256: await sha256File(chromiumPath),
  },
  browserFlags,
  viewport: { width: 320, height: 240, deviceScaleFactor: 1 },
  observation: captureResult.result,
  limits: ['single pinned Chrome case', 'not a WPT conformance result'],
};
await mkdir(dirname(evidencePath), { recursive: true });
await writeFile(evidencePath, `${JSON.stringify(record, null, 2)}\n`, { flag: 'wx' });
console.log(JSON.stringify({ evidencePath, record }, null, 2));
