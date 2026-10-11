const fixture = document.createElement('div');
const nodes = Object.create(null);

function c124Node(parent, id, declarations) {
  if (nodes[id]) throw new Error('C12.4 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c12-node', '');
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

fixture.setAttribute('id', 'c124-root');
fixture.setAttribute('data-c12-node', '');
fixture.setAttribute(
  'style',
  'display:block;position:relative;box-sizing:border-box;width:320px;height:240px;background-color:#101827',
);
nodes['c124-root'] = fixture;

const autoParent = c124Node(
  fixture,
  'c124-relative-auto',
  'position:relative;left:20px;top:20px;width:120px;height:80px;background-color:#243b5a',
);
c124Node(
  autoParent,
  'c124-auto-child-two',
  'position:absolute;left:0;top:0;width:100px;height:70px;--c124-level:2;z-index:var(--c124-level);background-color:#38bdf8',
);
c124Node(
  fixture,
  'c124-auto-sibling-one',
  'position:absolute;left:50px;top:50px;width:100px;height:70px;z-index:1;background-color:#28a745',
);

const nestedParent = c124Node(
  fixture,
  'c124-outer-context-one',
  'position:absolute;left:165px;top:20px;width:120px;height:80px;z-index:1;background-color:#f59e0b',
);
c124Node(
  nestedParent,
  'c124-inner-context-9999',
  'position:absolute;left:0;top:0;width:120px;height:80px;z-index:9999;background-color:#2866f6',
);
c124Node(
  fixture,
  'c124-outer-context-two',
  'position:absolute;left:165px;top:20px;width:120px;height:80px;z-index:2;background-color:#22aa66',
);

const flex = c124Node(
  fixture,
  'c124-static-flex-container',
  'display:flex;position:absolute;left:20px;top:125px;width:240px;height:90px;flex-direction:row;align-items:flex-start;justify-content:flex-start',
);
c124Node(
  flex,
  'c124-static-flex-z-one',
  'position:static;flex:0 0 150px;height:70px;margin-right:-100px;z-index:1;background-color:#a855f7',
);
c124Node(
  flex,
  'c124-static-flex-z-two',
  'position:static;flex:0 0 150px;height:70px;z-index:2;background-color:#e5484d',
);

(document.body || document).appendChild(fixture);
globalThis.spinonC124NodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: fixture.id });
