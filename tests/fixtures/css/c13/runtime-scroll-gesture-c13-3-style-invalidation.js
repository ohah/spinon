(() => {
  const mode = Number(globalThis.__spinonC133StyleMode);
  const nodes = globalThis.spinonC133GestureNodes;
  const inner = nodes && nodes['c133-inner'];
  const target = mode === 10 ? nodes && nodes['c133-inner-content'] : inner;
  if (!target || typeof target.setAttribute !== 'function') {
    throw new Error('C13.3 style invalidation target is missing');
  }

  const styles = {
    8: 'position:absolute;left:20px;top:20px;box-sizing:border-box;width:120px;height:80px;border:3px solid #63a9ed;overflow:auto;overscroll-behavior-x:contain;overscroll-behavior-y:auto;touch-action:none;background-color:#17253a',
    9: 'position:absolute;left:20px;top:20px;box-sizing:border-box;width:140px;height:80px;border:3px solid #63a9ed;overflow:auto;overscroll-behavior-x:auto;overscroll-behavior-y:auto;touch-action:auto;background-color:#17253a',
    10: 'position:absolute;left:20px;top:20px;box-sizing:border-box;width:120px;height:80px;border:3px solid #63a9ed;overflow:auto;overscroll-behavior-x:auto;overscroll-behavior-y:auto;touch-action:auto;background-color:#0e5650',
  };
  const nextStyle = mode === 10
    ? 'position:relative;box-sizing:border-box;width:200px;height:180px;touch-action:auto;background-color:#0e5650'
    : styles[mode];
  if (!nextStyle) throw new Error(`Unsupported C13.3 style invalidation mode ${mode}`);
  const previousStyle = target.getAttribute('style');
  target.setAttribute('style', nextStyle);
  return { mode, target: mode === 10 ? 'c133-inner-content' : 'c133-inner', previousStyle, nextStyle };
})();
