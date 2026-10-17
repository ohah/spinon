const nodes = Object.create(null);

function node(parent, id, style) {
  if (nodes[id]) throw new Error('중복된 C13.1 stacking clip node: ' + id);
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c131-stacking-clip-node', '');
  element.setAttribute('style', style);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const root = node(
  document.body || document,
  'root',
  'display:block;position:relative;box-sizing:border-box;width:540px;height:128px;background-color:#f0f4f7',
);

for (const [caseId, left, overflow] of [
  ['visible', 20, 'overflow:visible;'],
  ['clip', 190, 'overflow:clip;'],
  ['hidden', 360, 'overflow:hidden;'],
]) {
  const parent = node(
    root,
    `${caseId}-parent`,
    `position:absolute;left:${left}px;top:28px;box-sizing:border-box;width:120px;height:72px;z-index:0;${overflow}background-color:#f0c674`,
  );
  const layer = node(
    parent,
    `${caseId}-layer`,
    'position:absolute;left:56px;top:12px;box-sizing:border-box;width:80px;height:48px;z-index:900',
  );
  node(
    layer,
    `${caseId}-target`,
    'position:absolute;left:32px;top:0;box-sizing:border-box;width:72px;height:48px;z-index:10;background-color:#57a6d9',
  );
  node(
    root,
    `${caseId}-overlay`,
    `position:absolute;left:${left + 128}px;top:40px;box-sizing:border-box;width:16px;height:48px;z-index:1;background-color:#e4514f`,
  );
}

globalThis.spinonC131StackingDescendantClipNodes = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
