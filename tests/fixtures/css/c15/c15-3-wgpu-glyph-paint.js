const body = document.body || document;
if (body !== document && typeof body.setAttribute === 'function') {
  body.setAttribute('style', 'margin:0;padding:0;width:360px;height:240px;overflow:hidden');
}

const stage = document.createElement('div');
stage.setAttribute('id', 'c15-3-glyph-stage');
stage.setAttribute('style',
  'display:block;position:relative;box-sizing:border-box;width:360px;height:240px;background-color:rgb(247 249 252)',
);
body.appendChild(stage);

const clippedRemainder = document.createElement('div');
clippedRemainder.setAttribute('id', 'c15-3-clipped-remainder');
clippedRemainder.setAttribute('style',
  'position:absolute;left:160px;top:0;width:200px;height:60px;background-color:rgb(222 233 246)',
);
stage.appendChild(clippedRemainder);

({
  schema: 'spinon-c15-3-wgpu-glyph-paint/v1',
  viewport: { width: 360, height: 240, deviceScaleFactor: 1 },
  glyphClip: { left: 12, right: 160, top: 12 },
})
