const specs = globalThis.__spinonC123FixtureNodes;
if (!Array.isArray(specs) || specs.length === 0 || specs[0].parentId !== null) {
  throw new Error('C12.3 runtime inventory root가 없습니다');
}

const fixture = document.createElement('div');
const nodes = Object.create(null);
const runtimePaintStyle = (spec) => {
  const color = spec.expectedPosition === 'fixed'
    ? '#38bdf8'
    : spec.id === 'c12-root' ? '#101827' : '#243b5a';
  return `${spec.style};background-color:${color}`;
};

for (const spec of specs) {
  const element = spec.parentId === null ? fixture : document.createElement('div');
  if (nodes[spec.id]) throw new Error('C12.3 runtime node ID가 중복되었습니다: ' + spec.id);
  element.setAttribute('id', spec.id);
  element.setAttribute('data-c12-node', '');
  element.setAttribute('data-c12-case', spec.caseId);
  element.setAttribute('style', runtimePaintStyle(spec));
  nodes[spec.id] = element;
  if (spec.parentId !== null) {
    const parent = nodes[spec.parentId];
    if (!parent) throw new Error('C12.3 runtime 부모가 먼저 생성되지 않았습니다: ' + spec.parentId);
    parent.appendChild(element);
  }
}

const mountPoint = document.body || document;
mountPoint.appendChild(fixture);
globalThis.spinonC123NodeRefs = nodes;
({ nodeCount: specs.length, rootId: specs[0].id });
