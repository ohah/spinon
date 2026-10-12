const fixture = document.createElement('div');
fixture.setAttribute('id', 'c11-fixed-grid');
fixture.setAttribute('data-c11-node', '');
fixture.setAttribute(
  'style',
  'display:grid;width:320px;height:130px;grid-template-columns:80px 100px 120px;grid-template-rows:50px 70px;column-gap:8px;row-gap:6px;background-color:#edf1f7',
);

const nodes = [
  ['c11-fixed-a', 'width:40px;height:20px;background-color:#2866f6'],
  ['c11-fixed-b', 'width:50px;height:24px;background-color:#e5484d'],
  ['c11-fixed-c', 'width:60px;height:26px;background-color:#19a974'],
  ['c11-fixed-d', 'width:30px;height:18px;background-color:#ffc857'],
];

for (const [id, style] of nodes) {
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c11-node', '');
  element.setAttribute('style', style);
  fixture.appendChild(element);
}

(document.body || document).appendChild(fixture);
({ sceneId: 'fixed-tracks-gap', nodeCount: nodes.length + 1, rootId: fixture.id });
