const nodes = Object.create(null);

function c131BorderNode(parent, id, declarations) {
  if (nodes[id]) throw new Error('C13.1 border node ID가 중복되었습니다: ' + id);
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c131-border-node', '');
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const root = c131BorderNode(
  document.body || document,
  'root',
  'display:flow-root;position:relative;box-sizing:border-box;width:380px;height:120px;background-color:#101827',
);
const staticClip = c131BorderNode(
  root,
  'static-clip',
  'position:static;margin-left:10px;margin-top:10px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid #f0c674;overflow-x:clip;overflow-y:visible;background-color:#344256',
);
c131BorderNode(
  staticClip,
  'static-target',
  'width:140px;height:120px;background-color:#57a6d9',
);

const positionedClip = c131BorderNode(
  root,
  'positioned-clip',
  'position:absolute;left:200px;top:10px;box-sizing:border-box;width:120px;height:96px;padding:10px;border:4px solid #f0c674;overflow-x:clip;overflow-y:visible;background-color:#344256',
);
c131BorderNode(
  positionedClip,
  'positioned-target',
  'width:140px;height:120px;background-color:#57a6d9',
);

globalThis.spinonC131BorderNodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
