const nodes = Object.create(null);

function c192Node(parent, id, declarations) {
  if (nodes[id]) throw new Error('C19.2 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c192-node', id);
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const body = document.body || document;
if (body !== document && typeof body.setAttribute === 'function') {
  body.setAttribute('style', 'margin:0;padding:0;width:360px;height:240px;overflow:hidden');
}
const stage = c192Node(
  body,
  'c192-stage',
  'display:block;position:relative;box-sizing:border-box;width:360px;height:240px;background-color:rgb(32 49 75)',
);
c192Node(stage, 'c192-alpha', 'position:absolute;left:8px;top:8px;width:36px;height:36px;background-color:rgb(255 0 0 / 50%)');
const group = c192Node(stage, 'c192-group', 'position:absolute;left:56px;top:8px;width:48px;height:40px;opacity:0.5');
c192Node(group, 'c192-group-blue', 'position:absolute;left:0;top:0;width:32px;height:40px;background-color:rgb(0 128 255)');
c192Node(group, 'c192-group-yellow', 'position:absolute;left:16px;top:0;width:32px;height:40px;background-color:rgb(255 192 0)');
const nestedOuter = c192Node(stage, 'c192-nested-outer', 'position:absolute;left:112px;top:8px;width:32px;height:32px;opacity:0.5');
c192Node(nestedOuter, 'c192-nested-inner', 'position:absolute;left:0;top:0;width:32px;height:32px;opacity:0.5;background-color:rgb(0 255 0)');
c192Node(stage, 'c192-alpha-opacity', 'position:absolute;left:152px;top:8px;width:32px;height:32px;opacity:0.5;background-color:rgb(255 0 0 / 50%)');
c192Node(stage, 'c192-wide-gamut', 'position:absolute;left:192px;top:8px;width:32px;height:32px;background-color:oklch(60% 0.15 230deg)');

for (const [id, left, opacity] of [
  ['c192-opacity-zero', 8, 0],
  ['c192-opacity-half', 56, 0.5],
  ['c192-opacity-one', 112, 1],
]) {
  const parent = c192Node(
    stage,
    id,
    `position:absolute;left:${left}px;top:56px;width:40px;height:40px;background-color:rgb(255 0 0);opacity:${opacity}`,
  );
  if (opacity > 0) {
    c192Node(parent, `${id}-blue-child`, 'position:absolute;left:0;top:0;width:20px;height:20px;background-color:rgb(0 0 255)');
  }
}

const clip = c192Node(stage, 'c192-clip-context', 'position:absolute;left:168px;top:56px;width:48px;height:40px;overflow:hidden;opacity:0.5');
c192Node(clip, 'c192-clip-red', 'position:absolute;left:30px;top:0;width:30px;height:40px;z-index:1;background-color:rgb(255 0 0)');
c192Node(clip, 'c192-clip-blue', 'position:absolute;left:15px;top:15px;width:23px;height:20px;z-index:2;background-color:rgb(0 0 255)');
c192Node(stage, 'c192-current-color-border', 'position:absolute;left:232px;top:56px;box-sizing:border-box;width:48px;height:48px;color:rgb(24 192 128 / 50%);border:4px solid currentColor');
c192Node(stage, 'c192-root-hit', 'position:absolute;left:288px;top:56px;width:40px;height:40px;background-color:rgb(255 255 255 / 10%)');

globalThis.spinonC192Nodes = nodes;
({
  schema: 'spinon-c19-2-box-paint-opacity/v1',
  nodeCount: Object.keys(nodes).length,
  samplePoints: {
    rootBackdrop: [340, 232], sourceAlpha: [26, 26], groupSingle: [64, 26],
    groupOverlap: [80, 26], nestedOpacity: [128, 24], alphaTimesGroupOpacity: [168, 24],
    wideGamutSrgb: [208, 24], opacityZero: [16, 72], opacityHalfSingle: [84, 90],
    opacityHalfOverlap: [68, 68], opacityOneOverlap: [124, 68],
    clippedZOverlap: [202, 76], clippedRed: [212, 76], clippedAway: [220, 76],
    currentColorBorder: [234, 58], borderInterior: [256, 80],
  },
})
