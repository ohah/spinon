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

for (const mode of ['visible', 'hidden']) {
  const parent = document.createElement('div');
  parent.setAttribute('id', `c15-3-overflow-${mode}`);
  parent.setAttribute('style',
    `position:absolute;left:12px;top:${mode === 'visible' ? 80 : 120}px;width:15px;height:10px;` +
    `border:1px solid rgb(136 136 136);background-color:rgb(222 233 246);overflow:${mode}`,
  );
  stage.appendChild(parent);

  const textOwner = document.createElement('div');
  textOwner.setAttribute('id', `c15-3-overflow-${mode}-text-owner`);
  textOwner.setAttribute('style',
    'position:absolute;left:0;top:0;width:40px;height:10px;background-color:transparent',
  );
  parent.appendChild(textOwner);
}

const horizontalScroll = document.createElement('div');
horizontalScroll.setAttribute('id', 'c15-3-text-scroll-horizontal');
horizontalScroll.setAttribute('style',
  'position:absolute;left:40px;top:160px;box-sizing:border-box;width:15px;height:22px;' +
  'border:1px solid rgb(136 136 136);background-color:rgb(222 233 246);overflow:auto',
);
stage.appendChild(horizontalScroll);
const horizontalText = document.createElement('div');
horizontalText.setAttribute('id', 'c15-3-text-scroll-horizontal-owner');
horizontalText.setAttribute('style',
  'position:absolute;left:0;top:0;width:40px;height:20px;background-color:transparent',
);
horizontalScroll.appendChild(horizontalText);

const verticalScroll = document.createElement('div');
verticalScroll.setAttribute('id', 'c15-3-text-scroll-vertical');
verticalScroll.setAttribute('style',
  'position:absolute;left:100px;top:160px;box-sizing:border-box;width:30px;height:15px;' +
  'border:1px solid rgb(136 136 136);background-color:rgb(222 233 246);overflow:auto',
);
stage.appendChild(verticalScroll);
const verticalText = document.createElement('div');
verticalText.setAttribute('id', 'c15-3-text-scroll-vertical-owner');
verticalText.setAttribute('style',
  'position:absolute;left:0;top:22px;width:28px;height:10px;background-color:transparent',
);
verticalScroll.appendChild(verticalText);

const nestedScroll = document.createElement('div');
nestedScroll.setAttribute('id', 'c15-3-text-scroll-nested-outer');
nestedScroll.setAttribute('style',
  'position:absolute;left:260px;top:160px;box-sizing:border-box;width:30px;height:22px;' +
  'border:1px solid rgb(136 136 136);background-color:rgb(222 233 246);overflow:auto',
);
stage.appendChild(nestedScroll);
const nestedExtent = document.createElement('div');
nestedExtent.setAttribute('id', 'c15-3-text-scroll-nested-extent');
nestedExtent.setAttribute('style',
  'position:absolute;left:0;top:0;width:50px;height:20px;background-color:transparent',
);
// Keep the nested scroll container directly under the outer owner so scroll parent identity
// and the text's ancestor offset metadata describe the same two-container chain.
nestedScroll.appendChild(nestedExtent);
const nestedInnerScroll = document.createElement('div');
nestedInnerScroll.setAttribute('id', 'c15-3-text-scroll-nested-inner');
nestedInnerScroll.setAttribute('style',
  'position:absolute;left:0;top:0;box-sizing:border-box;width:20px;height:22px;' +
  'border:1px solid rgb(136 136 136);background-color:rgb(222 233 246);overflow:auto',
);
nestedScroll.appendChild(nestedInnerScroll);
const nestedText = document.createElement('div');
nestedText.setAttribute('id', 'c15-3-text-scroll-nested-owner');
nestedText.setAttribute('style',
  'position:absolute;left:0;top:0;width:40px;height:20px;background-color:transparent',
);
nestedInnerScroll.appendChild(nestedText);

({
  schema: 'spinon-c15-3-wgpu-glyph-paint/v2',
  viewport: { width: 360, height: 240, deviceScaleFactor: 1 },
  glyphClip: { left: 12, right: 160, top: 12 },
  overflowParents: [
    { id: 'c15-3-overflow-visible', overflow: 'visible', text: 'XX' },
    { id: 'c15-3-overflow-hidden', overflow: 'hidden', text: 'XX' },
  ],
  scrollTextOwners: [
    { id: 'c15-3-text-scroll-horizontal-owner', text: 'XX', scrollLeft: 3 },
    { id: 'c15-3-text-scroll-vertical-owner', text: 'XX', scrollTop: 18 },
    {
      id: 'c15-3-text-scroll-nested-owner', text: 'XX',
      outerScrollLeft: 4, innerScrollLeft: 5,
    },
  ],
})
