(() => {
  const old = globalThis.spinonC133GestureNodes;
  if (old && old.scene && old.scene.parentNode) old.scene.parentNode.removeChild(old.scene);
  const nodes = Object.create(null);

  function make(parent, id, declarations) {
    const element = document.createElement('div');
    element.setAttribute('id', id);
    element.setAttribute('data-c133-gesture-node', '');
    element.setAttribute('style', declarations);
    parent.appendChild(element);
    nodes[id] = element;
    return element;
  }

  const policyScenario = Number(globalThis.__spinonC133GesturePolicyScenario || 0);
  const innerOverscroll = policyScenario === 2
    ? 'overscroll-behavior-x:contain;overscroll-behavior-y:auto;'
    : 'overscroll-behavior-x:auto;overscroll-behavior-y:auto;';
  const innerTouchAction = policyScenario === 3 ? 'none'
    : policyScenario === 4 ? 'pan-x'
    : policyScenario === 5 || policyScenario === 7 ? 'pan-y' : 'auto';
  const innerContentTouchAction = policyScenario === 6 || policyScenario === 7
    ? 'pan-x' : 'auto';
  const body = document.body;
  const scene = make(body || document, 'c133-scene',
    'display:block;box-sizing:border-box;width:400px;height:240px;background-color:#101827');
  const root = make(scene, 'c133-root',
    'position:absolute;left:16px;top:16px;box-sizing:border-box;width:368px;height:208px;border:3px solid #f0c674;overflow:auto;overscroll-behavior-x:auto;overscroll-behavior-y:auto;background-color:#344256');
  const rootContent = make(root, 'c133-root-content',
    'position:relative;box-sizing:border-box;width:520px;height:440px;padding:10px;background-color:#344256');
  const outer = make(rootContent, 'c133-outer',
    'position:absolute;left:30px;top:30px;box-sizing:border-box;width:220px;height:140px;border:3px solid #61c8a5;overflow:auto;overscroll-behavior-x:auto;overscroll-behavior-y:auto;background-color:#1b3441');
  const outerContent = make(outer, 'c133-outer-content',
    'position:relative;box-sizing:border-box;width:320px;height:320px;background-color:#1b3441');
  const inner = make(outerContent, 'c133-inner',
    'position:absolute;left:20px;top:20px;box-sizing:border-box;width:120px;height:80px;border:3px solid #63a9ed;overflow:auto;'
      + innerOverscroll + 'touch-action:' + innerTouchAction + ';background-color:#17253a');
  const innerContent = make(inner, 'c133-inner-content',
    'position:relative;box-sizing:border-box;width:200px;height:180px;touch-action:'
      + innerContentTouchAction + ';background-color:#17253a');
  make(innerContent, 'c133-inner-horizontal-marker',
    'position:absolute;left:140px;top:25px;box-sizing:border-box;width:24px;height:24px;background-color:#e3544e');
  make(innerContent, 'c133-inner-marker',
    'position:absolute;left:70px;top:105px;box-sizing:border-box;width:24px;height:24px;background-color:#e3544e');
  make(outerContent, 'c133-outer-marker',
    'position:absolute;left:270px;top:270px;box-sizing:border-box;width:24px;height:24px;background-color:#ed8d3f');
  make(rootContent, 'c133-root-marker',
    'position:absolute;left:470px;top:390px;box-sizing:border-box;width:24px;height:24px;background-color:#b184e3');
  globalThis.spinonC133GestureNodes = nodes;
  return { nodeCount: Object.keys(nodes).length, viewportCssPx: [400, 240] };
})();
