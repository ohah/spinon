const cases = [
  { id: 'visible', overflow: 'overflow:visible;' },
  { id: 'clip-x', overflow: 'overflow-x:clip;overflow-y:visible;' },
  { id: 'clip-y', overflow: 'overflow-x:visible;overflow-y:clip;' },
  { id: 'clip-both', overflow: 'overflow:clip;' },
  { id: 'clip-margin-padding', overflow: 'overflow:clip;overflow-clip-margin:padding-box 4px;' },
  { id: 'clip-margin-content', overflow: 'overflow:clip;overflow-clip-margin:content-box 4px;' },
  { id: 'clip-margin-border', overflow: 'overflow:clip;overflow-clip-margin:border-box 4px;' },
  { id: 'clip-margin-hidden', overflow: 'overflow:hidden;overflow-clip-margin:border-box 4px;' },
  { id: 'mixed-visible-hidden', overflow: 'overflow-x:visible;overflow-y:hidden;' },
  { id: 'mixed-clip-auto', overflow: 'overflow-x:clip;overflow-y:auto;overflow-clip-margin:border-box 4px;' },
  { id: 'positioned-contained', overflow: 'overflow:clip;', positioned: 'contained' },
  { id: 'positioned-escape', overflow: 'overflow:clip;', positioned: 'escape' },
  { id: 'nested-clip', overflow: 'overflow:visible;', nested: true },
];

const nodes = Object.create(null);
function node(parent, id, style) {
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c131-surface-node', '');
  element.setAttribute('style', style);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const root = node(
  document.body,
  'c131-surface-root',
  'display:block;position:relative;box-sizing:border-box;width:300px;height:100px;background-color:#101827',
);

for (const [index, spec] of cases.entries()) {
  const column = index % 3;
  const row = Math.floor(index / 3);
  const section = node(
    root,
    `c131-surface-case-${spec.id}`,
    `position:absolute;left:${column * 100}px;top:${row * 20}px;box-sizing:border-box;width:96px;height:18px;background-color:#101827`,
  );
  let boxStyle = `position:absolute;left:0;top:1px;box-sizing:border-box;width:64px;height:14px;padding:2px;border:1px solid #f0c674;${spec.overflow}background-color:#344256`;
  if (spec.positioned === 'contained') boxStyle = boxStyle.replace('position:absolute;', 'position:relative;');
  if (spec.positioned === 'escape') boxStyle = boxStyle.replace('position:absolute;', 'position:static;');
  const box = node(section, `c131-surface-box-${spec.id}`, boxStyle);

  if (spec.nested) {
    const inner = node(
      box,
      'c131-surface-inner-nested-clip',
      'position:absolute;left:20px;top:3px;box-sizing:border-box;width:34px;height:10px;padding:1px;border:1px solid #8a45ad;overflow-x:clip;overflow-y:visible;background-color:#f4ecf7',
    );
    node(inner, `c131-surface-target-${spec.id}`, 'width:56px;height:12px;background-color:#57a6d9');
  } else if (spec.positioned === 'contained') {
    node(box, `c131-surface-target-${spec.id}`, 'position:absolute;left:58px;top:7px;box-sizing:border-box;width:14px;height:6px;background-color:#e3544e');
  } else if (spec.positioned === 'escape') {
    node(section, `c131-surface-target-${spec.id}`, 'position:absolute;left:58px;top:8px;box-sizing:border-box;width:30px;height:12px;background-color:#e3544e');
  } else {
    node(box, `c131-surface-target-${spec.id}`, 'width:88px;height:16px;background-color:#57a6d9');
  }
}

globalThis.spinonC131CompactSurfaceMatrixNodes = nodes;
({ caseCount: cases.length, nodeCount: Object.keys(nodes).length, rootId: root.id });
