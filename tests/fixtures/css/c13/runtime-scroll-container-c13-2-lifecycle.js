{
  const state = globalThis.spinonC132ScrollLifecycle || { phase: 0 };
  const nodes = globalThis.spinonC132ScrollNodeRefs;
  if (!nodes) throw new Error('C13.2 base fixture가 없습니다');

  if (state.phase === 0) {
    nodes['c132-one-axis-x-fill'].setAttribute(
      'style',
      'width:260px;height:36px;background-color:#57a6d9',
    );
  } else if (state.phase === 1) {
    nodes['c132-one-axis-x-fill'].setAttribute(
      'style',
      'width:60px;height:36px;background-color:#57a6d9',
    );
  } else if (state.phase === 2) {
    const root = nodes['c132-root'];
    const target = document.createElement('div');
    target.setAttribute('id', 'c132-reparent-target');
    target.setAttribute(
      'style',
      'position:absolute;left:1800px;top:0;width:160px;height:120px;background-color:#101827',
    );
    nodes['c132-reparent-target'] = target;
    root.appendChild(target);
    root.removeChild(nodes['c132-hidden']);
  } else if (state.phase === 3) {
    nodes['c132-reparent-target'].appendChild(nodes['c132-hidden']);
  } else {
    throw new Error('C13.2 lifecycle phase가 범위를 벗어났습니다: ' + state.phase);
  }

  state.phase += 1;
  globalThis.spinonC132ScrollLifecycle = state;
  ({ phase: state.phase });
}
