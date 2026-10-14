const nodes = Object.create(null);

function c131Node(parent, id, declarations) {
  if (nodes[id]) throw new Error('C13.1 runtime node ID가 중복되었습니다: ' + id);
  const element = document.createElement('div');
  element.setAttribute('id', id);
  element.setAttribute('data-c131-node', '');
  element.setAttribute('style', declarations);
  parent.appendChild(element);
  nodes[id] = element;
  return element;
}

const root = c131Node(
  document.body || document,
  'c131-root',
  'display:flow-root;position:relative;box-sizing:border-box;width:300px;height:100px;background-color:#101827',
);
const visible = c131Node(
  root,
  'c131-visible',
  'display:block;margin-left:10px;margin-top:6px;box-sizing:border-box;width:90px;height:44px;padding:6px;border:3px solid #f0c674;background-color:#344256',
);
c131Node(
  visible,
  'c131-visible-target',
  'width:130px;height:70px;background-color:#57a6d9',
);

const clipped = c131Node(
  root,
  'c131-clip',
  'display:block;margin-left:105px;margin-top:-44px;box-sizing:border-box;width:90px;height:44px;padding:6px;border:3px solid #f0c674;overflow:clip;background-color:#344256',
);
c131Node(
  clipped,
  'c131-clip-target',
  'width:130px;height:70px;background-color:#57a6d9',
);

const margin = c131Node(
  root,
  'c131-margin',
  'display:block;margin-left:200px;margin-top:-44px;box-sizing:border-box;width:90px;height:44px;padding:6px;border:3px solid #f0c674;overflow:clip;overflow-clip-margin:padding-box 8px;background-color:#344256',
);
c131Node(
  margin,
  'c131-margin-target',
  'width:130px;height:70px;background-color:#57a6d9',
);

const nested = c131Node(
  root,
  'c131-nested-outer',
  'display:block;margin-left:10px;margin-top:12px;box-sizing:border-box;width:120px;height:34px;padding:4px;border:2px solid #f0c674;overflow:clip;background-color:#344256',
);
const inner = c131Node(
  nested,
  'c131-nested-inner',
  'display:block;margin-left:21px;box-sizing:border-box;width:50px;height:20px;padding:2px;border:1px solid #8a45ad;overflow-x:clip;overflow-y:visible;background-color:#f4ecf7',
);
c131Node(
  inner,
  'c131-nested-target',
  'width:90px;height:28px;background-color:#e3544e',
);

const escape = c131Node(
  root,
  'c131-escape-outer',
  'display:block;margin-left:150px;margin-top:-34px;box-sizing:border-box;width:140px;height:34px;padding:4px;border:2px solid #f0c674;overflow:clip;background-color:#344256',
);
const staticClip = c131Node(
  escape,
  'c131-escape-static-clip',
  'position:static;box-sizing:border-box;width:45px;height:24px;padding:2px;border:1px solid #8a45ad;overflow-x:clip;overflow-y:visible;background-color:#f4ecf7',
);
c131Node(
  staticClip,
  'c131-escape-target',
  'position:absolute;left:222px;top:68px;width:50px;height:22px;background-color:#e3544e',
);

globalThis.spinonC131NodeRefs = nodes;
({ nodeCount: Object.keys(nodes).length, rootId: root.id });
